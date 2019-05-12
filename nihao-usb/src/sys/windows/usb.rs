use std::io;
use super::setup;

use winapi::{
    um::{
        winnt::{
            GENERIC_READ, GENERIC_WRITE, 
            FILE_SHARE_READ, FILE_SHARE_WRITE,
            FILE_ATTRIBUTE_NORMAL,
        },
        winbase::{FILE_FLAG_OVERLAPPED},
        fileapi::{CreateFileW, OPEN_EXISTING},
        handleapi::{CloseHandle, INVALID_HANDLE_VALUE},
        winusb::{WinUsb_Initialize, WINUSB_INTERFACE_HANDLE}
    },
    shared::{
        minwindef::FALSE,
        usbiodef::GUID_DEVINTERFACE_USB_DEVICE
    },
};

pub trait ListOptionsExt {
    fn all_usb_interfaces() -> setup::ListOptions<setup::Interface, InfoHandle>;
}

impl ListOptionsExt for setup::ListOptions<setup::Interface, InfoHandle> {
    fn all_usb_interfaces() -> setup::ListOptions<setup::Interface, InfoHandle> {
        setup::ListOptions::interface_by_class(&GUID_DEVINTERFACE_USB_DEVICE)
    }
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Info<'a> {
    inner: setup::Info<'a>,
}

impl<'a> Info<'a> {
    pub fn open(&self) -> io::Result<InterfaceHandle> {
        let device_handle = unsafe { CreateFileW(
            self.inner.path_ptr(),
            GENERIC_READ | GENERIC_WRITE,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            core::ptr::null_mut(),
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL | FILE_FLAG_OVERLAPPED,
            core::ptr::null_mut()
        ) };
        if device_handle == INVALID_HANDLE_VALUE {
            return Err(io::Error::last_os_error())
        }
        let winusb_handle: WINUSB_INTERFACE_HANDLE = core::ptr::null_mut();
        let result = unsafe { WinUsb_Initialize(
            device_handle,
            &winusb_handle as *const _ as *mut _
        ) };
        if result == FALSE {
            let err = io::Error::last_os_error();
            unsafe { CloseHandle(device_handle) };
            return Err(err)
        }
        Ok(InterfaceHandle { winusb_handle })
    }
}

#[derive(Debug, Clone)]
pub struct InterfaceHandle {
    winusb_handle: WINUSB_INTERFACE_HANDLE,
}
