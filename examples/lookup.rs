extern crate mac_address;

use mac_address::mac_address_by_name;

fn main() {
    #[cfg(target_os = "linux")]
    let name = "eth0";

    #[cfg(target_os = "windows")]
    let name = "Ethernet";

    match mac_address_by_name(name) {
        Ok(Some(ma)) => {
            println!("MAC addr of {} = {}", name, ma);
            println!("bytes = {:?}", ma.bytes());
        }
        Ok(None) => println!("Interface \"{}\" not found", name),
        Err(e) => println!("{:?}", e),
    }
}
