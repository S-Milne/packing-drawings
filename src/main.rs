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
        }
        Err(err) => {
            eprintln!("{}", err);
            return Err(err);
        }
    }

    Ok(())
}

async fn print_keys(token: CancellationToken) {
    let device_id_kbd = "/dev/input/by-id/usb-SIGMACHIP_USB_Keyboard-event-kbd";

    let mut device_kbd = match Device::open(device_id_kbd) {
        Ok(d) => d,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    match device_kbd.grab() {
        Ok(_) => {}
        Err(err) => {
            eprintln!("device_kbd: {}", err);
            return;
        }
    };

    loop {
        let kbd_keys = match device_kbd.get_key_state() {
            Ok(k) => k,
            Err(err) => {
                eprintln!("device_kbd: {}", err);
                let _ = device_kbd.ungrab();
                return;
            }
        };

        println!("device_kbd: {:?}", kbd_keys);

        sleep(Duration::from_secs(1)).await;
        match token.is_cancelled() {
            true => break,
            false => {}
        }
    }

    let _ = device_kbd.ungrab();
}
