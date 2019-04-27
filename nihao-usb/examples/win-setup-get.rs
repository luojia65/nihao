use nihao_usb::sys::windows::{setup::ListOptions, usb::GUID_DEVINTERFACE_USB_DEVICE};
use std::io;

fn main() -> io::Result<()> {
    let device_info_set = ListOptions::interface_by_class(&GUID_DEVINTERFACE_USB_DEVICE)
        .present()
        .list()?;
    for ans in device_info_set.iter(&GUID_DEVINTERFACE_USB_DEVICE) {
        println!("{:?}", ans?);
    }
    Ok(())
}
