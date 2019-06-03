#![allow(non_upper_case_globals)]

use core::{
    iter::FusedIterator,
    marker::PhantomData
};
use std::io;
use super::setup;
use crate::{
    DeviceDescriptor,
    Speed
};

use winapi::{
    um::{
        winnt::{
            HANDLE,
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
            WinUsb_QueryDeviceInformation,
            WINUSB_INTERFACE_HANDLE
        },
    },
    shared::{
        minwindef::{FALSE, DWORD, UCHAR},
        usbiodef::GUID_DEVINTERFACE_USB_DEVICE,
        usbspec::{
            USB_DEVICE_DESCRIPTOR, USB_DEVICE_DESCRIPTOR_TYPE,
            USB_DEVICE_SPEED,
            UsbLowSpeed, UsbFullSpeed, UsbHighSpeed, UsbSuperSpeed,
        },
        winusbio::{
            DEVICE_SPEED,
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

impl<'iter> FusedIterator for InfoIter<'iter> {}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Info<'a> {
    inner: setup::Info<'a>,
}

impl<'a> Info<'a> {
    pub fn open(&self) -> io::Result<WinUsbInterface> {
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
        Ok(WinUsbInterface::new(device_handle, winusb_handle))
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct WinUsbInterface<'a> {
    device_handle: HANDLE,
    winusb_handle: WINUSB_INTERFACE_HANDLE,
    _lifetime_of_handles: PhantomData<&'a ()>,
}

impl WinUsbInterface<'_> {
    fn new(device_handle: HANDLE, winusb_handle: WINUSB_INTERFACE_HANDLE) -> Self {
        WinUsbInterface {
            device_handle, winusb_handle, _lifetime_of_handles: PhantomData
        }
    }

    pub fn device_descriptor(&self) -> io::Result<USB_DEVICE_DESCRIPTOR> {
        let desc: USB_DEVICE_DESCRIPTOR = unsafe { core::mem::zeroed() };
        let len = 0;
        // This function not only fails when handle is null (which is impossible here),
        // but also fails when there being an error while reading.
        // When this fails, `len` remains zero and `ans` is set `FALSE`.
        // ref: https://docs.microsoft.com/en-us/windows/desktop/api/winusb/nf-winusb-winusb_getdescriptor
        let ans = unsafe { WinUsb_GetDescriptor(
            self.winusb_handle,
            USB_DEVICE_DESCRIPTOR_TYPE,
            0,
            LANG_NEUTRAL,
            &desc as *const _ as *mut _,
            core::mem::size_of::<USB_DEVICE_DESCRIPTOR>() as DWORD,
            &len as *const _ as *mut _
        ) };
        if ans == FALSE {
            return Err(io::Error::last_os_error())
        }
        Ok(desc)
    }

    pub fn speed(&self) -> io::Result<USB_DEVICE_SPEED> {
        let device_speed = 0 as UCHAR;
        // this variable cannot be `static`: otherwise there would be a 
        // STATUS_ACCESS_VIOLATION error with exit code 0xc0000005 returned
        let buf_size = core::mem::size_of::<UCHAR>() as DWORD;
        let ans = unsafe {
            WinUsb_QueryDeviceInformation(
                self.winusb_handle, 
                DEVICE_SPEED,
                &buf_size as *const _ as *mut _, 
                &device_speed as *const _ as *mut _
            )
        };
        // If the device is unplugged during this operation,
        // an error code 22 would be returned indicating the device
        // does not identify the speed command
        if ans == FALSE {
            return Err(io::Error::last_os_error())
        }
        Ok(device_speed.into())
    }
}

impl Drop for WinUsbInterface<'_> {
    fn drop(&mut self) {
        unsafe { 
            // reversed free order in destructor
            WinUsb_Free(self.winusb_handle);
            CloseHandle(self.device_handle);
        }
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

impl From<USB_DEVICE_SPEED> for Speed {
    fn from(src: USB_DEVICE_SPEED) -> Speed {
        match src {
            UsbLowSpeed => Speed::Low,
            UsbFullSpeed => Speed::Full,
            UsbHighSpeed => Speed::High,
            UsbSuperSpeed => Speed::Super,
            _ => Speed::Unknown,
        }
    }
}
