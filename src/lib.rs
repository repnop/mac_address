//! `mac_address` provides a cross platform way to retrieve the MAC
//! address of network hardware. See [the Wikipedia entry](https://en.wikipedia.org/wiki/MAC_address)
//! for more information.
//!
//! Currenly does not support MacOS.

#![deny(missing_docs)]

#[cfg(target_os = "windows")]
extern crate winapi;

#[cfg(target_os = "linux")]
extern crate nix;

#[cfg(target_os = "windows")]
#[path = "windows/mod.rs"]
mod os;

#[cfg(target_os = "linux")]
#[path = "linux.rs"]
mod os;

/// Possible errors when attempting to retrieve a MAC address.
#[derive(Debug)]
pub enum MacAddressError {
    /// Signifies an internal API error has occurred.
    InternalError,
    /// Failed to find a device with a MAC address.
    NoDevicesFound,
}

#[cfg(target_os = "linux")]
impl From<nix::Error> for MacAddressError {
    fn from(_: nix::Error) -> MacAddressError {
        MacAddressError::InternalError
    }
}

impl std::fmt::Display for MacAddressError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use MacAddressError::*;

        write!(
            f,
            "{}",
            match self {
                &InternalError => "Internal API error",
                &NoDevicesFound => "No network interface devices found",
            }
        )?;

        Ok(())
    }
}

impl std::error::Error for MacAddressError {
    fn description(&self) -> &str {
        use MacAddressError::*;

        match self {
            &InternalError => "Internal API error",
            &NoDevicesFound => "No network interface devices found",
        }
    }
}

/// Contains the individual bytes of the MAC address.
#[derive(Debug, Clone, Copy)]
pub struct MacAddress {
    bytes: [u8; 6],
}

/// Calls the OS-specific function for retrieving the MAC address of the first
/// network device that contains one.
pub fn get_mac_address() -> Result<MacAddress, MacAddressError> {
    let bytes = os::get_mac()?;

    Ok(MacAddress { bytes })
}

impl MacAddress {
    /// Returns the array of MAC address bytes.
    pub fn bytes(&self) -> [u8; 6] {
        self.bytes
    }
}

impl std::fmt::Display for MacAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let _ = write!(
            f,
            "{:<02X}:{:<02X}:{:<02X}:{:<02X}:{:<02X}:{:<02X}",
            self.bytes[0],
            self.bytes[1],
            self.bytes[2],
            self.bytes[3],
            self.bytes[4],
            self.bytes[5]
        );

        Ok(())
    }
}
