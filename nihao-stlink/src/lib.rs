use core::{
    iter::FusedIterator,
    fmt,
    result
};

#[cfg(not(no_std))]
pub fn handles<'iter>() -> nihao_usb::Result<List> {
    nihao_usb::devices().map(|inner| List { inner })
}

#[derive(Debug)]
pub struct List {
    inner: nihao_usb::List,
}

#[derive(Debug)]
pub struct IntoIter {
    inner: nihao_usb::IntoIter,
}

impl Iterator for IntoIter {
    type Item = result::Result<Handle, OpenError>;

    fn next(&mut self) -> Option<Self::Item> {
        use OpenError::*;
        while let Some(Ok(usb_device)) = self.inner.next() {
            match usb_device.open_stlink() {
                Ok(h) => return Some(Ok(h)),
                Err(UsbError(e)) => return Some(Err(e.into())),
                Err(_) => continue, // unable to open, continue
            }
        }
        None
    }
}

impl FusedIterator for IntoIter {}

pub trait DeviceExt {
    fn open_stlink(&self) -> result::Result<Handle, OpenError>;
}

#[derive(Debug)]
pub enum OpenError {
    InvalidVendorProductId(u16, u16),
    UsbError(nihao_usb::Error),
}

impl std::error::Error for OpenError {}

impl fmt::Display for OpenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<nihao_usb::Error> for OpenError {
    fn from(src: nihao_usb::Error) -> Self {
        OpenError::UsbError(src)
    }
}

impl DeviceExt for nihao_usb::Device {
    fn open_stlink(&self) -> result::Result<Handle, OpenError> {
        use OpenError::*;
        let src = self.open()?;
        let desc = src.device_descriptor()?;
        if desc.id_vendor != STLINK_VID || desc.id_product != STLINK_V2_PID {
            return Err(InvalidVendorProductId(desc.id_vendor, desc.id_product))
        }
        let ans = Handle { inner: src, version: Version {} };
        Ok(ans)
    }
}

/// A handle of a device connection. 
/// 
/// The connection between the programmer and the system may break at any time. 
/// When this happens, methods of `Handle` should return `Err` values other than 
/// normal `Ok` with results.
#[derive(Debug)] // , Hash, Eq, PartialEq
pub struct Handle {
    inner: nihao_usb::Handle,
    version: Version,
}

impl Handle {
    pub fn into_inner(self) -> nihao_usb::Handle {
        self.inner
    }

    pub fn version(&self) -> Version {
        self.version
    }
}

const STLINK_VID: u16 = 0x0483;
const STLINK_V2_PID: u16 = 0x3748;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Version {
    // TODO
}
