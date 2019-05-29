pub mod setup;
pub mod usb;

use std::io;

pub fn devices() -> io::Result<DeviceList> {
    use usb::ListOptionsExt;
    let handle = setup::ListOptions::all_usb_interfaces()
        .present()
        .list()?;
    Ok(handle.into())
}

#[derive(Debug, Clone)]
pub struct DeviceList {
    info_handle: usb::InfoHandle,
}

impl From<usb::InfoHandle> for DeviceList {
    fn from(src: usb::InfoHandle) -> DeviceList {
        DeviceList { info_handle: src }
    }
}

impl DeviceList {
    pub fn iter<'iter>(&self) -> Devices<'iter> {
        Devices { iter: self.info_handle.iter() }
    }

    pub fn len(&self) -> usize {
        self.iter().count()
    } 
}

#[derive(Debug, Clone)]
pub struct Devices<'iter> {
    iter: usb::InfoIter<'iter>,
}

impl<'iter> Iterator for Devices<'iter> {
    type Item = io::Result<Device<'iter>>;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|res| res.map(|info| Device { info }))
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Device<'device> {
    info: usb::Info<'device>,
}

impl<'device> Device<'device> {
    pub fn open(&self) -> io::Result<Handle> {
        self.info.open().map(|handle| Handle { handle })
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Handle {
    handle: usb::WinUsbHandle
}

impl Handle {
    pub fn device_descriptor(&self) -> io::Result<crate::DeviceDescriptor> {
        self.handle.device_descriptor()
    }
}
