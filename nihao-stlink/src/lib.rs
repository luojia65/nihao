use core::{
    iter::FusedIterator,
    convert::TryFrom,
    fmt,
};
use std::io;

pub fn handles<'iter>() -> io::Result<Handles<'iter>> {
    nihao_usb::devices().map(|inner| Handles { inner: inner.iter() })
}

#[derive(Debug, Clone)]
pub struct Handles<'iter> {
    inner: nihao_usb::Devices<'iter>,
}

impl<'iter> Iterator for Handles<'iter> {
    type Item = io::Result<Handle<'iter>>;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: Which error should we drop or handle?

        self.inner.next().map(|r| r
            .and_then(|d| d.open())
            .and_then(|h| Handle::try_from(h).map_err(|(_h, err)| err.into()))
        )

        // use TryFromHandleError::*;
        // while let Some(Ok(d)) = self.inner.next() {
        //     // unable to open, continue
        //     let h = if let Ok(h) = d.open() { h } else { continue };
        //     match Handle::try_from(h) {
        //         Ok(h) => return Some(Ok(h)),
        //         Err((_h, IoError(e))) => return Some(Err(e)),
        //         Err(_) => continue,
        //     }
        // }
        // None
    }
}

impl FusedIterator for Handles<'_> {}

/// A handle of a device connection. 
/// 
/// The connection between the programmer and the system may break at any time. 
/// When this happens, methods of `Handle` should return `Err` values other than 
/// normal `Ok` with results.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Handle<'h> {
    inner: nihao_usb::Handle<'h>,
    version: Version,
}

impl<'h> Handle<'h> {
    pub fn into_inner(self) -> nihao_usb::Handle<'h> {
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

impl<'h> TryFrom<nihao_usb::Handle<'h>> for Handle<'h> {
    type Error = (nihao_usb::Handle<'h>, TryFromHandleError);

    fn try_from(src: nihao_usb::Handle<'h>) -> Result<Handle<'h>, Self::Error> {
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