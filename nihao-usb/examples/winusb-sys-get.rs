use std::io;

fn main() -> io::Result<()> {
    for device in nihao_usb::sys::windows::devices()?.iter() {
        if let Ok(handle) = device?.open() {
            let buf_send = [0xF1u8, 0x80];
            let mut buf_recv = vec![0u8; 1024];
            handle.write_pipe(0x02, &buf_send)?;
            let len = handle.read_pipe(0x81, &mut buf_recv)?;
            println!("{:?}", &buf_recv[..len]);
        }
    }
    Ok(())
}
