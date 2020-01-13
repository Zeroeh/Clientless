use std::thread;
use std::time;

use realmlib;

fn main() {
    let config = realmlib::Config::new();
    let index = config.index;
    let amount = config.amount;
    let mut clients = realmlib::accounts_to_clients(realmlib::read_accounts(), config);
    for _ in index..amount {
        realmlib::launch_client(clients.remove(index));
    }

    loop {
        thread::sleep(time::Duration::from_secs(120));
    }
}
