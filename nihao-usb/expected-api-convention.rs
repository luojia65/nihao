// get an iterator if an operating system is present
pub fn devices<'a>(&'a self) 
    -> io::Result<Devices<'a>> { ... }

// contains setup search handle, current index 
// as well as a pointer to the shared buffer 
pub struct Devices<'a> { ... }  

// close search handle and release buffer
impl Drop for Devices { ... }

impl<'a> Iterator for Devices<'a> { 
    type Item = io::Result<&'a DeviceSlot>;
    ...
}

// contains usb device information handle, information 
// and address; the connection is not actually opened 
pub struct DeviceSlot { ... }

impl DeviceSlot {
    pub fn device_descriptor(&self) 
        -> io::Result<DeviceDescriptor> { ... }
    pub fn config_descriptor(&self) 
        -> io::Result<ConfigDescriptor> { ... }

    pub fn open(&self) -> io::Result<Device> { ... }
}

// contains a device handle which is opened
pub struct Device { ... }

// closes this device
impl Drop for Device { ... }

impl Device {
    pub fn reset(&mut self) -> io::Result<()> { ... }
    // no close function here! just `drop(device)`.
}

/* =-=-=-=-= descriptors =-=-=-=-= */

pub struct VendorId(u16);
pub struct ProductId(u16);

pub struct EndpointAddr(u8);

// follow usb-standard descriptor
pub struct DeviceDescriptor { ... }

impl DeviceDescriptor { 
    fn vendor_id(&self) -> VendorId { ... }
    fn product_id(&self) -> ProductId { ... }
}

pub struct ConfigDescriptor { ... }

impl ConfigDescriptor { ... }

pub struct InterfaceDescriptor { ... }

impl InterfaceDescriptor { ... }

pub struct EndpointDescriptor { ... }

impl EndpointDescriptor { ... }

/* =-=-=-=-= example of main function =-=-=-=-= */

// example: list all devices and pipes
for device in usb::devices()? {
    println!("Device:\t{:?}", device);
    let mut handle = device.open()?;
    for pipe in handle.pipes()? {
        println!("Pipe:\t{:?}", pipe);
    }
}

// example: count all st-link's with proper voltage (sync)
let mut buf = vec![0u8; 4096];
let cnt = usb::devices()?.filter_vid_pid(0x0483, 0x3748)
    .filter_map(|device| device.open().map(|handle| {
        let handle = handle?;
        block_on!(|| {
            handle.write_pipe(0x02, &[0xF7, 0x00, 0x00, 0x00])?;
            handle.read_pipe(&mut buf)?;
        });
        let r1 = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
        let r2 = u32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]);
        Ok(8 * r2 > 5 * r1) // okay := 2.4*r2/r1 > 1.5 => r2 / r1 > 5 / 8
    }).ok())
    .fold(0, |prev, okay| prev + if okay { 1 } else { 0 });

// example: count all st-link's with proper voltage
// in an advanced application interface
let mut buf = vec![0u8; 4096];
let cnt = stlink::devices()?.filter_map(|device| device.open().map(|handle| {
    let volt: f32 = block_on!(handle?.query_voltage()?).into();
    Ok(volt > 1.5)
}).ok())
.fold(0, |prev, okay| prev + if okay { 1 } else { 0 });

// example: debug print all st-link dongles connected
stlink::devices()?.for_each(|dev| println!("{:?}", dev));

// example: erase all connected stm32f103 chips via st-link
// count all successes and fails
let (successes, fails) = stlink::devices()?
    .map(|device| device.open().map(|handle| {
        block_on!(handle?.erase_all()).is_ok()
    }).ok())
    .fold((0, 0), |(success, fail), ok| 
        if ok { (success + 1, fail) } else { (success, fail + 1)}
    );

// example: flash one file onto one chip
use nihao_stm32::DevicesExt;
let source = include!("my_program");
let chip = nihao::devices()?.filter_stm32f103().next()
    .expect("have a device plugged in")?;
block_on!(nihao::Flash::new()
    .erase_all()
    .program_entire(source)
    .verify(nihao_stm32::verify::CRC)
    .flash(chip));

// example: read bluetooth mac address from cc2640r2f
use nihao_simplelink::DevicesExt;
let addr = block_on!(nihao::devices().filter_cc2640r2f().next()
    .expect("have a device plugged in")?
    .query_primary_ble_mac_addr()?
);

// example: erase all chips connected via all supported
// flashers and programmers
nihao::devices()?.for_each(|device| 
    block_on!(device?.open()?.erase_all()?));

// example: choose chip at runtime
//todo
