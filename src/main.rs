

use evdev::{Device, EventType};
use tokio::signal;
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() -> Result<(), tokio::io::Error> {
    println!("Listening for keyboard input...");

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
    println!("Listening to device: {}", device_id_kbd);
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

    let mut events = match device_kbd.into_event_stream() {
        Ok(e) => e,
        Err(err) => {
            eprintln!("device_kbd: {}", err);
            return;
        },
    };

    loop {

        let event = match events.next_event().await {
            Ok(e) => e,
            Err(err) => {
                eprintln!("{}", err);
                let _ = events.device_mut().ungrab();
                return;
            },
        };
        

        let event_type = event.event_type();

        match event_type {
            EventType::KEY => {
                let pressed = match event.value() {
                    0 => "Released",
                    1 => "Pressed",
                    _ => "Not a key"
                };
                println!("Code: {:?} {}", event.code(), pressed);
            }
            _ => {

            }
        }

        

        match token.is_cancelled() {
            true => break,
            false => {}
        }
    }

    let _ = events.device_mut().ungrab();
}
