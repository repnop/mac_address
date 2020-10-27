use std::{ffi::OsString, os::windows::ffi::OsStringExt, ptr, slice};
use winapi::shared::{ntdef::ULONG, winerror::ERROR_SUCCESS, ws2def::AF_UNSPEC};
use winapi::um::{iphlpapi::GetAdaptersAddresses, iptypes::PIP_ADAPTER_ADDRESSES};
use MacAddressError;

const GAA_FLAG_NONE: ULONG = 0x0000;

/// Uses bindings to the `Iphlpapi.h` Windows header to fetch the interface devices
/// list with [GetAdaptersAddresses][https://msdn.microsoft.com/en-us/library/windows/desktop/aa365915(v=vs.85).aspx]
/// then loops over the returned list until it finds a network device with a MAC address,
/// and returns it.
///
/// If it fails to find a device, it returns a `NoDevicesFound` error.
pub fn get_mac(name: Option<&str>) -> Result<Option<[u8; 6]>, MacAddressError> {
    let mut adapters = get_adapters()?;
    // Pointer to the current location in the linked list
    let mut ptr = adapters.as_mut_ptr() as PIP_ADAPTER_ADDRESSES;

    loop {
        // Break if we've gone through all devices
        if ptr.is_null() {
            break;
        }

        // Copy over the 6 MAC address bytes to the buffer.
        // PhysicalAddress is a `[u8; 8]`, until `TryFrom` stabilizes, this
        // is the easiest way to turn it into a `[u8; 6]`.
        let bytes = unsafe { *((&(*ptr).PhysicalAddress).as_ptr() as *const [u8; 6]) };

        if let Some(name) = name {
            let adapter_name = unsafe { construct_string((*ptr).FriendlyName) };

            if &adapter_name == name {
                return Ok(Some(bytes));
            }
        } else if bytes.iter().any(|&x| x != 0) {
            return Ok(Some(bytes));
        }

        // Otherwise go to the next device
        ptr = unsafe { (*ptr).Next };
    }

    Ok(None)
}

pub(crate) fn get_adapters() -> Result<Vec<u8>, MacAddressError> {
    let mut buf_len = 0;

    // This will get the number of bytes we need to allocate for all devices
    unsafe {
        GetAdaptersAddresses(
            AF_UNSPEC as u32,
            GAA_FLAG_NONE,
            ptr::null_mut(),
            ptr::null_mut(),
            &mut buf_len,
        );
    }

    // Allocate `buf_len` bytes, and create a raw pointer to it
    let mut adapters_list = vec![0u8; buf_len as usize];
    let adapter_addresses: PIP_ADAPTER_ADDRESSES = adapters_list.as_mut_ptr() as *mut _;

    // Get our list of adapters
    let result = unsafe {
        GetAdaptersAddresses(
            // [IN] Family
            AF_UNSPEC as u32,
            // [IN] Flags
            GAA_FLAG_NONE,
            // [IN] Reserved
            ptr::null_mut(),
            // [INOUT] AdapterAddresses
            adapter_addresses as *mut _,
            // [INOUT] SizePointer
            &mut buf_len,
        )
    };

    // Make sure we were successful
    if result != ERROR_SUCCESS {
        return Err(MacAddressError::InternalError);
    }

    Ok(adapters_list)
}

unsafe fn construct_string(ptr: *mut u16) -> OsString {
    let slice = slice::from_raw_parts(ptr, get_null_position(ptr));
    OsStringExt::from_wide(slice)
}

unsafe fn get_null_position(ptr: *mut u16) -> usize {
    assert!(!ptr.is_null());

    for i in 0.. {
        if *ptr.offset(i) == 0 {
            return i as usize;
        }
    }

    unreachable!()
}
