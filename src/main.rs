use evdev::Device;
fn main() {
    println!("Hello, world!");
    print_keys();
}

fn print_keys() {
    let device_id = "/dev/input/by-id/usb-SIGMACHIP_USB_Keyboard-event-kbd";
    #[allow(unused)]
    let device_redirect = "/dev/packingkeyboard";
    let mut device = match Device::open(device_id) {
        Ok(d) => d,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    let _ = device.grab();

    loop {
        let keys = match device.get_key_state() {
            Ok(k) => k,
            Err(err) => {
                eprintln!("{}", err);
                let _ = device.ungrab();
                return;
            }
        };
        println!("{:#?}", keys);
    }
    
    let _ = device.ungrab();
}
