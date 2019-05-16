#[cfg(windows)]
pub mod windows;

#[cfg(windows)]
pub use windows::{devices, DeviceList, Devices, Device, Handle};
