use core::{fmt, iter::*, marker::PhantomData, mem, ptr};
use std::os::windows::ffi::OsStringExt;
use std::{ffi::*, io};
use winapi::{
    shared::{guiddef::GUID, minwindef::*, windef::HWND, winerror::*},
    um::{errhandlingapi::*, handleapi::*, heapapi::*, setupapi::*, winnt::*},
};

pub use winapi::um::setupapi::HDEVINFO;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct InfoHandle<'h> {
    handle_dev_info: HDEVINFO,
    _lifetime_of_handle: PhantomData<&'h ()>
}

impl<'h> InfoHandle<'h> {
    #[inline]
    pub fn iter<'a>(&self, guid: &GUID) -> InfoIter<'a> {
        InfoIter::from_handle_guid(self.handle_dev_info, guid)
    }
}

impl<'h> Drop for InfoHandle<'h> {
    #[inline]
    fn drop(&mut self) {
        // println!("Drop called");
        unsafe { SetupDiDestroyDeviceInfoList(self.handle_dev_info) };
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct Info<'p> {
    path_ptr: LPCWSTR,
    path_len_in_u16: DWORD,
    _lifetime_of_path: PhantomData<&'p ()>,
}

impl<'p> Info<'p> {
    fn from_device_path(path_ptr: LPCWSTR, path_len_in_u16: DWORD) -> Self {
        Info {
            path_ptr,
            path_len_in_u16,
            _lifetime_of_path: PhantomData,
        }
    }

    /// Copies the path slice into an owned OsString.
    pub fn to_os_string(&self) -> OsString {
        OsString::from_wide(unsafe { core::slice::from_raw_parts(
            self.path_ptr,
            self.path_len_in_u16 as usize
        ) })
    } 

    pub fn path_ptr(&self) -> LPCWSTR {
        self.path_ptr
    }
}

impl fmt::Debug for Info<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.to_os_string())
    }
}

/// This iterator also manages a tiny heap buffer
/// 
/// TODO: is this exact sized?
#[derive(Clone)]
pub struct InfoIter<'iter> {
    handle_dev_info: HDEVINFO, // maybe reused, do NOT free here
    iter_index: DWORD,
    interface_class_guid: *const GUID, // must be non-null
    _lifetime_of_guid: PhantomData<&'iter ()>,
    dev_interface_data: SP_DEVICE_INTERFACE_DATA,
    detail_ptr: PSP_DEVICE_INTERFACE_DETAIL_DATA_W, 
    detail_len: DWORD, // size in u8, not in u16
    detail_cap: DWORD,
    _lifetime_of_detail: PhantomData<&'iter ()>,
}

impl<'iter> InfoIter<'iter> {
    fn from_handle_guid(handle_dev_info: HDEVINFO, guid: &GUID) -> InfoIter<'iter> {
        InfoIter {
            handle_dev_info: handle_dev_info,
            iter_index: 0,
            interface_class_guid: guid as *const _,
            _lifetime_of_guid: PhantomData,
            dev_interface_data: create_sp_dev_interface_data(),
            detail_ptr: core::ptr::null_mut(),
            detail_len: 0,
            detail_cap: 0,
            _lifetime_of_detail: PhantomData,
        }
    }
}

impl fmt::Debug for InfoIter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("InfoIter")
            .field("handle_dev_info", &self.handle_dev_info)
            .field("iter_index", &self.iter_index)
            .field("detail_ptr", &self.detail_ptr)
            .field("detail_len", &self.detail_len)
            .field("detail_cap", &self.detail_cap)
            .finish()
    }
}

// todo: const fn
#[inline]
fn create_sp_dev_interface_data() -> SP_DEVICE_INTERFACE_DATA {
    let mut ans: SP_DEVICE_INTERFACE_DATA = 
        unsafe { mem::MaybeUninit::uninit().assume_init() };
    ans.cbSize = mem::size_of::<SP_DEVICE_INTERFACE_DATA>() as DWORD;
    ans
}

impl<'iter> Drop for InfoIter<'iter> {
    fn drop(&mut self) {
        if self.detail_ptr != core::ptr::null_mut() {
            let heap_handle = unsafe { GetProcessHeap() };
            unsafe { HeapFree(heap_handle, 0, self.detail_ptr as *mut _) };
        }
    }
}

impl<'iter> Iterator for InfoIter<'iter> {
    type Item = io::Result<Info<'iter>>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let ans = unsafe {
            SetupDiEnumDeviceInterfaces(
                self.handle_dev_info,
                core::ptr::null_mut(),
                self.interface_class_guid,
                self.iter_index,
                &self.dev_interface_data as *const _ as *mut _,
            )
        };
        if ans == FALSE {
            return if unsafe { GetLastError() } == ERROR_NO_MORE_ITEMS {
                None
            } else {
                Some(Err(io::Error::last_os_error()))
            };
        }
        loop {
            let ans = unsafe {
                SetupDiGetDeviceInterfaceDetailW(
                    self.handle_dev_info,
                    &self.dev_interface_data as *const _ as *mut _,
                    self.detail_ptr,
                    self.detail_cap,
                    &self.detail_len as *const _ as *mut _,
                    core::ptr::null_mut(),
                )
            };
            if ans == TRUE {
                break;
            }
            if unsafe { GetLastError() } == ERROR_INSUFFICIENT_BUFFER {
                let heap_handle = unsafe { GetProcessHeap() };
                self.detail_ptr = unsafe {
                    if self.detail_ptr == core::ptr::null_mut() {
                        HeapAlloc(heap_handle, 0, self.detail_len as usize) as *mut _
                    } else {
                        HeapReAlloc(
                            heap_handle,
                            0,
                            self.detail_ptr as *mut _,
                            self.detail_len as usize,
                        ) as *mut _
                    }
                };
                unsafe {
                    (*self.detail_ptr).cbSize =
                        core::mem::size_of::<PSP_DEVICE_INTERFACE_DETAIL_DATA_W>() as DWORD
                };
                self.detail_cap = self.detail_len;
            } else {
                return Some(Err(io::Error::last_os_error()));
            }
        }
        self.iter_index += 1;
        let ret = Info::from_device_path(
            unsafe { &(*self.detail_ptr).DevicePath as *const _ },
            (self.detail_len / 2) - 3, // path_len_in_u16
        );
        Some(Ok(ret))
    }
}

