extern crate mac_address;

use mac_address::get_mac_address;

fn main() {
    match get_mac_address() {
        Ok(ma) => {
            println!("MAC addr = {}", ma);
            println!("bytes = {:?}", ma.bytes());
        }
        Err(e) => println!("{:?}", e),
    }
}
