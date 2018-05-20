#[cfg(target_pointer_width = "32")]
#[path = "win32.rs"]
mod win;

#[cfg(target_pointer_width = "64")]
#[path = "win64.rs"]
mod win;

use MacAddressError::{self, *};

use winapi::shared::winerror::ERROR_SUCCESS;
use winapi::shared::ws2def::AF_UNSPEC;

use std::ptr::null_mut;

use self::win::{GetAdaptersAddresses, PIP_ADAPTER_ADDRESSES};

const GAA_FLAG_NONE: win::ULONG = 0x0000;

/// Uses bindings to the `Iphlpapi.h` Windows header to fetch the interface devices
/// list with [GetAdaptersAddresses][https://msdn.microsoft.com/en-us/library/windows/desktop/aa365915(v=vs.85).aspx]
/// then loops over the returned list until it finds a network device with a MAC address,
/// and returns it. If it fails to find a device, it returns a `NoDevicesFound` error.
pub fn get_mac() -> Result<Option<[u8; 6]>, MacAddressError> {
    unsafe {
        let mut buf_len = 0;

        // This will get the number of bytes we need to allocate for all devices
        GetAdaptersAddresses(
            AF_UNSPEC as u32,
            GAA_FLAG_NONE,
            null_mut(),
            null_mut(),
            &mut buf_len,
        );

        // Allocate `buf_len` bytes, and create a raw pointer to it
        let mut adapters_list = vec![0u8; buf_len as usize];
        let adapter_addresses: PIP_ADAPTER_ADDRESSES = adapters_list.as_mut_ptr() as *mut _;

        // Get our list of adapters
        let result = GetAdaptersAddresses(
            // [IN] Family
            AF_UNSPEC as u32,
            // [IN] Flags
            GAA_FLAG_NONE,
            // [IN] Reserved
            null_mut(),
            // [INOUT] AdapterAddresses
            adapter_addresses as *mut _,
            // [INOUT] SizePointer
            &mut buf_len,
        );

        // Make sure we were successful
        if result != ERROR_SUCCESS {
            return Err(InternalError);
        }

        let mut bytes = [0; 6];

        // Pointer to the current location in the linked list
        let mut ptr = adapters_list.as_mut_ptr() as PIP_ADAPTER_ADDRESSES;

        loop {
            // Break if we've gone through all devices
            if ptr.is_null() {
                break;
            }

            // Copy over the 6 MAC address bytes to the buffer
            for (i, j) in bytes.iter_mut().zip((*ptr).PhysicalAddress.iter()) {
                *i = *j as u8;
            }

            // If its not local-loopback (MAC address = all zeroes), return it
            if bytes.iter().any(|&x| x != 0) {
                return Ok(Some(bytes));
            }

            // Otherwise go to the next device
            ptr = (*ptr).Next;
        }
    }

    Ok(None)
}

pub fn get_mac_from_name(name: &str) -> Result<Option<[u8; 6]>, MacAddressError> {
    unsafe {
        let mut buf_len = 0;

        // This will get the number of bytes we need to allocate for all devices
        GetAdaptersAddresses(
            AF_UNSPEC as u32,
            GAA_FLAG_NONE,
            null_mut(),
            null_mut(),
            &mut buf_len,
        );

        // Allocate `buf_len` bytes, and create a raw pointer to it
        let mut adapters_list = vec![0u8; buf_len as usize];
        let adapter_addresses: PIP_ADAPTER_ADDRESSES = adapters_list.as_mut_ptr() as *mut _;

        // Get our list of adapters
        let result = GetAdaptersAddresses(
            // [IN] Family
            AF_UNSPEC as u32,
            // [IN] Flags
            GAA_FLAG_NONE,
            // [IN] Reserved
            null_mut(),
            // [INOUT] AdapterAddresses
            adapter_addresses as *mut _,
            // [INOUT] SizePointer
            &mut buf_len,
        );

        // Make sure we were successful
        if result != ERROR_SUCCESS {
            return Err(InternalError);
        }

        let mut bytes = [0; 6];

        // Pointer to the current location in the linked list
        let mut ptr = adapters_list.as_mut_ptr() as PIP_ADAPTER_ADDRESSES;

        loop {
            // Break if we've gone through all devices
            if ptr.is_null() {
                break;
            }

            let adapt_name = String::from_utf16(&get_utf16_bytes((*ptr).FriendlyName))
                .map_err(|_| MacAddressError::InternalError)?;

            if adapt_name == name {
                // Copy over the 6 MAC address bytes to the buffer
                for (i, j) in bytes.iter_mut().zip((*ptr).PhysicalAddress.iter()) {
                    *i = *j as u8;
                }

                return Ok(Some(bytes));
            }

            // Otherwise go to the next device
            ptr = (*ptr).Next;
        }
    }

    Ok(None)
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
