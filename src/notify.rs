use std::time::Duration;
use rodio::{OutputStream, Sink, Source};
use rodio::source::SineWave;
use crate::world::{Event, World};

pub struct Notifications {
    max_sound_volume: f32
}

impl Notifications {
    pub fn new(max_sound_volume: f32) -> Self {
        Notifications { max_sound_volume }
    }

    pub fn handle_world_events(&self, world: &World) {
        for event in world.events() {
            self.handle_event(event.clone());
        }
    }

    pub fn handle_event(&self, event: Event) {
        match event {
            Event::Welcome => self.play_sound(&START_GAME),
            Event::SimpleMove => self.play_sound(&SIMPLE_MOVE),
            Event::AppleEaten => self.play_sound(&APPLE_EATEN)
        }
    }

    fn play_sound(&self, sound: &[Note]) {
        let volume = self.max_sound_volume.clone();
        let mut sine_waves = vec![];

        for note in sound.clone().iter() {
            let new_note = note.clone();
            let amplitude = if new_note.2 <= volume { new_note.2 } else { volume };
            sine_waves.push(
                SineWave::new(new_note.0)
                    .take_duration(Duration::from_millis(new_note.1))
                    .amplify(amplitude)
                );
        }

        std::thread::spawn(move || {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();
            for note in sine_waves {
                sink.append(note);
            }
            sink.sleep_until_end();
        });
    }
}


// ----------------------- SOUNDS -----------------------
#[derive(Copy, Clone)]
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