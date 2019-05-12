use nihao_usb::sys::windows::{setup::ListOptions, usb::ListOptionsExt};
use std::io;

fn main() -> io::Result<()> {
    let info_handle = ListOptions::all_usb_interfaces()
        .present()
        .list()?;
    info_handle.iter()
        .filter_map(|info| info.ok())
        .map(|info| info.open())
        .filter_map(|usb| usb.ok())
        .for_each(|usb| println!("{:?}", usb.device_descriptor()));
    Ok(())
}
