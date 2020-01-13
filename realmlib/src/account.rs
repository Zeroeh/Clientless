use std::fs::File;
use std::io::BufReader;

use super::client;

extern crate serde;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub email: String,
    pub password: String,
    pub server_ip: String,
    pub fetch_new_data: bool,
    pub char_id: i32,
    pub module: String,
    pub use_socks: bool,
    pub socks_proxy: String,
    pub use_http: bool,
    pub http_proxy: String,
}

trait IAccount {
    fn get_new_char_id(&mut self);
    fn get_url(s: &str) -> String;
}

/// This trait moves the 'Account' struct methods to the 'Client'
impl IAccount for client::Client {
    fn get_new_char_id(&mut self) {}
    fn get_url<'a>(s: &'a str) -> String {
        String::from(s)
    }
}

impl Account {
    /// Creates a Client struct from an Account struct by transferring ownership
    pub fn create_client(self, c: crate::Config) -> client::Client {
        client::Client::new(self, c)
    }
}

pub fn read_accounts() -> Vec<Account> {
    let a: Vec<Account> =
        serde_json::from_reader(BufReader::new(File::open("config/accounts.json").unwrap()))
            .unwrap();
    a
}
