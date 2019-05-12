use std::io;

fn main() -> io::Result<()> {
    for device in nihao_usb::devices()? {
        // println!("{:?}", device);
        if let Ok(handle) = device?.open() {
            println!("{:?}", handle.device_descriptor());
        }
    }
    Ok(())
}
