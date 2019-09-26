use std::io;

fn main() -> io::Result<()> {
    for handle in nihao_stlink::handles()? {
        println!("Handle: {:?}", handle);
        if let Ok(handle) = handle {
            println!("Desc: {:?}", handle.into_inner().device_descriptor());
        }
    }
    println!("Finished");
    Ok(())
}
