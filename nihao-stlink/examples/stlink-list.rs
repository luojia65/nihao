use std::io;

fn main() -> io::Result<()> {
    for handle in nihao_stlink::handles()? {
        println!("{:?}", handle?.into_inner().device_descriptor());
    }
    Ok(())
}
