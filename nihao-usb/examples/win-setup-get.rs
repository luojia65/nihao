use nihao_usb::sys::windows::{setup::ListOptions, usb::{ListOptionsExt, *}};
use std::io;

fn main() -> io::Result<()> {
    let info_handle = ListOptions::all_usb_interfaces()
        .present()
        .list()?;
    for ans in info_handle.iter(&GUID_DEVINTERFACE_USB_DEVICE) {
        println!("{:?}", ans);
    }
    Ok(())
}
