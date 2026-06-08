#![allow(dead_code)]

use crate::MacAddressError;

/// Always returns no available MAC address.
pub fn get_mac(_: Option<&str>) -> Result<Option<[u8; 6]>, MacAddressError> {
    Ok(None)
}

pub fn get_ifname(_: &[u8; 6]) -> Result<Option<String>, MacAddressError> {
    Ok(None)
}
