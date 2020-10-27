//! `mac_address` provides a cross platform way to retrieve the MAC address of
//! network hardware. See [the Wikipedia
//! entry](https://en.wikipedia.org/wiki/MAC_address) for more information.
//!
//! Supported platforms: Linux, Windows, MacOS, FreeBSD

#![deny(missing_docs)]

#[cfg(target_os = "windows")]
extern crate winapi;

#[cfg(any(target_os = "linux", target_os = "macos", target_os = "freebsd"))]
extern crate nix;

#[cfg(test)]
extern crate serde_test;

#[cfg(test)]
extern crate serde_json;

#[cfg(target_os = "windows")]
#[path = "windows.rs"]
mod os;

#[cfg(any(target_os = "linux", target_os = "macos", target_os = "freebsd"))]
#[path = "linux.rs"]
mod os;

mod iter;
pub use iter::MacAddressIterator;

/// Possible errors when attempting to retrieve a MAC address.
///
/// Eventually will expose more detailed error information.
#[derive(Debug)]
pub enum MacAddressError {
    /// Signifies an internal API error has occurred.
    InternalError,
}

#[cfg(any(target_os = "linux", target_os = "macos", target_os = "freebsd"))]
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
                InternalError => "Internal API error",
            }
        )?;

        Ok(())
    }
}

impl std::error::Error for MacAddressError {
    fn description(&self) -> &str {
        use MacAddressError::*;

        match self {
            InternalError => "Internal API error",
        }
    }
}

/// An error that may occur when parsing a MAC address string.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MacParseError {
    /// Parsing of the MAC address contained an invalid digit.
    InvalidDigit,
    /// The MAC address did not have the correct length.
    InvalidLength,
}

impl std::fmt::Display for MacParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match *self {
            MacParseError::InvalidDigit => "invalid digit",
            MacParseError::InvalidLength => "invalid length",
        })
    }
}

impl std::error::Error for MacParseError {}

/// Contains the individual bytes of the MAC address.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "&str"))]
pub struct MacAddress {
    bytes: [u8; 6],
}

impl MacAddress {
    /// Creates a new `MacAddress` struct from the given bytes.
    pub fn new(bytes: [u8; 6]) -> MacAddress {
        MacAddress { bytes }
    }
}

impl From<[u8; 6]> for MacAddress {
    fn from(v: [u8; 6]) -> Self {
        MacAddress::new(v)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for MacAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self)
    }
}

/// Calls the OS-specific function for retrieving the MAC address of the first
/// network device containing one, ignoring local-loopback.
pub fn get_mac_address() -> Result<Option<MacAddress>, MacAddressError> {
    let bytes = os::get_mac(None)?;

    Ok(match bytes {
        Some(b) => Some(MacAddress { bytes: b }),
        None => None,
    })
}

/// Attempts to look up the MAC address of an interface via the specified name.
/// **NOTE**: On Windows, this uses the `FriendlyName` field of the adapter, which
/// is the same name shown in the "Network Connections" Control Panel screen.
pub fn mac_address_by_name(name: &str) -> Result<Option<MacAddress>, MacAddressError> {
    let bytes = os::get_mac(Some(name))?;

    Ok(match bytes {
        Some(b) => Some(MacAddress { bytes: b }),
        None => None,
    })
}

impl MacAddress {
    /// Returns the array of MAC address bytes.
    pub fn bytes(self) -> [u8; 6] {
        self.bytes
    }
}

impl std::str::FromStr for MacAddress {
    type Err = MacParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut array = [0u8; 6];

        let mut nth = 0;
        for byte in input.split(|c| c == ':' || c == '-') {
            if nth == 6 {
                return Err(MacParseError::InvalidLength);
            }

            array[nth] = u8::from_str_radix(byte, 16).map_err(|_| MacParseError::InvalidDigit)?;

            nth += 1;
        }

        if nth != 6 {
            return Err(MacParseError::InvalidLength);
        }

        Ok(MacAddress::new(array))
    }
}

impl std::convert::TryFrom<&'_ str> for MacAddress {
    type Error = MacParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_str_colon() {
        let string = "80:FA:5B:41:10:6B";
        let address = string.parse::<MacAddress>().unwrap();
        assert_eq!(address.bytes(), [128, 250, 91, 65, 16, 107]);
        assert_eq!(&format!("{}", address), string);
    }

    #[test]
    fn parse_str_hyphen() {
        let string = "01-23-45-67-89-AB";
        let address = string.parse::<MacAddress>().unwrap();
        assert_eq!(address.bytes(), [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB]);
        assert_eq!(format!("{}", address), string.replace("-", ":"));
    }

    #[test]
    fn parse_invalid_length() {
        let string = "80:FA:5B:41:10:6B:AC";
        let address = string.parse::<MacAddress>().unwrap_err();
        assert_eq!(MacParseError::InvalidLength, address);

        let string = "80:FA:5B:41";
        let address = string.parse::<MacAddress>().unwrap_err();
        assert_eq!(MacParseError::InvalidLength, address);
    }

    #[test]
    fn parse_invalid_digit() {
        let string = "80:FA:ZZ:41:10:6B:AC";
        let address = string.parse::<MacAddress>().unwrap_err();
        assert_eq!(MacParseError::InvalidDigit, address);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_works() {
        use serde_test::{assert_tokens, Token};
        let mac: MacAddress = "80:FA:5B:41:10:6B".parse().unwrap();

        assert_tokens(&mac, &[Token::BorrowedStr("80:FA:5B:41:10:6B")]);

        #[derive(serde::Serialize, serde::Deserialize)]
        struct Test {
            mac: MacAddress,
        }

        assert_eq!(
            serde_json::to_string(&Test { mac }).unwrap(),
            serde_json::to_string::<Test>(
                &serde_json::from_str("{ \"mac\": \"80:FA:5B:41:10:6B\" }").unwrap()
            )
            .unwrap(),
        );
    }
}
