use std::io::ErrorKind;
use std::io::{Read, Write};
use std::net;
use std::sync::mpsc;
use std::thread;
use std::time;

use crate::network::buffer;

extern crate socks;

/* factory.rs - Networking factory for handling all low-level socket related stuff */

/// Holds the network factory's connection
enum Connection {
    Normal(net::TcpStream),
    Socks(socks::Socks5Stream),
}

impl Connection {
    /// Destructures the enum to the underlying socket
    fn pull_stream(&self) -> &net::TcpStream {
        match self {
            Self::Normal(v) => v,
            Self::Socks(v) => v.get_ref(),
        }
    }
}

/// The container struct for all network operations
/// The client will communicate with the factory using channels
pub struct NetworkFactory {
    pub tx: mpsc::Sender<buffer::Buffer>, //s2c channel for network factory sending packets to the client
    pub rx: mpsc::Receiver<buffer::Buffer>, //c2s channel for client sending packets to the networkfactory
    estream: Connection,                    //contains the underlying tcp connection
    pub timeout_ms: u64, //delay between reads, can be used to "fake lag". Use 1 thru 5 for optimal speed. Set to 50 for artificial lag
    stream_killed: bool, //if the tcp socket is closed
}

impl NetworkFactory {
    /// The "main()" for NetworkFactory
    pub fn start(mut self) {
        //block for the hello packet
        self.send_packet(&self.rx.recv().expect("Didn't get hello packet??"));
        let mut packets: Vec<buffer::Buffer> = Vec::with_capacity(5);
        'z: loop {
            if self.stream_killed == true {
                return;
            }
            //read if the server has anything for us
            match self.recv_packet_from_server() {
                Some(p) => { //forward the packet to the client
                    match self.send_packet_to_client(p) {
                        Some(()) => (),
                        None => {
                            self.kill_stream();
                            return;
                        }
                    }
                },
                None => {
                    self.kill_stream();
                    return;
                },
            }
            'y: loop {
                match self.recv_packet_from_client() {
                    Some(p) => packets.push(p),
                    None => break 'y,
                }
            }
            for packet in packets.iter() {
                //send out all pending packets
                self.send_packet(packet);
            }
            packets.clear(); //clear vector for next iteration
        }
    }
    /// Writes the packet to the server
    pub fn send_packet(&mut self, p: &buffer::Buffer) {
        if self.stream_killed == true {
            return;
        }
        match self.stream().write(&p.data) {
            Ok(_) => (),
            Err(e) => match e.kind() {
                ErrorKind::ConnectionReset => (),
                ErrorKind::BrokenPipe => (),
                _ => {
                    eprintln!("Got unknown error writing packet: {}", e);
                }
            },
        }
    }
    /// Reads a packet from the server
    pub fn recv_packet_from_server(&mut self) -> Option<buffer::Buffer> {
        match self.get_packet_header() {
            Some(mut p) => {
                //check kill byte or blank packet
                if p.data[0] == 255 || p.data[3] == 0 && p.data[4] == 0 {
                    return None;
                }
                let mut bytes_read = 5; //we already have the header, read the data
                let goal_bytes = p.data.len();
                'z: loop {
                    if self.stream_killed == true {
                        return None;
                    }
                    match self.stream().read(&mut p.data[bytes_read..goal_bytes]) {
                        Ok(read) => {
                            bytes_read += read;
                            if bytes_read == goal_bytes {
                                break 'z; //we have the full packet
                            }
                        }
                        Err(e) => {
                            match e.kind() {
                                ErrorKind::ConnectionReset => (),
                                _ => {
                                    eprintln!(
                                        "NetFactory::recv_packet: Error reading packet: {}",
                                        e
                                    );
                                }
                            };
                            break 'z;
                        }
                    };
                }
                return Some(p)
            }
            None => return None,
        }
    }
    /// Grabs the first 5 bytes of the packet and resizes the buffer (if not to size)
    pub fn get_packet_header(&mut self) -> Option<buffer::Buffer> {
        let mut p = buffer::new_with_header();
        let mut bytes_read = 0;
        'z: loop {
            if self.stream_killed == true {
                return None;
            }
            match self.stream().read(&mut p.data[bytes_read..]) {
                Ok(read) => {
                    bytes_read += read;
                    if p.data[0] == 255 {
                        // println!("Got kill byte!");
                        return None;
                    }
                    if bytes_read == 5 {
                        break 'z;
                    }
                }
                Err(e) => {
                    match e.kind() {
                        ErrorKind::ConnectionReset => (),
                        _ => {
                            eprintln!("NetFactory::recv_packet: Error reading packet: {}", e);
                        }
                    };
                    return None; //break 'z
                }
            };
        }
        Some(p.resize())
    }
    /// Sends provided packet to client
    pub fn send_packet_to_client(&mut self, p: buffer::Buffer) -> Option<()> {
        match self.tx.send(p) {
            Ok(_) => Some(()),
            Err(_e) => { //channel closed
                // eprintln!("Couldn't send packet to client: {}", e);
                return None;
            },
        }
    }
    /// Receives a packet from the client thread
    pub fn recv_packet_from_client(&mut self) -> Option<buffer::Buffer> {
        if self.stream_killed == true {
            return None;
        }
        match self
            .rx
            .recv_timeout(time::Duration::from_millis(self.timeout_ms))
        {
            Ok(packet) => Some(packet),
            Err(_) => None,
        }
    }
    /// Sends all packets in the vec
    pub fn send_all(&mut self) {

    }
    pub fn is_read_available(&mut self) -> bool {
        let mut buf = [0u8; 5];
        match self.stream().peek(&mut buf) {
            Ok(v) => match v {
                5 => return true,
                _ => return false,
            },
            Err(_) => {
                return false;
            }
        };
    }
    pub fn adjust_timeout(&mut self, new: u64) {
        self.timeout_ms = new;
    }
    /// Consume the netfactory, shutting down the socket and dropping the channels
    pub fn kill_stream(mut self) {
        match self.stream().shutdown(net::Shutdown::Both) {
            _ => self.stream_killed = true,
        }
        drop(self.rx);
        drop(self.tx);
    }
}

