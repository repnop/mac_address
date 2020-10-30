#![allow(dead_code)]

use crate::MacAddressError;
use nix::{ifaddrs::*, sys::socket::SockAddr};

/// Uses the `getifaddrs` call to retrieve a list of network interfaces on the
/// host device and returns the first MAC address listed that isn't
/// local-loopback or if a name was specified, that name.
pub fn get_mac(name: Option<&str>) -> Result<Option<[u8; 6]>, MacAddressError> {
    let ifiter = getifaddrs()?;

    for interface in ifiter {
        if let Some(address) = interface.address {
            if let SockAddr::Link(link) = address {
                let bytes = link.addr();

                if let Some(name) = name {
                    if interface.interface_name == name {
                        return Ok(Some(bytes));
                    }
                } else if bytes.iter().any(|&x| x != 0) {
                    return Ok(Some(bytes));
                }
            }
        }
    }

    Ok(None)
}
