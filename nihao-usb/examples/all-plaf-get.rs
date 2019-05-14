use std::io;

fn main() -> io::Result<()> {
    for device in nihao_usb::devices()? {
        if let Ok(handle) = device?.open() {
            println!("{:?}", handle.device_descriptor());
        }
    }
    Ok(())
}
