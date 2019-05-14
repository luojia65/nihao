use core::{
    iter::FusedIterator,
    convert::TryFrom,
    fmt,
};
use std::io;

pub fn handles<'iter>() -> io::Result<Handles<'iter>> {
    nihao_usb::devices().map(|inner| Handles { inner })
}

#[derive(Debug, Clone)]
pub struct Handles<'iter> {
    inner: nihao_usb::Devices<'iter>,
}

impl<'iter> Iterator for Handles<'iter> {
    type Item = io::Result<Handle>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|r| r
            .and_then(|d| d.open())
            .and_then(|h| Handle::try_from(h).map_err(|(_h, err)| err.into()))
        )
    }
}

impl FusedIterator for Handles<'_> {}

/// A handle of a device connection. 
/// 
/// The connection between the programmer and the system may break at any time. 
/// When this happens, methods of `Handle` should return `Err` values other than 
/// normal `Ok` with results.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
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

#[derive(Debug)]
pub enum TryFromHandleError {
    InvalidVendorProductId(u16, u16),
    IoError(io::Error),
}

impl std::error::Error for TryFromHandleError {}

impl fmt::Display for TryFromHandleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<io::Error> for TryFromHandleError {
    fn from(src: io::Error) -> TryFromHandleError {
        TryFromHandleError::IoError(src)
    }
}

impl From<TryFromHandleError> for io::Error {
    fn from(src: TryFromHandleError) -> io::Error {
        io::Error::new(io::ErrorKind::Other, src)
    }
}

impl TryFrom<nihao_usb::Handle> for Handle {
    type Error = (nihao_usb::Handle, TryFromHandleError);

    fn try_from(src: nihao_usb::Handle) -> Result<Handle, Self::Error> {
        use TryFromHandleError::*;
        let desc = match src.device_descriptor() {
            Ok(desc) => desc,
            Err(err) => return Err((src, err.into())),
        };
        if desc.id_vendor != STLINK_VID || desc.id_product != STLINK_V2_PID {
            return Err((src, InvalidVendorProductId(desc.id_vendor, desc.id_product)))
        }
        let ans = Handle { inner: src, version: Version {} };
        Ok(ans)
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Version {
    // TODO
}