pub fn begin_networking(
    server: String,
    proxy: String,
    timeout: u64,
    stack: usize,
) -> (
    mpsc::Sender<buffer::Buffer>,
    mpsc::Receiver<buffer::Buffer>,
    thread::JoinHandle<()>,
) {
    let (c2s_tx, c2s_rx) = mpsc::channel::<buffer::Buffer>(); //client to server transceivers
    let (s2c_tx, s2c_rx) = mpsc::channel::<buffer::Buffer>(); //server to client transceivers
    let net_factory = NetworkFactory::new(server, proxy, timeout, s2c_tx, c2s_rx);
    let built_thread = thread::Builder::new()
        .name(String::from("NetFactory"))
        .stack_size(stack*1024);
    let handle = built_thread.spawn(move || net_factory.start()).unwrap();
    (c2s_tx, s2c_rx, handle)
}

impl NetworkFactory {
    pub fn new(
        server: String,
        proxy: String,
        recv_timeout: u64,
        transmitter: mpsc::Sender<buffer::Buffer>,
        receiver: mpsc::Receiver<buffer::Buffer>,
    ) -> NetworkFactory {
        if proxy.is_empty() {
            let stream = net::TcpStream::connect(server).expect("Couldn't connect to server");
            stream
                .set_nodelay(true)
                .expect("Couldn't set nodelay on socket");
            // println!("Connected to server!");
            NetworkFactory {
                rx: receiver,
                tx: transmitter,
                estream: Connection::Normal(stream),
                timeout_ms: recv_timeout,
                stream_killed: false,
            }
        } else {
            let target = socks::TargetAddr::Ip(server.parse::<net::SocketAddr>().unwrap());
            let mut stream = socks::Socks5Stream::connect(proxy.clone(), target)
                .expect("Couldn't connect to server");
            stream
                .get_mut()
                .set_nodelay(true)
                .expect("Couldn't set nodelay on socks");
            // println!("Connected to server!");
            NetworkFactory {
                rx: receiver,
                tx: transmitter,
                estream: Connection::Socks(stream),
                timeout_ms: recv_timeout,
                stream_killed: false,
            }
        }
    }
    fn stream(&mut self) -> &net::TcpStream {
        self.estream.pull_stream()
    }
}
