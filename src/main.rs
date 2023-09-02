mod snake;
mod ui;
mod world;
mod notify;

use std::thread::sleep;
use std::time::Duration;
use rand::Rng;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Coordinate {
    x_pos: usize,
    y_pos: usize
}

impl Coordinate {
    pub fn new(x_pos: usize, y_pos: usize) -> Self {
        Coordinate { x_pos, y_pos }
    }

    pub fn random(max_x: usize, max_y: usize) -> Coordinate {
        Coordinate {
            x_pos: rand::thread_rng().gen_range(0..max_x),
            y_pos: rand::thread_rng().gen_range(0..max_y)
        }
    }
}

fn main() {
    let mut world = world::World::new(40, 15);
    let game_speed = 300;

    ui::draw_intro();
    let notifications = notify::Notifications::new(0.005);
    notifications.handle_event(world::Event::Welcome);
    ui::user_input();
    ui::draw_screen(&world);

    loop {
        world.tick();
        ui::draw_screen(&world);
        notifications.handle_world_events(&world);

        sleep(Duration::from_millis(game_speed));
    }
}
