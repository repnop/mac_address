#[cfg(target_pointer_width = "32")]
#[path = "win32.rs"]
mod win;

#[cfg(target_pointer_width = "64")]
#[path = "win64.rs"]
mod win;

use self::win::{GetAdaptersAddresses, PIP_ADAPTER_ADDRESSES};
use super::{MacAddress, MacAddressError};
use std::ptr::null_mut;
use winapi::shared::{winerror::ERROR_SUCCESS, ws2def::AF_UNSPEC};

const GAA_FLAG_NONE: win::ULONG = 0x0000;

/// Uses bindings to the `Iphlpapi.h` Windows header to fetch the interface devices
/// list with [GetAdaptersAddresses][https://msdn.microsoft.com/en-us/library/windows/desktop/aa365915(v=vs.85).aspx]
/// then loops over the returned list until it finds a network device with a MAC address,
/// and returns it. If it fails to find a device, it returns a `NoDevicesFound` error.
pub struct MacAddresses {
    #[allow(dead_code)]
    adapters_list: Vec<u8>,
    ptr: PIP_ADAPTER_ADDRESSES,
    include_loopback: bool,
}

impl MacAddresses {
    pub fn new() -> Result<Self, MacAddressError> {
        Self::with_loopback(false)
    }

    pub fn with_loopback(include_loopback: bool) -> Result<Self, MacAddressError> {
        let mut buf_len = 0;

        // This will get the number of bytes we need to allocate for all devices
        unsafe {
            GetAdaptersAddresses(
                AF_UNSPEC as u32,
                GAA_FLAG_NONE,
                null_mut(),
                null_mut(),
                &mut buf_len,
            );
        }

        // Allocate `buf_len` bytes, and create a raw pointer to it
        let mut adapters_list = vec![0u8; buf_len as usize];
        let adapter_addresses = adapters_list.as_mut_ptr() as PIP_ADAPTER_ADDRESSES;

        // Get our list of adapters
        let result = unsafe {
            GetAdaptersAddresses(
                // [IN] Family
                AF_UNSPEC as u32,
                // [IN] Flags
                GAA_FLAG_NONE,
                // [IN] Reserved
                null_mut(),
                // [INOUT] AdapterAddresses
                adapter_addresses,
                // [INOUT] SizePointer
                &mut buf_len,
            )
        };

        // Make sure we were successful
        if result != ERROR_SUCCESS {
            return Err(MacAddressError::InternalError);
        }

        // Pointer to the current location in the linked list
        let ptr = adapters_list.as_mut_ptr() as PIP_ADAPTER_ADDRESSES;

        Ok(Self {
            adapters_list,
            ptr,
            include_loopback,
        })
    }
}

impl Iterator for MacAddresses {
    type Item = MacAddress;

    fn next(&mut self) -> Option<Self::Item> {
        let mut bytes = [0; 6];

        loop {
            // Break if we've gone through all devices
            if self.ptr.is_null() {
                break None;
            }

            // Copy over the 6 MAC address bytes to the buffer
            unsafe {
                for (i, j) in bytes.iter_mut().zip((*self.ptr).PhysicalAddress.iter()) {
                    *i = *j as u8;
                }
            }

            // If the user does not want local-loopback (MAC address = all zeroes), continue
            // to next device.
            let ret_val = if !self.include_loopback && !bytes.iter().any(|&x| x != 0) {
                None
            } else {
                let adapt_name =
                    String::from_utf16(unsafe { &get_utf16_bytes((*self.ptr).FriendlyName) }).ok();

                Some(MacAddress::new(bytes, adapt_name))
            };

            // Go to the next device
            self.ptr = unsafe { (*self.ptr).Next };

            if ret_val.is_some() {
                break ret_val;
            }
        }
    }
}

unsafe fn get_utf16_bytes(ptr: *mut u16) -> Vec<u16> {
    assert!(!ptr.is_null());

    let mut offset = 0;
    let mut vec = Vec::with_capacity(64);

    let mut c = *ptr;

    while c != 0 {
        vec.push(c);
        offset += 1;
        c = *ptr.offset(offset);
    }

    vec
}
