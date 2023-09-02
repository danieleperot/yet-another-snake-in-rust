use crate::world::{Event, World};

pub fn handle_world_events(world: &World) {
    for event in world.events() {
        std::thread::spawn(move || {
            handle_event(&event);
        });
    }
}

pub fn handle_event(event: &Event) {
    match event {
        Event::Welcome => play_sound(&START_GAME),
        Event::SimpleMove => play_sound(&SIMPLE_MOVE),
        Event::AppleEaten => play_sound(&APPLE_EATEN)
    }
}

fn play_sound(sound: &[Note]) {
    use std::time::Duration;
    use rodio::{OutputStream, Sink};
    use rodio::source::{SineWave, Source};

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    for note in sound {
        sink.append(
            SineWave::new(note.0)
                .take_duration(Duration::from_millis(note.1))
                .amplify(note.2)
        );
    }
    sink.sleep_until_end();
}

// ----------------------- SOUNDS -----------------------
struct Note (
    f32, // Frequency
    u64, // Duration (in milliseconds)
    f32  // Amplitude (from 0.0 to 1.0)
);

const SIMPLE_MOVE: [Note; 1] = [
    Note(300.0, 150, 0.05)
];

const APPLE_EATEN: [Note; 2] = [
    Note(600.0, 200, 0.20),
    Note(1000.0, 400, 0.20)
];

const START_GAME: [Note; 6] = [
    Note(600.0, 200, 0.20),
    Note(800.0, 200, 0.20),
    Note(1000.0, 200, 0.20),
    Note(1200.0, 400, 0.20),
    Note(1000.0, 200, 0.20),
    Note(1200.0, 400, 0.20)
];