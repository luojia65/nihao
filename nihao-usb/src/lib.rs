pub mod sys;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct DeviceDescriptor {
    length: u8,
    descriptor_type: u8,
    bcd_usb: u16,
    device_class: u8,
    device_sub_class: u8,
    device_protocol: u8,
    max_packet_size_0: u8,
    id_vendor: u16,
    id_product: u16,
    bcd_device: u16,
    manufacturer: u8,
    product: u8,
    serial_number: u8,
    num_configurations: u8,
}
