use winapi::um::iptypes::PIP_ADAPTER_ADDRESSES;
use {MacAddress, MacAddressError};

/// An iterator over all available MAC addresses on the system.
pub struct MacAddressIterator {
    #[allow(dead_code)]
    buffer: Vec<u8>,
    ptr: PIP_ADAPTER_ADDRESSES,
}

impl MacAddressIterator {
    /// Creates a new `MacAddressIterator`.
    pub fn new() -> Result<MacAddressIterator, MacAddressError> {
        let mut adapters = ::os::get_adapters()?;
        let ptr = adapters.as_mut_ptr() as PIP_ADAPTER_ADDRESSES;

        Ok(Self {
            buffer: adapters,
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
            // PhysicalAddress is a `[u8; 8]`, until `TryFrom` stabilizes, this
            // is the easiest way to turn it into a `[u8; 6]`.
            let bytes = unsafe { *((&(*self.ptr).PhysicalAddress).as_ptr() as *const [u8; 6]) };
            self.ptr = unsafe { (*self.ptr).Next };

            Some(MacAddress::new(bytes))
        }
    }
}
