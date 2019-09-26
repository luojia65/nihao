use nihao_usb::sys::windows::{setup::ListOptions, usb::ListOptionsExt};
use nihao_usb::{DeviceDescriptor, InterfaceDescriptor};
use std::io;
use core::task::Poll;

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
                println!("= Write!");
                let ov_write = usb.write_pipe_overlapped(0x02, &buf_send).unwrap();
                println!("= Read!");
                let ov_read = usb.read_pipe_overlapped(0x81, &mut buf_recv).unwrap();
                println!("= Poll write!");
                while let Poll::Pending = usb.poll_overlapped(ov_write.as_ref()) {}
                // while let Poll::Pending = usb.poll_overlapped(&ov_read) {}
                println!("= Poll read!");
                loop {
                    if let Poll::Ready(len) = usb.poll_overlapped(ov_read.as_ref()) {
                        let len = len.unwrap();
                        println!("Bytes read: {:?}", len);
                        println!("{:?}", &buf_recv[..len]);
                        break;
                    }
                } 
                println!("== Finished");
            }
        }
    }
    Ok(())
}
