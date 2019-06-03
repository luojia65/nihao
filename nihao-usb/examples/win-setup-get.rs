use nihao_usb::sys::windows::{setup::ListOptions, usb::ListOptionsExt};
use nihao_usb::DeviceDescriptor;
use std::io;

fn main() -> io::Result<()> {
    let info_handle = ListOptions::all_usb_interfaces()
        .present()
        .list()?;
    for info in info_handle.iter() {
        if let Ok(info) = info {
            if let Ok(usb) = info.open() {
                println!("{:?}", usb.device_descriptor().map(|d| DeviceDescriptor::from(d)));
                println!("Speed: {:?}", usb.speed());
            }
        }
    }
    Ok(())
}
