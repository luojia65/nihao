use nihao_usb::sys::windows::{setup::GetOptions, usb::GUID_DEVINTERFACE_USB_DEVICE};

fn main() {
    let device_info_set = GetOptions::interface_by_class(&GUID_DEVINTERFACE_USB_DEVICE)
        .present()
        .get()
        .unwrap();
    for ans in device_info_set.iter(&GUID_DEVINTERFACE_USB_DEVICE) {
        println!("{:?}", ans);
    }
}
