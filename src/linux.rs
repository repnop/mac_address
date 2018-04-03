#![allow(dead_code)]

use libc::{ioctl, sockaddr, socket, AF_UNIX, SOCK_DGRAM};
use libc::{c_char, c_int, c_short, c_uchar, c_ulong, c_ushort};

use super::MacAddressError;

const SIOCGIFHWADDR: c_ulong = 35111;
const SIOCGIFCONF: c_ulong = 35090;

// Data structures not available in `libc` that are required
// to "properly" get the MAC address
#[repr(C)]
struct Ifreq {
    ifr_name: [c_char; ::libc::IFNAMSIZ],
    data: IfrData,
}

union IfrData {
    ifr_addr: sockaddr,
    ifr_dstaddr: sockaddr,
    ifr_broadaddr: sockaddr,
    ifr_netmask: sockaddr,
    ifr_hwaddr: sockaddr,
    ifr_flags: c_short,
    ifr_ifindex: c_int,
    ifr_metric: c_int,
    ifr_mtu: c_int,
    ifr_map: Ifmap,
    ifr_slave: [c_char; ::libc::IFNAMSIZ],
    ifr_newname: [c_char; ::libc::IFNAMSIZ],
    ifr_data: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Ifmap {
    mem_start: c_ulong,
    mem_end: c_ulong,
    base_addr: c_ushort,
    irq: c_uchar,
    dma: c_uchar,
    port: c_uchar,
}

#[repr(C)]
struct Ifconf {
    ifc_len: c_int,
    data: IfconfData,
}

union IfconfData {
    ifc_buf: *mut c_char,
    ifc_req: *mut Ifreq,
}

/// Uses `libc` to fetch the interface devices list with `ioctl`, then
/// loops over the returned list until it finds a network device with a MAC
/// address, and returns it. If it fails to find a device, it returns a
/// `NoDevicesFound` error (ignores local loopback).
pub fn get_mac() -> Result<[u8; 6], MacAddressError> {
    use MacAddressError::*;

    unsafe {
        // Get a UNIX socket
        let sock = socket(AF_UNIX, SOCK_DGRAM, 0);

        if sock < 0 {
            return Err(InternalError);
        }

        // Allocate some configuration stuff and a buffer to store info
        let mut ifc: Ifconf = ::std::mem::zeroed();
        let mut buf = [0; 1024];

        ifc.ifc_len = 1024;
        ifc.data.ifc_buf = (&mut buf).as_mut_ptr();

        // Get the ifconfig data
        if ioctl(sock, SIOCGIFCONF, &ifc) < 0 {
            return Err(InternalError);
        }

        let ifr = ifc.data.ifc_req;
        let num_interfaces = ifc.ifc_len as usize / ::std::mem::size_of::<Ifreq>();

        let mut bytes = [0; 6];

        // Loop through each interface
        for n in 0..num_interfaces {
            let device = ifr.offset(n as isize);

            // Get the HWADDR, aka MAC address
            if ioctl(sock, SIOCGIFHWADDR, device) < 0 {
                return Err(InternalError);
            }

            // Copy over the bytes to the buffer
            for (i, j) in bytes
                .iter_mut()
                .zip((*device).data.ifr_hwaddr.sa_data.iter())
            {
                *i = *j as u8;
            }

            // If its not local-loopback, return the MAC address
            if bytes.iter().any(|&x| x != 0) {
                return Ok(bytes);
            }
        }
    }

    Err(NoDevicesFound)
}
