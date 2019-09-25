use std::io;
use std::convert::TryFrom;

fn main() -> io::Result<()> {
    for device in nihao_usb::devices()?.iter() {
        if let Ok(usb_handle) = device?.open() {
            // println!("{:?}", usb_handle.device_descriptor());
            let stlink_handle = nihao_stlink::Handle::try_from(usb_handle)
                .expect("open stlink handle");
            println!("{:?}", stlink_handle);
        }
    }
    Ok(())
}
