use evdev::Device;
fn main() {
    println!("Hello, world!");
    print_keys();
}



fn print_keys() {
    let mut device = match Device::open("/dev/packingkeyboard") {
        Ok(d) => d,
        Err(err) => {
            eprintln!("{}", err);
            return;
        },
    };
    

    let _ = device.grab();

    let keys = match device.get_key_state() {
        Ok(k) => k,
        Err(err) => {
            eprintln!("{}", err);
            let _ = device.ungrab();
            return;
        },
    };

    println!("{:#?}", keys);
    let _ = device.ungrab();
}