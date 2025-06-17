use evdev::{Device, EventSummary, KeyCode};
use tokio::{
    signal,
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
};
use tokio_util::sync::CancellationToken;
#[derive(Debug)]
enum KeyMessage {
    Key(KeyCode),
}

#[tokio::main]
async fn main() -> Result<(), tokio::io::Error> {
    println!("Listening for keyboard input...");

    let token = CancellationToken::new();

    let (key_sender, key_reciever) = tokio::sync::mpsc::unbounded_channel::<KeyMessage>();

    let key_thread = tokio::spawn(keyboard_thread(key_sender.clone(), token.clone()));
    let media_thread = tokio::spawn(media_key_thread(key_sender.clone(), token.clone()));
    let print_thread = tokio::spawn(print_keys(token.clone(), key_reciever));

    match signal::ctrl_c().await {
        Ok(_) => {
            token.cancel();
            print_thread.await?;
            media_thread.await?;
            key_thread.await?;
        }
        Err(err) => {
            eprintln!("{}", err);
            return Err(err);
        }
    }

    Ok(())
}

async fn keyboard_thread(sender: UnboundedSender<KeyMessage>, token: CancellationToken) {
    let device_id_kbd = "/dev/input/event0";
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
        }
    };
    loop {
        let event = match events.next_event().await {
            Ok(e) => e,
            Err(err) => {
                eprintln!("{}", err);
                let _ = events.device_mut().ungrab();
                return;
            }
        };

        match event.destructure() {
            EventSummary::Key(_, code, value) => {
                if code != KeyCode::KEY_NUMLOCK {
                    if value == 1 {
                        sender.send(KeyMessage::Key(code)).unwrap();
                    }
                }
            }
            _ => {}
        };

        match token.is_cancelled() {
            true => break,
            false => {}
        }
    }
    let _ = events.device_mut().ungrab();
}

async fn media_key_thread(sender: UnboundedSender<KeyMessage>, token: CancellationToken) {
    let device_id_media_keys = "/dev/input/event1";
    println!("Listening to device: {}", device_id_media_keys);
    let mut device_media = match Device::open(device_id_media_keys) {
        Ok(d) => d,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };
    match device_media.grab() {
        Ok(_) => {}
        Err(err) => {
            eprintln!("device_media: {}", err);
            return;
        }
    };
    let mut events = match device_media.into_event_stream() {
        Ok(e) => e,
        Err(err) => {
            eprintln!("device_media: {}", err);
            return;
        }
    };
    loop {
        let event = match events.next_event().await {
            Ok(e) => e,
            Err(err) => {
                eprintln!("{}", err);
                let _ = events.device_mut().ungrab();
                return;
            }
        };

        match event.destructure() {
            EventSummary::Key(_, code, value) => {
                if code != KeyCode::KEY_NUMLOCK {
                    if value == 1 {
                        sender.send(KeyMessage::Key(code)).unwrap();
                    }
                }
            }
            _ => {}
        };

        match token.is_cancelled() {
            true => break,
            false => {}
        }
    }
    let _ = events.device_mut().ungrab();
}

async fn print_keys(token: CancellationToken, mut key_reciever: UnboundedReceiver<KeyMessage>) {
    while !token.is_cancelled() {
        let message = match key_reciever.recv().await {
            Some(m) => m,
            None => {
                return;
            }
        };
        match message {
            KeyMessage::Key(key_code) => {
                println!("{:?}", key_code)
            }
        }
    }
}
