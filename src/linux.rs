#![allow(dead_code)]

use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
};

use crate::MacAddressError;
use nix::ifaddrs::*;

/// Uses the `getifaddrs` call to retrieve a list of network interfaces on the
/// host device and returns the first MAC address listed that isn't
/// local-loopback or if a name was specified, that name.
pub fn get_mac(name: Option<&str>) -> Result<Option<[u8; 6]>, MacAddressError> {
    let ifiter = getifaddrs()?;

    for interface in ifiter {
        if let Some(iface_address) = interface.address {
            if let Some(link) = iface_address.as_link_addr() {
                let bytes = link.addr();

                if let Some(name) = name {
                    if interface.interface_name == name {
                        return Ok(bytes);
                    }
                } else if let Some(bytes) = bytes {
                    if bytes.iter().any(|&x| x != 0) {
                        return Ok(Some(bytes));
                    }
                }
            }
        }
    }

    Ok(None)
}

/// Uses the `getifaddrs` call to retrieve a list of network interfaces on the
/// host device and returns the MAC address that matching the adapter with
/// the given IP
///
/// Because nix returns all of the IP's and MAC's in a combined list, we need
/// to map the IP to an inteface name and track the MAC addresses by interface name
/// and see if there's a match
pub fn get_mac_address_by_ip(ip: &IpAddr) -> Result<Option<[u8; 6]>, MacAddressError> {
    let ifiter = getifaddrs()?;

    let mut ip_on_inferface: Option<String> = None;
    let mut mac_to_interface: HashMap<String, Option<[u8; 6]>> = HashMap::new();
    for interface in ifiter {
        if let Some(iface_address) = interface.address {
            // is this a mac address?
            if let Some(link) = iface_address.as_link_addr() {
                mac_to_interface.insert(interface.interface_name.clone(), link.addr());
                // did we just find what we're looking for?
                if let Some(intf_name) = &ip_on_inferface {
                    if *intf_name == interface.interface_name {
                        return Ok(link.addr());
                    }
                }
            }
            if let Some(adapter_ip) = if let Some(sin4) = iface_address.as_sockaddr_in() {
                // v4 addr?
                Some(IpAddr::from(Ipv4Addr::from(sin4.ip())))
            } else if let Some(sin6) = iface_address.as_sockaddr_in6() {
                // v6 addr?
                Some(IpAddr::from(Ipv6Addr::from(sin6.ip())))
            } else {
                // something else, ignore
                None
            } {
                // found an IP for this adapter - if it's the one we're looking for, save it
                if adapter_ip == *ip {
                    ip_on_inferface = Some(interface.interface_name.clone());
                    if let Some(mac) = mac_to_interface.get(&interface.interface_name) {
                        return Ok(mac.clone());
                    }
                }
            }
        }
    }

    Ok(None)
}

pub fn get_ifname(mac: &[u8; 6]) -> Result<Option<String>, MacAddressError> {
    let ifiter = getifaddrs()?;

    for interface in ifiter {
        if let Some(iface_address) = interface.address {
            if let Some(link) = iface_address.as_link_addr() {
                let bytes = link.addr();

                if bytes == Some(*mac) {
                    return Ok(Some(interface.interface_name));
                }
            }
        }
    }

    Ok(None)
}
