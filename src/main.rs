use std::{thread::sleep, time::Duration, thread  };
use rdev::{EventType,Event};
use rdev::{simulate, SimulateError};

fn main() {
    let callback = |event: Event| {
        if let EventType::KeyPress(key) = event.event_type {
            if key == rdev::Key::Alt {
                println!("space pressed!");
                send(&EventType::KeyRelease(rdev::Key::Space));
                send(&EventType::KeyPress(rdev::Key::Space));
                send(&EventType::KeyRelease(rdev::Key::Space));
            }
            if key == rdev::Key::KeyR {
                send(&EventType::KeyRelease(rdev::Key::Space));
                send(&EventType::KeyPress(rdev::Key::Space));
                send(&EventType::KeyRelease(rdev::Key::Space));
                sleep(Duration::from_millis(30));
                send(&EventType::KeyPress(rdev::Key::Space));
                send(&EventType::KeyRelease(rdev::Key::Space));
            }
        }
    };
    if let Err(error) = rdev::listen(callback) {
        println!("Error: {:?}", error);
    }
}

fn send(event_type: &EventType) {
    let delay = std::time::Duration::from_millis(20);
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
    
    thread::sleep(delay);
}
