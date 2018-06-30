extern crate mac_address;

use mac_address::{get_mac_address, MacAddresses};

fn main() {
    match get_mac_address() {
        Ok(Some(ma)) => {
            println!("MAC addr = {}", ma);
            println!("bytes = {:?}", ma.bytes());
        }
        Ok(None) => println!("No MAC address found."),
        Err(e) => println!("{:?}", e),
    }

    let addresses = MacAddresses::new().expect("Error creating iterator");

    for address in addresses {
        if let Some(name) = address.name() {
            println!("{}", name);
        }

        println!("{}", address);
    }
}
