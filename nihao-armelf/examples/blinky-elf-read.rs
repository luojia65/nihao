use std::{
    fs::OpenOptions,
    io::{Read, Seek, SeekFrom},
};

const MAGIC: [u8; 4] = [0x7f, 0x45, 0x4c, 0x46];

#[derive(Clone, Copy, Debug)]
#[repr(C)]
struct EHead {
    ei_magic: [u8; 4],
    ei_class: u8,
    ei_data: u8,
    ei_version: u8,
    ei_osabi: u8,
    ei_abiversion: u8,
    _padding: [u8; 7],
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: u32,
    e_phoff: u32,
    e_shoff: u32,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}

union EHeadConv {
    array: [u8; 52],
    result: EHead,
}

impl From<[u8; 52]> for EHead {
    fn from(src: [u8; 52]) -> EHead {
        unsafe { EHeadConv { array: src }.result }
    }
}

impl<'a> From<&'a [u8; 52]> for &'a EHead {
    fn from(src: &'a [u8; 52]) -> &'a EHead {
        unsafe { &*(src as *const _ as *const EHead) }
    }
}

fn main() -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(false)
        .create_new(false)
        .open("./nihao-armelf/examples/blinky")?;
    let mut buf = [0u8; 52];
    file.seek(SeekFrom::Start(0))?;
    file.read_exact(&mut buf)?;
    let head = EHead::from(buf);
    println!("{:?}", head);
    assert_eq!(head.ei_magic, MAGIC);
    Ok(())
}
