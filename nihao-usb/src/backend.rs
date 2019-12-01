#[cfg(any(windows, doc))]
pub mod winusb;

use core::iter;
use core::time::Duration;

// host context. usually a pc, mac or embedded chips with usb otg.
pub trait Host<'lt> {
    // owned buffer to device path or repr struct
    // usually being able to clone
    // maybe a path string struct on windows & linux
    type Device: 'lt;    

    // owned list containing all paths
    type List: iter::IntoIterator<
        Item = Result<Self::Device, Self::Error>,
        IntoIter = Self::IntoIter,
    >;

    // ref to an iterator containing all paths
    type Iter: iter::Iterator<
        Item = Result<&'lt Self::Device, Self::Error>,
    >;

    // owned iterator containing all paths
    type IntoIter: iter::Iterator<
        Item = Result<Self::Device, Self::Error>,
    >;
    
    // error that may occur
    type Error;

    // get all devices available on this setup context
    // this is alike 
    fn available() -> Result<Self::List, Self::Error>;

    // owned ref to device handle for reading
    // maybe a winusb handle on windows, file handle on linux
    // how devices turn into handles remain to designs of device structs 
    type Handle;

    // open detected device into a handle
    fn open(device: Self::Device) -> Result<Self::Handle, Self::Error>;
}

pub trait Input {
    // error that may occur
    type Error;
    
    fn read_control(
        &mut self,
        request_type: u8,
        request: u8,
        value: u16,
        index: u16,
        buf: &mut [u8],
        timeout: Duration
    ) -> Result<usize, Self::Error>;

    fn read_interrupt(
        &mut self, 
        endpoint: u8, 
        buf: &mut [u8], 
        timeout: Duration
    ) -> Result<usize, Self::Error>;

    fn read_bulk(
        &mut self,
        endpoint: u8,
        buf: &mut [u8],
        timeout: Duration
    ) -> Result<usize, Self::Error>;
}

// pub trait AsyncInput: Input {
    // async fn read_bulk(
    //     &self,
    //     endpoint: u8,
    //     buf: &mut [u8],
    //     timeout: Duration
    // ) -> Result<usize, Self::Error>;
// }

pub trait Output {
    // error that may occur
    type Error;

    fn write_control(
        &mut self,
        request_type: u8,
        request: u8,
        value: u16,
        index: u16,
        buf: &[u8],
        timeout: Duration
    ) -> Result<usize, Self::Error>;

    fn write_interrupt(
        &mut self,
        endpoint: u8,
        buf: &[u8],
        timeout: Duration
    ) -> Result<usize, Self::Error>;

    fn write_bulk(
        &mut self,
        endpoint: u8,
        buf: &[u8],
        timeout: Duration
    ) -> Result<usize, Self::Error>;
}

// pub trait AsyncOutput: Output {

// }
