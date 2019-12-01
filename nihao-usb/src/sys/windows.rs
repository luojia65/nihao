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

impl IntoIterator for DeviceList {
    type Item = <Devices as Iterator>::Item;
    type IntoIter = Devices;

    fn into_iter(self) -> Devices {
        Devices { iter: self.info_handle.into_iter() }
    }
}

#[derive(Debug, Clone)]
pub struct Devices {
    iter: usb::InfoIntoIter,
}

impl<'iter> Iterator for Devices {
    type Item = io::Result<Device>;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|res| res.map(|info| Device { info }))
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Device {
    info: usb::Info,
}

impl Device {
    pub fn open(&self) -> io::Result<Handle> {
        self.info.open().map(|handle| Handle { winusb_interface: handle })
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Handle {
    winusb_interface: usb::WinUsbInterface
}

impl Handle {
    pub fn device_descriptor(&self) -> io::Result<crate::DeviceDescriptor> {
        self.winusb_interface.device_descriptor().map(|s| s.into())
    }

    pub fn speed(&self) -> io::Result<crate::Speed> {
        self.winusb_interface.speed().map(|s| s.into())
    }

    pub fn read_pipe(&self, pipe_index: u8, buf: &mut [u8]) -> io::Result<usize> {
        self.winusb_interface.read_pipe(pipe_index, buf)
    }

    pub fn write_pipe(&self, pipe_index: u8, buf: &[u8]) -> io::Result<usize> {
        self.winusb_interface.write_pipe(pipe_index, buf)
    }

    pub fn flush_pipe(&self, pipe_index: u8) -> io::Result<()> {
        self.winusb_interface.flush_pipe(pipe_index)
    }
}
