use std::io;

fn main() -> io::Result<()> {
    for device in nihao_usb::devices()? {
        println!("{:?}", device);
    }
    Ok(())
}