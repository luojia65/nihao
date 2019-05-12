use core::ptr::NonNull;
use std::io;
use super::setup;
// use winapi::{
//     um::winusb::WINUSB_INTERFACE_HANDLE,
// };

pub use winapi::shared::usbiodef::GUID_DEVINTERFACE_USB_DEVICE;

pub struct InterfaceHandle {
    handle: NonNull<()>,
}

#[derive(Debug, Clone)]
pub struct InfoHandle {
    inner: setup::InfoHandle,
}

impl From<setup::InfoHandle> for InfoHandle {
    fn from(src: setup::InfoHandle) -> InfoHandle {
        InfoHandle { inner: src }
    }
}

impl InfoHandle {
    pub fn iter<'a>(&'a self) -> InfoIter<'a> {
        InfoIter { inner: self.inner.iter(&GUID_DEVINTERFACE_USB_DEVICE) }
    }
}

pub struct InfoIter<'iter> {
    inner: setup::InfoIter<'iter>,
}

impl<'iter> Iterator for InfoIter<'iter> {
    type Item = io::Result<Info<'iter>>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|r| r.map(|i| Info { inner: i }))
    }
}

#[derive(Debug)]
pub struct Info<'a> {
    inner: setup::Info<'a>,
}

pub trait ListOptionsExt {
    fn all_usb_interfaces() -> setup::ListOptions<setup::Interface, InfoHandle>;
}

impl ListOptionsExt for setup::ListOptions<setup::Interface, InfoHandle> {
    fn all_usb_interfaces() -> setup::ListOptions<setup::Interface, InfoHandle> {
        setup::ListOptions::interface_by_class(&GUID_DEVINTERFACE_USB_DEVICE)
    }
}
