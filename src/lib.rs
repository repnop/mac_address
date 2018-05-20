//! `mac_address` provides a cross platform way to retrieve the MAC address of
//! network hardware. See [the Wikipedia
//! entry](https://en.wikipedia.org/wiki/MAC_address) for more information.
//!
//! Supported platforms: Linux, Windows

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
///
/// Eventually will expose more detailed error information.
#[derive(Debug)]
pub enum MacAddressError {
    /// Signifies an internal API error has occurred.
    InternalError,
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
        }
    }
}

/// Contains the individual bytes of the MAC address.
#[derive(Debug, Clone, Copy)]
pub struct MacAddress {
    bytes: [u8; 6],
}

/// Calls the OS-specific function for retrieving the MAC address of the first
/// network device containing one, ignoring local-loopback.
pub fn get_mac_address() -> Result<Option<MacAddress>, MacAddressError> {
    let bytes = os::get_mac()?;

    Ok(match bytes {
        Some(b) => Some(MacAddress { bytes: b }),
        None => None,
    })
}

/// Attempts to look up the MAC address of an interface via the specified name.
/// **NOTE**: On Windows, this uses the `FriendlyName` field of the adapter, which
/// is the same name shown in the "Network Connections" Control Panel screen.
pub fn mac_address_by_name(name: &str) -> Result<Option<MacAddress>, MacAddressError> {
    let bytes = os::get_mac_from_name(name)?;

    Ok(match bytes {
        Some(b) => Some(MacAddress { bytes: b }),
        None => None,
    })
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
