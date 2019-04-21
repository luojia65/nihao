use core::{marker::PhantomData, ptr};
use std::{ffi::OsStr, io};
use winapi::{
    shared::{guiddef::GUID, minwindef::*, windef::HWND},
    um::{handleapi::*, setupapi::*, winnt::PCWSTR},
};

#[derive(Debug, Clone)]
pub struct DeviceInfoSet {
    handle: HDEVINFO,
}

impl DeviceInfoSet {
    pub fn iter<'set, 'b, 'g>(&'set self) -> DeviceInfoIter<'b, 'g>
    where
        'set: 'b + 'g,
    {
        DeviceInfoIter {
            handle: self.handle,
            iter_index: 0,
            interface_class_guid: core::ptr::null(),
            _lifetime_of_guid: PhantomData,
            buffer: core::ptr::null_mut(),
            buf_len: 0,
            _lifetime_of_buffer: PhantomData,
        }
    }
}

impl Drop for DeviceInfoSet {
    fn drop(&mut self) {
        unsafe { SetupDiDestroyDeviceInfoList(self.handle) };
    }
}

pub struct DeviceInfo {}

#[derive(Debug)]
pub struct DeviceInfoIter<'b, 'g> {
    handle: HDEVINFO,
    iter_index: DWORD,
    interface_class_guid: *const GUID,
    _lifetime_of_guid: PhantomData<&'g ()>,
    buffer: PSP_DEVICE_INTERFACE_DETAIL_DATA_W,
    buf_len: DWORD,
    _lifetime_of_buffer: PhantomData<&'b ()>,
}

impl<'g> DeviceInfoIter<'_, 'g> {
    pub fn filter_class_guid(&mut self, class_guid: &'g GUID) -> &mut Self {
        self.interface_class_guid = class_guid as *const _;
        self
    }
}

impl<'b, 'g> Iterator for DeviceInfoIter<'b, 'g> {
    type Item = &'b DeviceInfo;

    fn next(&mut self) -> Option<Self::Item> {
        // SetupDiGetDeviceInterfaceDetailW(
        //     self.handle,

        // )
        unimplemented!()
    }
}

pub struct Device;

pub struct Interface;

#[derive(Debug)]
pub struct GetOptions<TARGET> {
    class_guid: *const GUID,
    enumerator: PCWSTR,
    hwnd_parent: HWND,
    flags: DWORD,
    _typestate: PhantomData<TARGET>,
}

impl<TARGET> GetOptions<TARGET> {
    pub const unsafe fn new_unchecked() -> Self {
        Self {
            class_guid: ptr::null(),
            enumerator: ptr::null(),
            hwnd_parent: ptr::null_mut(),
            flags: 0,
            _typestate: PhantomData,
        }
    }

    pub unsafe fn flags(&mut self, flags: DWORD) -> &mut Self {
        self.flags = flags;
        self
    }

    pub unsafe fn class_guid(&mut self, class_guid: &GUID) -> &mut Self {
        self.class_guid = class_guid as *const _;
        self
    }

    pub fn hwnd_parent(&mut self, hwnd_parent: HWND) -> &mut Self {
        self.hwnd_parent = hwnd_parent;
        self
    }

    pub fn present(&mut self) -> &mut Self {
        self.flags |= DIGCF_PRESENT;
        self
    }

    pub fn profile(&mut self) -> &mut Self {
        self.flags |= DIGCF_PROFILE;
        self
    }

    pub fn enumerator(&mut self, enumerator: &OsStr) -> &mut Self {
        self.enumerator = enumerator as *const _ as *const u16;
        self
    }

    pub fn get(&self) -> io::Result<DeviceInfoSet> {
        let handle = unsafe {
            SetupDiGetClassDevsW(
                self.class_guid,
                self.enumerator,
                self.hwnd_parent,
                self.flags,
            )
        };
        if handle == INVALID_HANDLE_VALUE {
            Err(io::Error::last_os_error())
        } else {
            Ok(DeviceInfoSet { handle })
        }
    }
}

impl GetOptions<Device> {
    pub const fn all_devices() -> GetOptions<Device> {
        Self {
            class_guid: ptr::null(),
            enumerator: ptr::null(),
            hwnd_parent: ptr::null_mut(),
            flags: DIGCF_ALLCLASSES,
            _typestate: PhantomData,
        }
    }

    pub const fn device_by_class(class_guid: &GUID) -> Self {
        Self {
            class_guid: class_guid as *const _,
            enumerator: ptr::null(),
            hwnd_parent: ptr::null_mut(),
            flags: 0,
            _typestate: PhantomData,
        }
    }
}

impl GetOptions<Interface> {
    pub const fn all_interfaces() -> GetOptions<Interface> {
        Self {
            class_guid: ptr::null(),
            enumerator: ptr::null(),
            hwnd_parent: ptr::null_mut(),
            flags: DIGCF_DEVICEINTERFACE | DIGCF_ALLCLASSES,
            _typestate: PhantomData,
        }
    }

    pub const fn interface_by_class(class_guid: &GUID) -> GetOptions<Interface> {
        Self {
            class_guid: class_guid as *const _,
            enumerator: ptr::null(),
            hwnd_parent: ptr::null_mut(),
            flags: DIGCF_DEVICEINTERFACE,
            _typestate: PhantomData,
        }
    }

    pub fn supports_default(&mut self) -> &mut Self {
        self.flags |= DIGCF_DEFAULT;
        self
    }
}
