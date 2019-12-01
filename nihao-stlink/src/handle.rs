use core::{
    convert::TryFrom,
    fmt,
};
use std::io;
use crate::version::Version;
use crate::consts::*;

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
    pub fn version(&self) -> Version {
        self.version
    }

    pub fn get_voltage(&self) -> io::Result<Option<f32>> {
        if !self.version.has_trace {
            return Ok(None);
        }
        let r = crate::command::command(&self.inner, STLINK_GET_TARGET_VOLTAGE, 0, 8)?;
        let adc_result = [
            u32::from_le_bytes([r[0], r[1], r[2], r[3]]),
            u32::from_le_bytes([r[4], r[5], r[6], r[7]]),
        ];
        Ok(Some(if adc_result[0] != 0 {
            2.0 * (adc_result[1] as f32) * 1.2 / (adc_result[0] as f32)
        } else { 
            0.0 
        } ))
    }

    pub fn get_mode(&self) -> io::Result<u8> {
        let r = crate::command::command(&self.inner, STLINK_GET_CURRENT_MODE, 0, 2)?;
        Ok(r[0])
    }
}

// //todo: bug?
// impl Drop for Handle<'_> {
//     fn drop(&mut self) {
//         crate::command::command(&self.inner, STLINK_DEBUG_APIV2_RESETSYS, 0x80, 2).unwrap();
//     }
// }

impl<'h> AsRef<nihao_usb::Handle<'h>> for Handle<'h> {
    fn as_ref(&self) -> &nihao_usb::Handle<'h> {
        &self.inner
    }
}

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
        // get version
        let version = match crate::version::read_handle(&src) {
            Ok(ver) => ver,
            Err(err) => return Err((src, err.into())),
        };
        let ans = Handle { inner: src, version };
        Ok(ans)
    }
}
