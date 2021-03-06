#![allow(non_upper_case_globals)]

use core::{
    iter::FusedIterator,
    marker::PhantomData,
    mem,
    task::Poll,
    pin::Pin,
};
use std::io;
use super::setup;
use crate::{
    DeviceDescriptor,
    InterfaceDescriptor,
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
        minwinbase::{OVERLAPPED},
        fileapi::{CreateFileW, OPEN_EXISTING},
        handleapi::{CloseHandle, INVALID_HANDLE_VALUE},
        errhandlingapi::GetLastError,
        winusb::{
            WinUsb_Initialize, WinUsb_Free,
            WinUsb_GetDescriptor,
            WinUsb_GetOverlappedResult,
            WinUsb_QueryDeviceInformation,
            WinUsb_QueryInterfaceSettings,
            WinUsb_QueryPipe,
            WinUsb_ReadPipe,
            WinUsb_WritePipe,
            WinUsb_FlushPipe,
            WinUsb_ResetPipe,
            WinUsb_AbortPipe,
            WINUSB_INTERFACE_HANDLE,
            USB_INTERFACE_DESCRIPTOR,
        },
    },
    shared::{
        minwindef::{FALSE, DWORD, UCHAR},
        winerror::{
            ERROR_NO_MORE_ITEMS,
            ERROR_IO_PENDING,
            ERROR_IO_INCOMPLETE,
        },
        usbiodef::GUID_DEVINTERFACE_USB_DEVICE,
        usbspec::{
            USB_DEVICE_DESCRIPTOR,
            USB_DEVICE_DESCRIPTOR_TYPE,
            USB_DEVICE_SPEED,
            UsbLowSpeed, UsbFullSpeed, UsbHighSpeed, UsbSuperSpeed,
        },
        winusbio::{
            DEVICE_SPEED,
            WINUSB_PIPE_INFORMATION,
        },
    },
};

pub trait ListOptionsExt<'g, 'h> {
    fn all_usb_interfaces() -> setup::ListOptions<'g, setup::Interface, InfoHandle<'h>>;
}

impl<'g, 'h> ListOptionsExt<'g, 'h> for setup::ListOptions<'g, setup::Interface, InfoHandle<'h>> {
    fn all_usb_interfaces() -> setup::ListOptions<'g, setup::Interface, InfoHandle<'h>> {
        setup::ListOptions::interface_by_class(&GUID_DEVINTERFACE_USB_DEVICE)
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct InfoHandle<'h> {
    inner: setup::InfoHandle<'h>,
}

impl<'h> From<setup::InfoHandle<'h>> for InfoHandle<'h> {
    fn from(src: setup::InfoHandle<'h>) -> InfoHandle {
        InfoHandle { inner: src }
    }
}

impl<'h> InfoHandle<'h> {
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
    pub fn open<'h>(&self) -> io::Result<WinUsbInterface<'h>> {
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
pub struct WinUsbInterface<'h> {
    device_handle: HANDLE,
    winusb_handle: WINUSB_INTERFACE_HANDLE,
    _lifetime_of_handles: PhantomData<&'h ()>,
}

impl<'h> WinUsbInterface<'h> {
    fn new(device_handle: HANDLE, winusb_handle: WINUSB_INTERFACE_HANDLE) -> Self {
        WinUsbInterface {
            device_handle, winusb_handle, _lifetime_of_handles: PhantomData
        }
    }

    pub fn device_descriptor(&self) -> io::Result<USB_DEVICE_DESCRIPTOR> {
        let mut dest = mem::MaybeUninit::<USB_DEVICE_DESCRIPTOR>::uninit();
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
            dest.as_mut_ptr() as *mut u8,
            core::mem::size_of::<USB_DEVICE_DESCRIPTOR>() as DWORD,
            &len as *const _ as *mut _
        ) };
        if ans == FALSE {
            return Err(io::Error::last_os_error())
        }
        Ok(unsafe { dest.assume_init() })
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

    pub fn interface_settings(&self, alternate_interface_number: u8) 
        -> io::Result<Option<USB_INTERFACE_DESCRIPTOR>>
    {
        let mut dest = mem::MaybeUninit::<USB_INTERFACE_DESCRIPTOR>::uninit();
        let ans = unsafe {
            WinUsb_QueryInterfaceSettings(
                self.winusb_handle,
                alternate_interface_number,
                dest.as_mut_ptr()
            )
        };
        if ans == FALSE {
            if unsafe { GetLastError() } == ERROR_NO_MORE_ITEMS {
                return Ok(None);
            }
            return Err(io::Error::last_os_error());
        }
        Ok(Some(unsafe { dest.assume_init() }))
    }

    pub fn query_pipe(&self, interface_number: u8, pipe_index: u8) 
        -> io::Result<Option<WINUSB_PIPE_INFORMATION>>
    {
        let mut dest = mem::MaybeUninit::<WINUSB_PIPE_INFORMATION>::uninit();
        let ans = unsafe {
            WinUsb_QueryPipe(
                self.winusb_handle,
                interface_number,
                pipe_index,
                dest.as_mut_ptr()
            )
        };
        if ans == FALSE {
            if unsafe { GetLastError() } == ERROR_NO_MORE_ITEMS {
                return Ok(None);
            }
            return Err(io::Error::last_os_error());
        }
        Ok(Some(unsafe { dest.assume_init() }))
    }

