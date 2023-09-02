mod snake;
mod interact;
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
    let game_speed = 300;
    let mut world = world::World::new(40, 15);
    let notifications = notify::Notifications::new(0.005);
    let user_interaction = interact::UserInteraction::new();

    user_interaction.draw_intro();
    notifications.handle_event(world::Event::Welcome);
    user_interaction.user_input();
    user_interaction.draw_screen(&world);

    loop {
        world.tick();

        user_interaction.draw_screen(&world);
        notifications.handle_world_events(&world);

        sleep(Duration::from_millis(game_speed));
    }
}
