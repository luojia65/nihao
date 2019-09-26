pub mod setup;
pub mod usb;

use std::io;

pub fn devices<'list>() -> io::Result<DeviceList<'list>> {
    use usb::ListOptionsExt;
    let handle = setup::ListOptions::all_usb_interfaces()
        .present()
        .list()?;
    Ok(handle.into())
}

#[derive(Debug, Clone)]
pub struct DeviceList<'list> {
    info_handle: usb::InfoHandle<'list>,
}

impl<'list> From<usb::InfoHandle<'list>> for DeviceList<'list> {
    fn from(src: usb::InfoHandle) -> DeviceList {
        DeviceList { info_handle: src }
    }
}

impl<'list> DeviceList<'list> {
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
    pub fn open<'handle>(&self) -> io::Result<Handle<'handle>> {
        self.info.open().map(|handle| Handle { winusb_interface: handle })
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Handle<'handle> {
    winusb_interface: usb::WinUsbInterface<'handle>
}

impl Handle<'_> {
    pub fn device_descriptor(&self) -> io::Result<crate::DeviceDescriptor> {
        self.winusb_interface.device_descriptor().map(|s| s.into())
    }

    pub fn speed(&self) -> io::Result<crate::Speed> {
        self.winusb_interface.speed().map(|s| s.into())
    }

    pub fn read_pipe<'pipe>(&self, pipe_index: u8) -> io::Result<ReadPipe<'pipe, '_>> {
        // todo: check for invalid pipe index and return error
        Ok(ReadPipe { pipe_index, winusb_interface: &self.winusb_interface })
    }

    pub fn write_pipe<'pipe>(&self, pipe_index: u8) -> io::Result<WritePipe<'pipe, '_>> {
        // todo: check for invalid pipe index and return error
        Ok(WritePipe { pipe_index, winusb_interface: &self.winusb_interface })
    }
}

pub struct ReadPipe<'pipe, 'handle> {
    pipe_index: u8,
    winusb_interface: &'pipe usb::WinUsbInterface<'handle>,
}

pub struct WritePipe<'pipe, 'handle> {
    pipe_index: u8,
    winusb_interface: &'pipe usb::WinUsbInterface<'handle>,
}

impl io::Read for ReadPipe<'_, '_> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.winusb_interface.read_pipe(self.pipe_index, buf)
    }
}

impl io::Write for WritePipe<'_, '_> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.winusb_interface.write_pipe(self.pipe_index, buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(()) // todo!
    }
}

// impl Read for Pipe
// impl Write for Pipe
// impl AsyncRead for Pipe
// impl AsyncWrite for Pipe
