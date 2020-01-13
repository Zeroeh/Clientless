use std::fs::File;
use std::io::BufReader;
use std::thread;
use std::time;

extern crate serde;
use serde::{Deserialize, Serialize};

pub mod account;
pub mod client;
pub mod game;
pub mod network;
pub mod utils;

/// Creates a new account object
pub fn new_account() {}

impl client::Client {
    /// Sets variables that might not be created at creation time, then starts the client. Returns the Client back if it ever needs to be relaunched.
    pub fn start(mut self, gid: i32, key: Vec<u8>, key_time: u32) -> Self {
        self.is_running = true;
        self.time_keeper.thread_delay_ms = self.config.thread_delay_ms as i32;
        self.recon.game_id = gid;
        self.recon.game_key = key;
        self.recon.game_key_time = key_time;
        self.recon.current_server = self.base.server_ip.clone();
        self.game_loop();
        self
    }
}

/// Launches ALL accounts in the supplied Vec
pub fn launch_clients(mut clients: Vec<client::Client>) {
    clients.reverse(); //load the accounts in the same order as the accounts file
    for _ in 0..clients.len() {
        launch_client(clients.pop().unwrap());
        thread::sleep(time::Duration::from_millis(3000));
    }
}

pub fn launch_client(client: client::Client) {
    thread::Builder::new()
        .name(client.base.email.clone())
        .stack_size(client.config.client_thread_stack_size_kb * 1024)
        .spawn(move || {
            client.start(-2, Vec::new(), u32::max_value());
        })
        .unwrap();
}

pub fn read_accounts() -> Vec<account::Account> {
    account::read_accounts()
}

pub fn accounts_to_clients(mut accounts: Vec<account::Account>, c: Config) -> Vec<client::Client> {
    let mut cli = Vec::new();
    for _ in 0..accounts.len() {
        cli.push(accounts.pop().unwrap().create_client(c.clone()));
    }
    cli.reverse(); //original order
    cli
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub amount: usize,
    pub index: usize,
    pub conn_limit: i32,
    pub game_version: String,
    pub thread_delay_ms: u64,
    pub factory_delay_ms: u64,
    pub factory_stack_size_kb: usize,
    pub client_thread_stack_size_kb: usize,
    pub save_delay_secs: i32,
}

impl Config {
    pub fn new() -> Config {
        let c: Config =
            serde_json::from_reader(BufReader::new(File::open("config/settings.json").unwrap()))
                .unwrap();
        c
    }
}
