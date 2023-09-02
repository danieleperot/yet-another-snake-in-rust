mod snake;
mod draw;
mod world;
mod notify;

use std::thread::sleep;
use std::time::Duration;
use rand::Rng;

const MAX_Y: usize = 15;
const MAX_X: usize = 40;
const PADDING: usize = 3;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Coordinate {
    x_pos: usize,
    y_pos: usize
}

impl Coordinate {
    pub fn new(x_pos: usize, y_pos: usize) -> Self {
        Coordinate { x_pos, y_pos }
    }

    pub fn random() -> Coordinate {
        Coordinate {
            x_pos: rand::thread_rng().gen_range(0..MAX_X),
            y_pos: rand::thread_rng().gen_range(0..MAX_Y)
        }
    }
}

fn main() {
    let mut world = world::World::new();
    let game_speed = 300;

    notify::handle_event(&world::Event::Welcome);

    loop {
        world.tick();
        draw::draw_screen(&world);
        notify::handle_world_events(&world);

        sleep(Duration::from_millis(game_speed));
    }
}