impl<'iter> FusedIterator for InfoIter<'iter> {}

/// Typestate struct
pub struct Device;

/// Typestate struct
pub struct Interface;

#[derive(Debug)]
pub struct ListOptions<'g, TYPE, OUTPUT> {
    class_guid: *const GUID,
    _lifetime_of_guid: PhantomData<&'g ()>,
    enumerator: PCWSTR,
    hwnd_parent: HWND,
    flags: DWORD,
    _typestate_setup_type: PhantomData<TYPE>,
    _typestate_output: PhantomData<OUTPUT>,
}

impl<TYPE, OUTPUT> ListOptions<'_, TYPE, OUTPUT> {
    #[inline]
    pub const unsafe fn new_unchecked() -> Self {
        Self {
            class_guid: ptr::null(),
            _lifetime_of_guid: PhantomData,
            enumerator: ptr::null(),
            hwnd_parent: ptr::null_mut(),
            flags: 0,
            _typestate_setup_type: PhantomData,
            _typestate_output: PhantomData,
        }
    }

    #[inline]
    pub unsafe fn flags(&mut self, flags: DWORD) -> &mut Self {
        self.flags = flags;
        self
    }

    #[inline]
    pub unsafe fn class_guid(&mut self, class_guid: &GUID) -> &mut Self {
        self.class_guid = class_guid as *const _;
        self
    }

    #[inline]
    pub fn hwnd_parent(&mut self, hwnd_parent: HWND) -> &mut Self {
        self.hwnd_parent = hwnd_parent;
        self
    }

    #[inline]
    pub fn present(&mut self) -> &mut Self {
        self.flags |= DIGCF_PRESENT;
        self
    }

    #[inline]
    pub fn profile(&mut self) -> &mut Self {
        self.flags |= DIGCF_PROFILE;
        self
    }

    #[inline]
    pub fn enumerator(&mut self, enumerator: &OsStr) -> &mut Self {
        self.enumerator = enumerator as *const _ as *const u16;
        self
    }
}

impl<'h, TYPE, OUTPUT> ListOptions<'_, TYPE, OUTPUT> 
where 
    OUTPUT: From<InfoHandle<'h>>
{
    #[inline]
    pub fn list(&self) -> io::Result<OUTPUT> {
        let handle_dev_info = unsafe {
            SetupDiGetClassDevsW(
                self.class_guid,
                self.enumerator,
                self.hwnd_parent,
                self.flags,
            )
        };
        if handle_dev_info == INVALID_HANDLE_VALUE {
            Err(io::Error::last_os_error())
        } else {
            Ok(OUTPUT::from(InfoHandle { 
                handle_dev_info,
                _lifetime_of_handle: PhantomData 
            }))
        }
    }
}

impl<'g, OUTPUT> ListOptions<'g, Device, OUTPUT> {
    #[inline]
    pub const fn all_devices() -> ListOptions<'g, Device, OUTPUT> {
        Self {
            class_guid: ptr::null(),
            _lifetime_of_guid: PhantomData,
            enumerator: ptr::null(),
            hwnd_parent: ptr::null_mut(),
            flags: DIGCF_ALLCLASSES,
            _typestate_setup_type: PhantomData,
            _typestate_output: PhantomData,
        }
    }

    #[inline]
    pub const fn device_by_class(class_guid: &'g GUID) -> Self {
        Self {
            class_guid: class_guid as *const _,
            _lifetime_of_guid: PhantomData,
            enumerator: ptr::null(),
            hwnd_parent: ptr::null_mut(),
            flags: 0,
            _typestate_setup_type: PhantomData,
            _typestate_output: PhantomData,
        }
    }
}

impl<'g, OUTPUT> ListOptions<'g, Interface, OUTPUT> {
    #[inline]
    pub const fn all_interfaces() -> ListOptions<'g, Interface, OUTPUT> {
        Self {
            class_guid: ptr::null(),
            _lifetime_of_guid: PhantomData,
            enumerator: ptr::null(),
            hwnd_parent: ptr::null_mut(),
            flags: DIGCF_DEVICEINTERFACE | DIGCF_ALLCLASSES,
            _typestate_setup_type: PhantomData,
            _typestate_output: PhantomData,
        }
    }

    #[inline]
    pub const fn interface_by_class(class_guid: &'g GUID) -> ListOptions<'g, Interface, OUTPUT> {
        Self {
            class_guid: class_guid as *const _,
            _lifetime_of_guid: PhantomData,
            enumerator: ptr::null(),
            hwnd_parent: ptr::null_mut(),
            flags: DIGCF_DEVICEINTERFACE,
            _typestate_setup_type: PhantomData,
            _typestate_output: PhantomData,
        }
    }

    #[inline]
    pub fn supports_default(&mut self) -> &mut Self {
        self.flags |= DIGCF_DEFAULT;
        self
    }
}
