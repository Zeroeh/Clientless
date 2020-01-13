pub mod buffer;
pub mod factory;
pub mod packets;
pub mod types;

use std::sync::mpsc;

use crate::client;
use crate::network::packets::client_packets;
use crate::network::packets::client_packets::ClientPacket;
use crate::utils;
use crate::utils::rc4;

/* mod.rs (network module) - TCP communications with server and packet controls */

pub fn netstat() {}

impl client::Client {
    /// Connects the client to the server with the provided IP address and sets the socket to nodelay
    pub fn connect(&mut self, ip: String, port: i32) {
        if self.recon.recon_queued == true {
            //were reconnecting, reset the ciphers
            self.game_connection.key_pair = rc4::CipherPair::new_const();
        }
        let full_ip = ip + ":" + &port.to_string();
        if self.base.use_socks {
            let (tx, rx, h) = factory::begin_networking(
                full_ip.clone(),
                self.base.socks_proxy.clone(),
                self.config.factory_delay_ms,
                self.config.factory_stack_size_kb,
            );
            self.game_connection.game_socket.tx = Some(tx);
            self.game_connection.game_socket.rx = Some(rx);
            self.factory_handle = Some(h);
        } else {
            let (tx, rx, h) = factory::begin_networking(
                full_ip.clone(),
                String::new(),
                self.config.factory_delay_ms,
                self.config.factory_stack_size_kb,
            );
            self.game_connection.game_socket.tx = Some(tx);
            self.game_connection.game_socket.rx = Some(rx);
            self.factory_handle = Some(h);
        }
        // self.sleep_thread(10); //reproduces the 100% usage bug
    }
    /// Disconnects the client from its socket
    pub fn disconnect(&mut self) {
        self.is_connected = false;
        self.game_connection.kill_connection();
    }
    /// Sends the packet to the server
    pub fn send(&mut self, mut packet: buffer::Buffer) {
        self.game_connection.cipher_data(&mut packet.data, false);
        self.game_connection.game_socket.send_packet_to_factory(packet);
    }
    /// Receives the packet from the server, return a packet enum
    pub fn receive(&mut self) {
        let packet = self.game_connection.game_socket.receive_packet_from_factory();
        match packet {
            Some(mut pkt) => {
                self.game_connection.cipher_data(&mut pkt.data, true);
                self.evaluate_packet(pkt);
            }
            None => { //todo: clean this spot up a bit???
                self.queue_recon(
                    self.recon.game_id,
                    self.recon.game_key.clone(),
                    self.recon.game_key_time,
                );
                self.disconnect();
                self.sleep_thread(1);
                return;
            }
        }
    }
    /// Wrapper for sending the hello packet and logging into the game world
    pub fn send_hello(&mut self, gid: i32, key: Vec<u8>, key_time: u32) {
        let mut hp = packets::client_packets::Hello::new();
        hp.build_version = self.config.game_version.clone();
        hp.game_id = gid;
        hp.guid = utils::encrypt_string(&self.base.email);
        hp.random1 = utils::rand_i32();
        hp.password = utils::encrypt_string(&self.base.password);
        hp.random2 = utils::rand_i32();
        // hp.secret = utils::encrypt_string(&String::new()); //if using steam/kongregate
        hp.secret = String::new();
        hp.key_time = key_time;
        hp.key = key;
        hp.map_json = String::new();
        hp.entry_tag = String::new();
        hp.game_net = String::from("rotmg");
        hp.game_net_user_id = String::new();
        hp.play_platform = String::from("rotmg");
        hp.platform_token = String::new();
        hp.user_token = String::new();
        hp.client_token = String::from("XTeP7hERdchV5jrBZEYNebAqDPU6tKU6");
        // println!("{:?}", hp); //debugging
        self.send(client_packets::ClientPackets::HelloPacket(hp).write());
    }
}

/// Contains the socket, ciphers, and connection related stuff for the Client
// #[derive(Debug)]
pub struct GameConnection {
    pub game_socket: GameSocket,
    pub key_pair: rc4::CipherPair,
    pub debug_socket: bool,
    pub reading_packets: bool,
}

/// Implements base GameConnection related stuff
impl GameConnection {
    pub fn new() -> GameConnection {
        GameConnection {
            game_socket: GameSocket::new(),
            key_pair: rc4::new_key_pair(rc4::OUTGOING_KEY, rc4::INCOMING_KEY),
            debug_socket: false,
            reading_packets: false,
        }
    }
    pub fn kill_connection(&mut self) {
        //Not sure if the keypairs need to be zeroed out, but heck, why not
        self.reading_packets = false;
        self.key_pair.reset();
        //drop channels
        match self.game_socket.rx.take() {
            Some(rx) => drop(rx),
            None => (),
        };
        match self.game_socket.tx.take() {
            Some(tx) => drop(tx),
            None => (),
        }
    }
    /// Might need to make this return the data buffer
    pub fn cipher_data(&mut self, data: &mut Vec<u8>, incoming: bool) {
        let mut buf_copy = data.clone();
        if incoming == true {
            self.key_pair
                .incoming
                .xor_key_stream(&mut data[5..], &mut buf_copy[5..]);
        } else {
            self.key_pair
                .outgoing
                .xor_key_stream(&mut data[5..], &mut buf_copy[5..]);
        }
    }
}

/// Contains the lower level implementations for read/write to the socket.
/// Basically we deal with the verbosity of Option in here
#[derive(Debug)]
pub struct GameSocket {
    pub tx: Option<mpsc::Sender<buffer::Buffer>>,
    pub rx: Option<mpsc::Receiver<buffer::Buffer>>,
}

impl GameSocket {
    /// Creates a new GameSocket object
    fn new() -> GameSocket {
        GameSocket {
            //initialize fields later, when we are ready to connect
            tx: None,
            rx: None,
        }
    }
    /// Sends the buffer to the ``NetworkFactory``
    pub fn send_packet_to_factory(&mut self, b: buffer::Buffer) {
        match self.tx.as_ref() {
            Some(v) => match v.send(b) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Error sending to netfactory: {}", e);
                }
            },
            None => (), //channel is closed
        }
    }
    /// Receives a packet from the ``NetworkFactory``.
    pub fn receive_packet_from_factory(&self) -> Option<buffer::Buffer> {
        match self.rx.as_ref() {
            Some(v) => {
                match v.recv() {
                    Ok(v) => return Some(v),
                    Err(_e) => {
                        //channel closed
                        // eprintln!("Server dropped the connection: {}", e);
                        return None;
                    }
                };
            }
            None => None, //channel is closed
        }
    }
}
