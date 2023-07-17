use crate::{MacAddress, MacAddressError};
use nix::ifaddrs;

/// An iterator over all available MAC addresses on the system.
pub struct MacAddressIterator {
    iter: std::iter::FilterMap<
        ifaddrs::InterfaceAddressIterator,
        fn(ifaddrs::InterfaceAddress) -> Option<MacAddress>,
    >,
}

impl MacAddressIterator {
    /// Creates a new `MacAddressIterator`.
    pub fn new() -> Result<MacAddressIterator, MacAddressError> {
        Ok(Self {
            iter: ifaddrs::getifaddrs()?.filter_map(filter_macs),
        })
    }
}

fn filter_macs(intf: ifaddrs::InterfaceAddress) -> Option<MacAddress> {
    if let Some(link) = intf.address?.as_link_addr() {
        if let Some(addr) = link.addr() {
            return Some(MacAddress::new(addr));
        } else {
            None
        }
    } else {
        None
    }
}

impl Iterator for MacAddressIterator {
    type Item = MacAddress;

    fn next(&mut self) -> Option<MacAddress> {
        self.iter.next()
    }
}
