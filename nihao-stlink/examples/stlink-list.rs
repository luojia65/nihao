use std::io;

fn main() -> io::Result<()> {
    for handle in nihao_stlink::handles()? {
        // println!("Handle: {:?}", handle);
        match handle {
            Ok(handle) => {
                println!("Desc: {:?}", handle.as_ref().device_descriptor());
                print_one(&handle);
            },
            Err(e) => println!("Error: {:?}", e)
        }
    }
    println!("Finished");
    Ok(())
}

fn print_one(handle: &nihao_stlink::Handle) {
    println!("{}", handle.version());
    if let Ok(Some(voltage)) = handle.get_voltage() {
        println!("Target voltage: {:?}", voltage)
    }
}
