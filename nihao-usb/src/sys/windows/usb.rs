use std::io;
use super::setup;
use crate::DeviceDescriptor;

use winapi::{
    um::{
        winnt::{
            GENERIC_READ, GENERIC_WRITE, 
            FILE_SHARE_READ, FILE_SHARE_WRITE,
            FILE_ATTRIBUTE_NORMAL,
            LANG_NEUTRAL,
        },
        winbase::{FILE_FLAG_OVERLAPPED},
        fileapi::{CreateFileW, OPEN_EXISTING},
        handleapi::{CloseHandle, INVALID_HANDLE_VALUE},
        winusb::{
            WinUsb_Initialize, WinUsb_Free,
            WinUsb_GetDescriptor,
            WINUSB_INTERFACE_HANDLE
        },
    },
    shared::{
        minwindef::{FALSE, DWORD},
        usbiodef::GUID_DEVINTERFACE_USB_DEVICE,
        usbspec::{
            USB_DEVICE_DESCRIPTOR, USB_DEVICE_DESCRIPTOR_TYPE,
        },
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

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct InfoHandle {
    inner: setup::InfoHandle,
}

impl From<setup::InfoHandle> for InfoHandle {
    fn from(src: setup::InfoHandle) -> InfoHandle {
        InfoHandle { inner: src }
    }
}

impl InfoHandle {
    pub fn iter<'a>(&self) -> InfoIter<'a> {
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

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Info<'a> {
    inner: setup::Info<'a>,
}

impl<'a> Info<'a> {
    pub fn open(&self) -> io::Result<WinUsbHandle> {
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
        Ok(WinUsbHandle { winusb_handle })
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct WinUsbHandle {
    winusb_handle: WINUSB_INTERFACE_HANDLE,
}

impl WinUsbHandle {
    pub fn device_descriptor(&self) -> io::Result<DeviceDescriptor> {
        let desc: USB_DEVICE_DESCRIPTOR = unsafe { core::mem::zeroed() };
        let len = 0;
        // Although this function only fails when handle is null which is impossible here
        // and the boolean value returned is not needed, there could exist errors when, 
        // for example, this device is plugged out during operation, when the descriptor 
        // is actually not changed. 
        // ref: https://docs.microsoft.com/en-us/windows/desktop/api/winusb/nf-winusb-winusb_getdescriptor
        unsafe { WinUsb_GetDescriptor(
            self.winusb_handle,
            USB_DEVICE_DESCRIPTOR_TYPE,
            0,
            LANG_NEUTRAL,
            &desc as *const _ as *mut _,
            core::mem::size_of::<USB_DEVICE_DESCRIPTOR>() as DWORD,
            &len as *const _ as *mut _
        ) };
        // If the descriptor is not changed, the `len` variant remains zero. 
        // I personally suggests to judge if this reading operation is successful by 
        // validating if `len` does not equal to zero, otherwise return the error value.
        // If there is a wiser way please fire an issue to let us know. Thanks! -- Luo Jia
        if len == 0 {
            return Err(io::Error::last_os_error())
        }
        Ok(desc.into())
    }
}

impl Drop for WinUsbHandle {
    fn drop(&mut self) {
        unsafe { WinUsb_Free(self.winusb_handle) };
    }
}

impl From<USB_DEVICE_DESCRIPTOR> for DeviceDescriptor {
    fn from(src: USB_DEVICE_DESCRIPTOR) -> DeviceDescriptor {
        DeviceDescriptor {        
            length: src.bLength,
            descriptor_type: src.bDescriptorType,
            bcd_usb: src.bcdUSB,
            device_class: src.bDeviceClass,
            device_sub_class: src.bDeviceSubClass,
            device_protocol: src.bDeviceProtocol,
            max_packet_size_0: src.bMaxPacketSize0,
            id_vendor: src.idVendor,
            id_product: src.idProduct,
            bcd_device: src.bcdDevice,
            manufacturer: src.iManufacturer,
            product: src.iProduct,
            serial_number: src.iSerialNumber,
            num_configurations: src.bNumConfigurations,
        }
    }
}
