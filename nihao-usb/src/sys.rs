#[cfg(windows)]
pub mod windows;

#[cfg(windows)]
pub use windows::{devices, Devices, Device, Handle};
