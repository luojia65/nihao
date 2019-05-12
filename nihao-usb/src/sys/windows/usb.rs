use core::ptr::NonNull;
use super::setup;
use winapi::{
    um::winusb::WINUSB_INTERFACE_HANDLE,
};

pub use winapi::shared::usbiodef::GUID_DEVINTERFACE_USB_DEVICE;

pub struct InterfaceHandle {
    handle: NonNull<()>,
}

pub trait ListOptionsExt {
    fn all_usb_interfaces() -> setup::ListOptions<setup::Interface>;
}

impl ListOptionsExt for setup::ListOptions<setup::Interface> {
    fn all_usb_interfaces() -> setup::ListOptions<setup::Interface> {
        setup::ListOptions::interface_by_class(&GUID_DEVINTERFACE_USB_DEVICE)
    }
}
