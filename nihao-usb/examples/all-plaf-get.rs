use std::io;

fn main() -> io::Result<()> {
    let mut i = 0;
    for device in nihao_usb::devices()? {
        i += 1;
        println!("= [{}] Device: {:?}", i, device);
        let device = device?;
        println!("{:?}", device.open());
        if let Ok(handle) = device.open() {
            println!("{:?}", handle.device_descriptor());
            println!("{:?}", handle.speed());
        }
    }
    println!("Count of devices: {}", i);
    Ok(())
}
