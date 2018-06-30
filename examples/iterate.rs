extern crate mac_address;

use mac_address::MacAddresses;
use std::{
    io::{self, Write}, process,
};

const WRITE_ERR: &str = "Error writing to stdout";

fn main() {
    let addresses = MacAddresses::new().expect("Could not get collection of MAC addresses");

    println!("No loopback");
    write(addresses);

    let addresses = MacAddresses::with_loopback(true)
        .expect("Could not get collection of MAC addresses (with loopback)");

    println!("With loopback");
    write(addresses);
}

fn write(addresses: MacAddresses) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for address in addresses {
        write!(handle, "{}", address).expect(WRITE_ERR);

        if let Some(name) = address.name() {
            writeln!(handle, " - {}", name).expect(WRITE_ERR);
        } else {
            writeln!(handle, "").expect(WRITE_ERR);
        }
    }
}