    pub fn write_pipe(&self, pipe_index: u8, buf: &[u8]) -> io::Result<usize> {
        let mut len = mem::MaybeUninit::<DWORD>::uninit();
        let ans = unsafe { WinUsb_WritePipe(
            self.winusb_handle,
            pipe_index,
            buf.as_ptr() as *mut u8,
            buf.len() as DWORD,
            len.as_mut_ptr(),
            core::ptr::null_mut(),
        ) }; 
        if ans == FALSE {
            return Err(io::Error::last_os_error())
        }
        Ok(unsafe { len.assume_init() } as usize)
    }

    pub fn read_pipe(&self, pipe_index: u8, buf: &mut [u8]) -> io::Result<usize> {
        let mut len = mem::MaybeUninit::<DWORD>::uninit();
        let ans = unsafe { WinUsb_ReadPipe(
            self.winusb_handle,
            pipe_index,
            buf.as_ptr() as *mut u8,
            buf.len() as DWORD,
            len.as_mut_ptr(),
            core::ptr::null_mut(),
        ) }; 
        if ans == FALSE {
            return Err(io::Error::last_os_error())
        }
        Ok(unsafe { len.assume_init() } as usize)
    }

    pub fn flush_pipe(&self, pipe_index: u8) -> io::Result<()> {
        let ans = unsafe { WinUsb_FlushPipe (
            self.winusb_handle,
            pipe_index,
        ) };
        if ans == FALSE {
            return Err(io::Error::last_os_error())
        }
        Ok(())
    }

    pub fn reset_pipe(&self, pipe_index: u8) -> io::Result<()> {
        let ans = unsafe { WinUsb_ResetPipe (
            self.winusb_handle,
            pipe_index,
        ) };
        if ans == FALSE {
            return Err(io::Error::last_os_error())
        }
        Ok(())
    }

    pub fn abort_pipe(&self, pipe_index: u8) -> io::Result<()> {
        let ans = unsafe { WinUsb_AbortPipe (
            self.winusb_handle,
            pipe_index,
        ) };
        if ans == FALSE {
            return Err(io::Error::last_os_error())
        }
        Ok(())
    }

    // WinUsb_WritePipe with OVERLAPPED
    pub fn write_pipe_overlapped(&self, pipe_index: u8, buf: &[u8])     
        -> io::Result<Pin<Box<OVERLAPPED>>>
    {
        let mut overlapped = Box::pin(unsafe { 
            mem::MaybeUninit::<OVERLAPPED>::uninit().assume_init()
        });
        overlapped.hEvent = core::ptr::null_mut();
        let ans = unsafe { WinUsb_WritePipe(
            self.winusb_handle,
            pipe_index,
            buf.as_ptr() as *mut u8,
            buf.len() as DWORD,
            core::ptr::null_mut(),
            &*overlapped as *const _ as *mut _,
        ) }; 
        if ans == FALSE {
            if unsafe { GetLastError() } == ERROR_IO_PENDING {
                return Ok(overlapped)
            }
            return Err(io::Error::last_os_error())
        }
        // todo: check if correct
        // return Ok(unsafe { overlapped.assume_init() })
        panic!("returned true for overlapped write")
    }

    pub fn read_pipe_overlapped(&self, pipe_index: u8, buf: &mut [u8]) 
        -> io::Result<Pin<Box<OVERLAPPED>>>
    {
        let mut overlapped = Box::pin(unsafe { 
            mem::MaybeUninit::<OVERLAPPED>::uninit().assume_init()
        });
        overlapped.hEvent = core::ptr::null_mut();
        let ans = unsafe { WinUsb_ReadPipe(
            self.winusb_handle,
            pipe_index,
            buf.as_ptr() as *mut u8,
            buf.len() as DWORD,
            core::ptr::null_mut(),
            &*overlapped as *const _ as *mut _,
        ) }; 
        if ans == FALSE {
            if unsafe { GetLastError() } == ERROR_IO_PENDING {
                return Ok(overlapped)
            }
            return Err(io::Error::last_os_error())
        }
        // todo: check if correct
        // return Ok(unsafe { overlapped.assume_init() })
        panic!("returned true for overlapped read")
    }

    pub fn poll_overlapped(&self, overlapped: Pin<&OVERLAPPED>) 
        -> Poll<io::Result<usize>>
    {
        let mut bytes_transferred = mem::MaybeUninit::uninit();
        let ans = unsafe { WinUsb_GetOverlappedResult(
            self.winusb_handle,
            &*overlapped as *const _ as *mut _,
            bytes_transferred.as_mut_ptr(),
            FALSE,
        ) };
        if ans == FALSE {
            if unsafe { GetLastError() } == ERROR_IO_INCOMPLETE {
                return Poll::Pending;
            }
            return Poll::Ready(Err(io::Error::last_os_error()))
        }
        return Poll::Ready(Ok(unsafe { bytes_transferred.assume_init() as usize }))
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

impl From<USB_INTERFACE_DESCRIPTOR> for InterfaceDescriptor {
    fn from(src: USB_INTERFACE_DESCRIPTOR) -> InterfaceDescriptor {
        InterfaceDescriptor {
            length: src.bLength,
            descriptor_type: src.bDescriptorType,
            interface_number: src.bInterfaceNumber,
            alternate_setting: src.bAlternateSetting,
            num_endpoints: src.bNumEndpoints,
            interface_class: src.bInterfaceClass,
            interface_subclass: src.bInterfaceSubClass,
            interface_protocol: src.bInterfaceProtocol,
            index_interface: src.iInterface,
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
