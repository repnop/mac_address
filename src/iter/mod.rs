#[cfg(target_os = "windows")]
#[path = "windows.rs"]
mod internal;

#[cfg(any(
    target_os = "linux",
    target_os = "macos",
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "android",
))]
#[path = "linux.rs"]
mod internal;

pub use internal::MacAddressIterator;
