use crate::world::{Event, World};
use rodio::source::SineWave;
use rodio::{OutputStream, Sink, Source};
use std::time::Duration;

pub struct Notifications {
    max_sound_volume: f32,
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
            Event::Welcome => self.play_sound(&START_GAME, true),
            Event::SimpleMove => self.play_sound(&SIMPLE_MOVE, true),
            Event::AppleEaten => self.play_sound(&APPLE_EATEN, true),
            Event::Crash => self.play_sound(&CRASH, false),
            Event::Exit => self.play_sound(&EXIT_GAME, false),
        }
    }

    fn play_sound(&self, sound: &[Note], in_thread: bool) {
        let volume = self.max_sound_volume;
        let mut waves = vec![];

        for note in sound.clone().iter() {
            let new_note = *note;
            let amplitude = if new_note.2 <= volume {
                new_note.2
            } else {
                volume
            };

            waves.push(
                SineWave::new(new_note.0)
                    .take_duration(Duration::from_millis(new_note.1))
                    .amplify(amplitude),
            );
        }

        let handler = move || {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();
            for note in waves {
                sink.append(note);
            }
            sink.sleep_until_end();
        };

        if in_thread {
            std::thread::spawn(handler);
        } else {
            handler();
        }
    }
}

// ----------------------- SOUNDS -----------------------
#[derive(Copy, Clone)]
struct Note(
    f32,  // Frequency
    u64,  // Duration (in milliseconds)
    f32,  // Amplitude (from 0.0 to 1.0)
    Wave, // Type of wave
);

#[derive(Copy, Clone)]
enum Wave {
    Sine,
}

impl Note {
    const fn new(frequency: f32, duration: u64, amplitude: f32, wave: Wave) -> Self {
        Note(frequency, duration, amplitude, wave)
    }

    const fn sine(frequency: f32, duration: u64, amplitude: f32) -> Self {
        Note::new(frequency, duration, amplitude, Wave::Sine)
    }
}

const SIMPLE_MOVE: [Note; 1] = [Note::sine(300.0, 150, 0.05)];

const APPLE_EATEN: [Note; 2] = [Note::sine(600.0, 200, 0.20), Note::sine(1000.0, 400, 0.20)];

const START_GAME: [Note; 6] = [
    Note::sine(600.0, 200, 0.20),
    Note::sine(800.0, 200, 0.20),
    Note::sine(1000.0, 200, 0.20),
    Note::sine(1200.0, 400, 0.20),
    Note::sine(1000.0, 200, 0.20),
    Note::sine(1200.0, 400, 0.20),
];

const EXIT_GAME: [Note; 3] = [
    Note::sine(600.0, 200, 0.20),
    Note::sine(1000.0, 400, 0.20),
    Note::sine(800.0, 600, 0.20),
];

const CRASH: [Note; 8] = [
    Note::sine(1200.0, 200, 0.20),
    Note::sine(800.0, 200, 0.20),
    Note::sine(1000.0, 200, 0.20),
    Note::sine(600.0, 200, 0.20),
    Note::sine(800.0, 200, 0.20),
    Note::sine(400.0, 200, 0.20),
    Note::sine(600.0, 200, 0.20),
    Note::sine(400.0, 800, 0.20),
];
