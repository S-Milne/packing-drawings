use std::time::Duration;

use evdev::Device;
use tokio::{signal, time::sleep};
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() -> Result<(), tokio::io::Error> {
    println!("Hello, world!");

    let token = CancellationToken::new();

    let print_thread = tokio::spawn(print_keys(token.clone()));

    match signal::ctrl_c().await {
        Ok(_) => {
            token.cancel();
            print_thread.await?;
        },
        Err(err) => {
            eprintln!("{}", err);
            return Err(err);
        },
    }

    Ok(())
    
}

async fn print_keys(token: CancellationToken) {
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
        sleep(Duration::from_secs(1)).await;
        match token.is_cancelled() {
            true => break,
            false => {},
        }
    }

    let _ = device.ungrab();
}
