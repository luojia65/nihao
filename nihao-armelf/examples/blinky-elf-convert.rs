use std::fs::OpenOptions;

fn main() -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(false)
        .create_new(false)
        .open("./nihao-armelf/examples/blinky")?;
    Ok(())
}
