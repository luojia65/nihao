use nihao_usb::sys::windows::setup;

fn main() {
    let device_info_set = 
        setup::GetOptions::all_interfaces().present().get();
    println!("{:?}", device_info_set);
}
