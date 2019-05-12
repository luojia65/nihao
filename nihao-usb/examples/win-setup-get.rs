use nihao_usb::sys::windows::{setup::ListOptions, usb::ListOptionsExt};
use std::io;

fn main() -> io::Result<()> {
    let info_handle = ListOptions::all_usb_interfaces()
        .present()
        .list()?;
    for info in info_handle.iter() {
        let info = info?;
        println!("info: {:?}", info);
        let usb = info.open();
        println!("{:?}", usb);
    }
    Ok(())
}
