pub mod sys;

use core::iter::FusedIterator;

use std::io;

/// Get an `Iterator` over all USB devices identified by your operating system.
/// 
/// Note that the return value for this iterator is a `Result`.
/// You may need to use a try operator `?` after the function call `devices()`
/// if you want to iterate everything in it by using `for` statements. 
/// That's because a `Result` is also an `Iterator`, and its `Item` is `Devices`
/// other than `Device` expected.
pub fn devices<'list>() -> io::Result<DeviceList<'list>> {
    sys::devices().map(|inner| DeviceList { inner })
}

#[derive(Debug, Clone)]
pub struct DeviceList<'list> {
    inner: sys::DeviceList<'list>
}

impl<'list> DeviceList<'list> {
    pub fn iter<'iter>(&self) -> Devices<'iter> {
        Devices { inner: self.inner.iter() }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    } 
}

// impl<'list> IntoIterator for DeviceList<'list> {
//     type Item = <device::IntoIter<'list> as Iterator>::Item;
//     type IntoIter = device::IntoIter<'list>;

//     fn into_iter(self) -> device::IntoIter<'list> {
//         self.iter() // todo
//     }
// }

/// An `Iterator` for USB devices.
#[derive(Debug, Clone)]
pub struct Devices<'iter> {
    inner: sys::Devices<'iter>,
}

impl<'iter> Iterator for Devices<'iter> {
    type Item = io::Result<Device<'iter>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|res| res.map(|inner| Device { inner }))
    }
}

impl FusedIterator for Devices<'_> {}

/// A path struct representing a certain USB device connected to underlying OS.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Device<'device> {
    inner: sys::Device<'device>,
}

impl<'device> Device<'device> {
    pub fn open<'handle>(&self) -> io::Result<Handle<'handle>> {
        self.inner.open().map(|inner| Handle { inner })
    }
}

/// A connection handle to the remote device.
/// 
/// Underlying code must ensure that this handle implements `Drop` and all relevant
/// resources are freed during their `drop` operations.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Handle<'handle> {
    inner: sys::Handle<'handle>,
}

impl<'handle> Handle<'handle> {
    pub fn device_descriptor(&self) -> io::Result<DeviceDescriptor> {
        self.inner.device_descriptor()
    }

    pub fn speed(&self) -> io::Result<crate::Speed>  {
        self.inner.speed()
    }
}

/// A `DeviceDescriptor` describing what this name represents in the USB specification
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct DeviceDescriptor {
    pub length: u8,
    pub descriptor_type: u8,
    pub bcd_usb: u16,
    pub device_class: u8,
    pub device_sub_class: u8,
    pub device_protocol: u8,
    pub max_packet_size_0: u8,
    pub id_vendor: u16,
    pub id_product: u16,
    pub bcd_device: u16,
    pub manufacturer: u8,
    pub product: u8,
    pub serial_number: u8,
    pub num_configurations: u8,
}

/// An `InterfaceDescriptor` describing what this name represents in the USB specification
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct InterfaceDescriptor {
    pub length: u8,
    pub descriptor_type: u8, 
    pub interface_number: u8,
    pub alternate_setting: u8,
    pub num_endpoints: u8,
    pub interface_class: u8,
    pub interface_subclass: u8,
    pub interface_protocol: u8,
    pub index_interface: u8,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)] 
pub enum Speed {
    Unknown,
    Low,
    Full,
    High,
    Super,
}
