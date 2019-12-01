pub mod handle;
pub mod version;
pub mod consts;
pub mod command;

pub use handle::Handle;

use core::iter::FusedIterator;
use std::io;

pub fn handles<'iter>() -> io::Result<HandleList<'iter>> {
    nihao_usb::devices().map(|inner| HandleList { inner })
}

#[derive(Debug, Clone)]
pub struct HandleList<'list> {
    inner: nihao_usb::DeviceList<'list>,
}

impl<'list> HandleList<'list> {
    pub fn iter<'iter>(&self) -> Handles<'iter> {
        Handles { inner: self.inner.iter() }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    } 
}

/// An owned iterator for USB devices.
#[derive(Debug, Clone)]
pub struct HandleIntoIter<'iter> {
    inner: nihao_usb::DeviceIntoIter<'iter>,
}

impl<'list> IntoIterator for HandleList<'list> {
    type Item = io::Result<Handle<'list>>;
    type IntoIter = HandleIntoIter<'list>;

    fn into_iter(self) -> Self::IntoIter {
        HandleIntoIter { inner: self.inner.into_iter() }
    }
}

impl<'iter> Iterator for HandleIntoIter<'iter> {
    type Item = io::Result<Handle<'iter>>;

    fn next(&mut self) -> Option<Self::Item> {
        use handle::TryFromHandleError::*;
        use core::convert::TryFrom;
        
        while let Some(Ok(usb_device)) = self.inner.next() {
            let h = if let Ok(h) = usb_device.open() { h } else { continue };
            match Handle::try_from(h) {
                Ok(h) => return Some(Ok(h)),
                Err((_h, IoError(e))) => return Some(Err(e)),
                Err(_) => continue,
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct Handles<'iter> {
    inner: nihao_usb::Devices<'iter>,
}

impl<'iter> Iterator for Handles<'iter> {
    type Item = io::Result<Handle<'iter>>;

    fn next(&mut self) -> Option<Self::Item> {
        use handle::TryFromHandleError::*;
        use core::convert::TryFrom;
        
        while let Some(Ok(usb_device)) = self.inner.next() {
            // unable to open, continue
            let h = if let Ok(h) = usb_device.open() { h } else { continue };
            match Handle::try_from(h) {
                Ok(h) => return Some(Ok(h)),
                Err((_h, IoError(e))) => return Some(Err(e)),
                Err(_) => continue,
            }
        }
        None
    }
}

impl FusedIterator for Handles<'_> {}
