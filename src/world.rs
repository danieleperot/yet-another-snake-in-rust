use crate::Coordinate;
use crate::snake::Snake;

pub struct World {
    snake: Snake,
    apples: Vec<Coordinate>
}

pub enum TileType {
    SnakeHead,
    SnakeBody,
    Apple,
    Empty
}

impl World {
    pub fn new() -> World {
        let mut world = World {
            snake: Snake::new(),
            apples: vec![]
        };

        world.snake.grow();
        world.apples.push(Coordinate::new(15, 7));

        world
    }

    pub fn tick(&mut self) -> () {
        self.snake.slither();

        for apple in self.apples.iter_mut() {
            if apple.clone() == self.snake.head_position() {
                std::thread::spawn(|| {
                    use std::time::Duration;
                    use rodio::{OutputStream, Sink};
                    use rodio::source::{SineWave, Source};

                    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
                    let sink = Sink::try_new(&stream_handle).unwrap();

                    // Add a dummy source of the sake of the example.
                    sink.append(SineWave::new(600.0).take_duration(Duration::from_millis(200)).amplify(0.10));
                    sink.append(SineWave::new(800.0).take_duration(Duration::from_millis(500)).amplify(0.10));

                    // The sound plays in a separate thread. This call will block the current thread until the sink
                    // has finished playing all its queued sounds.
                    sink.sleep_until_end();
                });
                self.snake.grow();
                *apple = Coordinate::random();
            }
        }
    }

    pub fn check_tile_in_position(&self, coordinate: Coordinate) -> TileType {
        for apple in &self.apples {
            if apple == &coordinate { return TileType::Apple; }
        }

        for (position, part_position) in self.snake.parts().iter().enumerate() {
            if part_position == &coordinate {
                return match position {
                    0 => TileType::SnakeHead,
                    _ => TileType::SnakeBody
                };
            }
        }

        TileType::Empty
    }
}

