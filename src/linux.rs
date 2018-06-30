use super::{MacAddress, MacAddressError};
use nix::{
    ifaddrs::{getifaddrs, InterfaceAddressIterator}, sys::socket::SockAddr,
};

/// Iterator for all MAC addresses on the running machine.
///
/// Uses the `getifaddrs` call to retrieve a list of network interfaces on the
/// host device.
pub struct MacAddresses {
    iter: InterfaceAddressIterator,
    include_loopback: bool,
}

impl MacAddresses {
    /// Create a new `MacAddresses` iterator without local-loopback addresses.
    pub fn new() -> Result<Self, MacAddressError> {
        Self::with_loopback(false)
    }

    /// Create a new `MacAddresses` iterator.
    ///
    /// Optionally include local-loopback addresses with `include_loopback`.
    pub fn with_loopback(include_loopback: bool) -> Result<Self, MacAddressError> {
        let iter = getifaddrs()?;

        Ok(Self {
            iter,
            include_loopback,
        })
    }
}

impl Iterator for MacAddresses {
    type Item = MacAddress;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let interface = self.iter.next()?;

            if let Some(address) = interface.address {
                if let SockAddr::Link(link) = address {
                    let bytes = link.addr();

                    if !self.include_loopback && !bytes.iter().any(|&x| x != 0) {
                        continue;
                    }

                    return Some(MacAddress::new(bytes, Some(interface.interface_name)));
                }
            }
        }
    }
}
