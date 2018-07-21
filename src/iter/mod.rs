#[cfg(target_os = "windows")]
#[path = "windows.rs"]
mod internal;

#[cfg(any(target_os = "linux", target_os = "macos"))]
#[path = "linux.rs"]
mod internal;

pub use self::internal::MacAddressIterator;
