#![allow(dead_code)]

use nix::ifaddrs::*;
use nix::sys::socket::SockAddr;
use MacAddressError::{self, *};

/// Uses the `getifaddrs` call to retrieve a list of network interfaces on the
/// host device and returns the first MAC address listed that isn't
/// local-loopback.
pub fn get_mac() -> Result<[u8; 6], MacAddressError> {
    let ifiter = getifaddrs()?;

    for interface in ifiter {
        if let Some(address) = interface.address {
            if let SockAddr::Link(link) = address {
                let bytes = link.addr();

                if bytes.iter().any(|&x| x != 0) {
                    return Ok(bytes);
                }
            }
        }
    }

    Err(NoDevicesFound)
}
