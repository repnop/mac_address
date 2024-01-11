use std::{net::IpAddr, str::FromStr};

use mac_address::get_mac_address_by_ip;

// UDP 'connect' to a remote IP (Google's DNS) and
// then see which local IP address we were bound to.
//
// NOTE: this is a nice portable way to use the routing
// table and sends no actual packets
fn lookup_default_adapter_ip() -> std::io::Result<IpAddr> {
    let udp_sock = std::net::UdpSocket::bind(("0.0.0.0", 0))?;
    udp_sock.connect((IpAddr::from_str("8.8.8.8").unwrap(), 53))?;
    Ok(udp_sock.local_addr()?.ip())
}

fn main() -> std::io::Result<()> {
    // find a useful IP local address to test against
    let local_ip = lookup_default_adapter_ip()?;
    // find the MacAddress of the Adapter with this IP
    match get_mac_address_by_ip(&local_ip) {
        Ok(Some(ma)) => {
            println!("MAC addr = {}", ma);
            println!("bytes = {:?}", ma.bytes());
        }
        Ok(None) => println!("No MAC address found."),
        Err(e) => println!("{:?}", e),
    }
    Ok(())
}
