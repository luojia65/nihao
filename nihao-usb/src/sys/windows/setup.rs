use core::{iter::*, marker::PhantomData, ptr, mem};
use std::{ffi::OsStr, io};
use winapi::{
    shared::{guiddef::GUID, minwindef::*, windef::HWND, winerror::*},
    um::{handleapi::*, setupapi::*, errhandlingapi::*, winnt::PCWSTR},
};

#[derive(Debug, Clone)]
pub struct DeviceInfoSet {
    handle: HDEVINFO,
}

impl DeviceInfoSet {
    pub fn iter<'set, 'b, 'g>(&'set self, guid: &'g GUID) -> DeviceInfoIter<'b, 'g>
    where
        'set: 'b + 'g,
    {
        DeviceInfoIter::from_handle_guid(self.handle, guid)
    }
}

impl Drop for DeviceInfoSet {
    fn drop(&mut self) {
        unsafe { SetupDiDestroyDeviceInfoList(self.handle) };
    }
}

#[derive(Debug)]
pub struct DeviceInfo {

}

pub struct DeviceInfoIter<'b, 'g> {
    handle: HDEVINFO,
    iter_index: DWORD,
    interface_class_guid: *const GUID, // must be non-null
    _lifetime_of_guid: PhantomData<&'g ()>,
    dev_interface_data: SP_DEVICE_INTERFACE_DATA,
    buffer: PSP_DEVICE_INTERFACE_DETAIL_DATA_W,
    buf_len: DWORD,
    _lifetime_of_buffer: PhantomData<&'b ()>,
}

impl<'b, 'g> DeviceInfoIter<'b, 'g> {
    fn from_handle_guid(handle: HDEVINFO, guid: &'g GUID) -> DeviceInfoIter<'b, 'g> {
        DeviceInfoIter {
            handle: handle,
            iter_index: 0,
            interface_class_guid: guid as *const _,
            _lifetime_of_guid: PhantomData,
            dev_interface_data: create_sp_dev_interface_data(),
            buffer: core::ptr::null_mut(),
            buf_len: 0,
            _lifetime_of_buffer: PhantomData,
        }
    }
}

fn create_sp_dev_interface_data() -> SP_DEVICE_INTERFACE_DATA {
    let mut ans = unsafe { mem::uninitialized::<SP_DEVICE_INTERFACE_DATA>() };
    ans.cbSize = mem::size_of::<SP_DEVICE_INTERFACE_DATA>() as DWORD;
    ans
}

impl<'b, 'g> Iterator for DeviceInfoIter<'b, 'g> {
    type Item = io::Result<DeviceInfo>;

    fn next(&mut self) -> Option<Self::Item> {
        let ans = unsafe { SetupDiEnumDeviceInterfaces(
            self.handle,
            core::ptr::null_mut(), 
            self.interface_class_guid,
            self.iter_index,
            &self.dev_interface_data as *const _ as *mut _,
        ) };
        if ans == FALSE {
            return if unsafe { GetLastError() } == ERROR_NO_MORE_ITEMS { None } 
            else { Some(Err(io::Error::last_os_error())) };
        }
        self.iter_index += 1;
        // todo unimplemented
        Some(Err(io::Error::last_os_error()))
        // None
    }
}

impl<'b, 'g> FusedIterator for DeviceInfoIter<'b, 'g> {}

pub struct Device;

pub struct Interface;

#[derive(Debug)]
pub struct ListOptions<TARGET> {
    class_guid: *const GUID,
    enumerator: PCWSTR,
    hwnd_parent: HWND,
    flags: DWORD,
    _typestate: PhantomData<TARGET>,
}

impl<TARGET> ListOptions<TARGET> {
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

    pub fn list(&self) -> io::Result<DeviceInfoSet> {
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

impl ListOptions<Device> {
    pub const fn all_devices() -> ListOptions<Device> {
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

impl ListOptions<Interface> {
    pub const fn all_interfaces() -> ListOptions<Interface> {
        Self {
            class_guid: ptr::null(),
            enumerator: ptr::null(),
            hwnd_parent: ptr::null_mut(),
            flags: DIGCF_DEVICEINTERFACE | DIGCF_ALLCLASSES,
            _typestate: PhantomData,
        }
    }

    pub const fn interface_by_class(class_guid: &GUID) -> ListOptions<Interface> {
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
