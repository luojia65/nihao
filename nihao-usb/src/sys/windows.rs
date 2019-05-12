pub mod setup;
pub mod usb;

use std::io;

pub fn devices<'iter>() -> io::Result<Devices<'iter>> {
    use usb::ListOptionsExt;
    let handle = setup::ListOptions::all_usb_interfaces()
        .present()
        .list()?;
    Ok(Devices { handle, iter: None })
}

#[derive(Debug, Clone)]//, Hash, Eq, PartialEq)]
pub struct Devices<'iter> {
    handle: usb::InfoHandle,
    iter: Option<usb::InfoIter<'iter>>,
}

impl<'iter> Iterator for Devices<'iter> {
    type Item = io::Result<Device<'iter>>;
    
    fn next(&mut self) -> Option<Self::Item> {
        if let None = &self.iter {
            self.iter = Some(self.handle.iter());
        };
        if let Some(iter) = &mut self.iter {
            iter.next().map(|res| res.map(|info| Device { info }))
        } else { unreachable!() }
    }
}

#[derive(Debug, Clone)]
pub struct Device<'device> {
    info: usb::Info<'device>,
}

// impl<'device> Device<'device> {
//     pub fn connect(&self) -> io::Result<Stream> {
//         self.info.open()
//     }
// }
