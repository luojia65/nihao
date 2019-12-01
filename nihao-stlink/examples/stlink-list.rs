use std::io;

fn main() -> io::Result<()> {
    for handle in nihao_stlink::handles()? {
        println!("Handle: {:?}", handle);
        match handle {
            Ok(handle) => println!("Desc: {:?}", handle.into_inner().device_descriptor()),
            Err(e) => println!("Error: {:?}", e)
        }
    }
    println!("Finished");
    Ok(())
}
