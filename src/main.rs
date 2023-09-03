mod snake;
mod interact;
mod world;
mod notify;

use std::thread::sleep;
use std::time::Duration;
use rand::Rng;
use crate::interact::{UserAction, UserInteraction};
use crate::notify::Notifications;
use crate::world::{Event, World};

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

fn main() -> () {
    let game_speed = 150;
    let mut world = World::new(40, 15);
    let notifications = Notifications::new(0.05);
    let mut user_interaction = UserInteraction::new();

    game(game_speed, &mut world, &notifications, &mut user_interaction);

    user_interaction.draw_outro();
    notifications.handle_event(Event::Exit);
}

fn game(game_speed: u64, world: &mut World, notifications: &Notifications, user_interaction: &mut UserInteraction) {
    user_interaction.draw_intro();
    notifications.handle_event(Event::Welcome);

    loop {
        match user_interaction.user_input() {
            UserAction::Unsupported => break,
            UserAction::Close => return,
            _ => {}
        }
    }
    user_interaction.draw_screen(&world);

    loop {
        let action = user_interaction.user_input();
        if action == UserAction::Close {
            return;
        }

        world.tick(action);

        user_interaction.draw_screen(&world);
        notifications.handle_world_events(&world);

        sleep(Duration::from_millis(game_speed));
    }
}
