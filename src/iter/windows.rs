use windows_sys::Win32::NetworkManagement::IpHelper::IP_ADAPTER_ADDRESSES_LH;

use crate::os;
use crate::{MacAddress, MacAddressError};

/// An iterator over all available MAC addresses on the system.
pub struct MacAddressIterator {
    // So we don't UAF during iteration.
    _buffer: os::AdaptersList,
    ptr: *mut IP_ADAPTER_ADDRESSES_LH,
}

impl MacAddressIterator {
    /// Creates a new `MacAddressIterator`.
    pub fn new() -> Result<MacAddressIterator, MacAddressError> {
        let adapters = os::get_adapters()?;
        let ptr = unsafe { adapters.ptr() };

        Ok(Self {
            _buffer: adapters,
            ptr,
        })
    }
}

impl Iterator for MacAddressIterator {
    type Item = MacAddress;

    fn next(&mut self) -> Option<MacAddress> {
        if self.ptr.is_null() {
            None
        } else {
            let bytes = unsafe { os::convert_mac_bytes(self.ptr) };

            #[cfg(target_pointer_width = "32")]
            {
                self.ptr = unsafe { self.ptr.read_unaligned().Next };
            }

            #[cfg(not(target_pointer_width = "32"))]
            {
                self.ptr = unsafe { (*self.ptr).Next };
            }

            Some(MacAddress::new(bytes))
        }
    }
}
