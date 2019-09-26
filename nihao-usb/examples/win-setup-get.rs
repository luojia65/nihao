use nihao_usb::sys::windows::{setup::ListOptions, usb::ListOptionsExt};
use nihao_usb::{DeviceDescriptor, InterfaceDescriptor};
use std::io;

fn main() -> io::Result<()> {
    let info_handle = ListOptions::all_usb_interfaces()
        .present()
        .list()?;
    for info in info_handle.iter() {
        if let Ok(info) = info {
            if let Ok(usb) = info.open() {
                println!("=1= Device: {:?}", usb.device_descriptor()
                    .map(|d| DeviceDescriptor::from(d)));
                println!("=2= Speed: {:?}", usb.speed());
                println!("=3= Interface 0: {:?}", usb.interface_settings(0).unwrap()
                    .map(|d| InterfaceDescriptor::from(d)));
                println!("=4= Has pipe 2:{}", usb.query_pipe(0, 2).expect("query pipe").is_some());
                let buf_send: &[u8] = &[
                    0xF1, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
                ];
                let mut buf_recv = vec![0u8; 1024];
                let _ov_read = usb.read_pipe_overlapped(0x81, &mut buf_recv).unwrap();
                let _ov_write = usb.write_pipe_overlapped(0x02, &buf_send).unwrap();
                std::thread::sleep(std::time::Duration::from_secs(1));
                println!("{:?}", &buf_recv[..16]);
                println!("== Finished");
            }
        }
    }
    Ok(())
}
