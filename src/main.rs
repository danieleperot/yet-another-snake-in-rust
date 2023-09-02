mod snake;
mod draw;
mod world;

use std::thread::sleep;
use std::time::Duration;

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
}

fn main() {
    let mut world = world::World::new();
    let game_speed = 500;

    loop {
        world.tick();
        draw::draw_screen(&world);

        sleep(Duration::from_millis(game_speed));
    }
}
