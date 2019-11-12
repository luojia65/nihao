use core::result;
use core::iter;
use core::time::Duration;

// host context. usually a pc, mac or embedded chips with usb otg.
pub trait Host {
    // owned ref to device path or repr struct
    // usually being able to clone
    // maybe a path string struct on windows & linux
    type Device;
    
    // owned ref to a list containing all paths
    type Devices: iter::Iterator<
        Item = result::Result<Self::Device, Self::Error>,
    >;

    // owned ref to a list containing all paths
    type DeviceList: iter::IntoIterator<
        Item = result::Result<Self::Device, Self::Error>,
        IntoIter = Self::Devices,
    >;

    // error that may occur
    type Error;

    // get all devices available on this setup context
    // this is alike 
    fn available() -> result::Result<Self::DeviceList, Self::Error>;
}

pub trait Input {
    // owned ref to device handle for reading
    // maybe a winusb handle on windows, file handle on linux
    // how devices turn into handles remain to designs of device structs 
    type Handle;

    // error that may occur
    type Error;
    
    fn read_control(
        &self,
        request_type: u8,
        request: u8,
        value: u16,
        index: u16,
        buf: &mut [u8],
        timeout: Duration
    ) -> result::Result<usize, Self::Error>;

    fn read_interrupt(
        &self, 
        endpoint: u8, 
        buf: &mut [u8], 
        timeout: Duration
    ) -> result::Result<usize, Self::Error>;

    fn read_bulk(
        &self,
        endpoint: u8,
        buf: &mut [u8],
        timeout: Duration
    ) -> result::Result<usize, Self::Error>;
}

// pub trait AsyncInput: Input {
    // async fn read_bulk(
    //     &self,
    //     endpoint: u8,
    //     buf: &mut [u8],
    //     timeout: Duration
    // ) -> result::Result<usize, Self::Error>;
// }

pub trait Output {
    // owned ref to device handle for writing
    type Handle;

    // error that may occur
    type Error;

    fn write_control(
        &self,
        request_type: u8,
        request: u8,
        value: u16,
        index: u16,
        buf: &[u8],
        timeout: Duration
    ) -> result::Result<usize, Self::Error>;

    fn write_interrupt(
        &self,
        endpoint: u8,
        buf: &[u8],
        timeout: Duration
    ) -> result::Result<usize, Self::Error>;

    fn write_bulk(
        &self,
        endpoint: u8,
        buf: &[u8],
        timeout: Duration
    ) -> result::Result<usize, Self::Error>;
}

// pub trait AsyncOutput: Output {

// }
