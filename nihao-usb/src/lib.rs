pub mod backend;

#[cfg(not(no_std))]
pub mod sys;
#[cfg(not(no_std))]
pub use sys::{devices, Error, Result, List, IntoIter, Device, Handle};

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
