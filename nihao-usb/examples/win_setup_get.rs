use nihao_usb::sys::windows::{setup::GetOptions, usb::GUID_DEVINTERFACE_USB_DEVICE};

fn main() {
    let device_info_set = GetOptions::device_by_class(&GUID_DEVINTERFACE_USB_DEVICE)
        .present()
        .get();
    println!("{:?}", device_info_set);
}
