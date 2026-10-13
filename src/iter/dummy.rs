use crate::{MacAddress, MacAddressError};

/// An iterator over all available MAC addresses on the system.
pub struct MacAddressIterator;

impl MacAddressIterator {
    /// Creates a new `MacAddressIterator`.
    pub fn new() -> Result<MacAddressIterator, MacAddressError> {
        Ok(Self {})
    }
}

impl Iterator for MacAddressIterator {
    type Item = MacAddress;

    fn next(&mut self) -> Option<MacAddress> {
        None
    }
}
