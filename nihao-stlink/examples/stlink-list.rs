use std::io;

fn main() -> io::Result<()> {
    for handle in nihao_stlink::handles()? {
        println!("Handle: {:?}", handle);
        if let Ok(handle) = handle {
            println!("Desc: {:?}", handle.into_inner().device_descriptor());
        }
    }
    // for device in nihao_usb::devices()?.iter() {
    //     // println!("Detected device: {:?}", device);
    //     if let Ok(usb_handle) = device?.open() {
    //         println!("{:?}", usb_handle.device_descriptor());
    //         // println!("{:?}", handle.speed());
    //         let stlink_handle = nihao_stlink::Handle::try_from(usb_handle)
    //             .expect("open stlink handle");
    //         println!("{:?}", stlink_handle);
    //     }
    // }
    Ok(())
}
