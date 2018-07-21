use nix::{ifaddrs, sys::socket::SockAddr};
use {MacAddress, MacAddressError};

/// An iterator over all available MAC addresses on the system.
pub struct MacAddressIterator {
    iter: ifaddrs::InterfaceAddressIterator,
}

impl MacAddressIterator {
    /// Creates a new `MacAddressIterator`.
    pub fn new() -> Result<MacAddressIterator, MacAddressError> {
        Ok(Self {
            iter: ifaddrs::getifaddrs()?,
        })
    }
}

impl Iterator for MacAddressIterator {
    type Item = MacAddress;

    fn next(&mut self) -> Option<MacAddress> {
        let intf = self.iter.next()?;

        if let SockAddr::Link(link) = intf.address? {
            Some(MacAddress::new(link.addr()))
        } else {
            None
        }
    }
}
