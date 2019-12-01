use std::io;

fn main() -> io::Result<()> {
    for handle in nihao_stlink::handles()? {
        // println!("Handle: {:?}", handle);
        match handle {
            Ok(handle) => {
                println!("Desc: {:?}", handle.as_ref().device_descriptor());
                println!("{}", handle.version());
            },
            Err(e) => println!("Error: {:?}", e)
        }
    }
    println!("Finished");
    Ok(())
}
