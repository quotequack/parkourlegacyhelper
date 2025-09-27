use std::{thread::sleep, time::Duration, thread  };
use rdev::{EventType,Event};
use rdev::{simulate, SimulateError};
use rodio::{play, Source};
use rodio::Sink;

fn main() {
    let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
        .expect("opened default stream");
    let sink = Sink::connect_new(&stream_handle.mixer());
    let callback = move |event: Event| {
        if let EventType::KeyPress(key) = event.event_type {
            if key == rdev::Key::Alt {
                send(&EventType::KeyRelease(rdev::Key::Space));
                send(&EventType::KeyPress(rdev::Key::Space));
                send(&EventType::KeyRelease(rdev::Key::Space));
            }
            if key == rdev::Key::KeyR {
                let wave = rodio::source::SineWave::new(394.0);
                stream_handle.mixer().add(wave.take_duration(Duration::from_millis(50)).amplify(0.20));
                send(&EventType::KeyRelease(rdev::Key::Space));
                send(&EventType::KeyPress(rdev::Key::Space));
                send(&EventType::KeyRelease(rdev::Key::Space));
                sleep(Duration::from_millis(30));
                send(&EventType::KeyPress(rdev::Key::Space));
                send(&EventType::KeyRelease(rdev::Key::Space));

            }
            if key == rdev::Key::Escape {
                println!("Exiting...");
                let wave = rodio::source::SineWave::new(880.0);
                stream_handle.mixer().add(wave.take_duration(Duration::from_millis(300)).amplify(0.20));
                sleep(Duration::from_millis(300));
                std::process::exit(0);
            }
            if key == rdev::Key::Num3 {
                let wave = rodio::source::SineWave::new(550.0);
                stream_handle.mixer().add(wave.take_duration(Duration::from_millis(50)).amplify(0.20));
                send(&EventType::KeyPress(rdev::Key::KeyE));
                send(&EventType::KeyRelease(rdev::Key::KeyE));
                sleep(Duration::from_millis(18));
                send(&EventType::KeyPress(rdev::Key::Space));
                send(&EventType::KeyRelease(rdev::Key::Space));
                send(&EventType::KeyRelease(rdev::Key::KeyW));
                send(&EventType::KeyPress(rdev::Key::KeyS));
                send(&EventType::KeyPress(rdev::Key::KeyC));
                send(&EventType::KeyRelease(rdev::Key::KeyC));
                sleep(Duration::from_millis(10));
                send(&EventType::KeyPress(rdev::Key::Space));
                send(&EventType::KeyRelease(rdev::Key::Space));
                send(&EventType::KeyRelease(rdev::Key::KeyS));
                send(&EventType::KeyPress(rdev::Key::KeyC));
                send(&EventType::KeyRelease(rdev::Key::KeyC));
                send(&EventType::KeyPress(rdev::Key::KeyW));
            }
            if key == rdev::Key::Num4 {
                let wave = rodio::source::SineWave::new(650.0);
                stream_handle.mixer().add(wave.take_duration(Duration::from_millis(50)).amplify(0.20));
                send(&EventType::KeyPress(rdev::Key::KeyE));
                send(&EventType::KeyRelease(rdev::Key::KeyE));
                sleep(Duration::from_millis(15));
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