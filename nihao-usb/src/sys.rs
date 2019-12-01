#[cfg(any(windows, doc))]
pub mod windows;

#[cfg(any(windows, doc))]
pub use windows::{devices, DeviceList, Devices, DeviceIntoIter, Device, Handle};
