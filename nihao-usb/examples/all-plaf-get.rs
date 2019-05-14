use std::io;

fn main() -> io::Result<()> {
    for device in nihao_usb::devices()? {
        if let Ok(handle) = device?.open() {
            println!("begin");
            std::thread::sleep(std::time::Duration::from_secs(2));
            println!("{:?}", handle.device_descriptor());
        }
    }
    Ok(())
}
