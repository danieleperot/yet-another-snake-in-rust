use crate::interact::UserAction;
use crate::snake::{Direction, Snake, SnakePart};
use crate::Coordinate;
use std::collections::HashMap;

#[derive(Clone)]
pub enum Event {
    Welcome,
    SimpleMove,
    AppleEaten,
    Crash,
    Exit,
}

pub struct World {
    snake: Snake,
    events: Vec<Event>,
    apples: HashMap<String, bool>,
    max_x: usize,
    max_y: usize,
    done: bool,
}

pub enum TileType {
    SnakeHead,
    SnakeBody,
    Apple,
    Empty,
}

impl World {
    pub fn new(max_x: usize, max_y: usize) -> World {
        let mut world = World {
            snake: Snake::new(
                Coordinate {
                    x_pos: max_x / 2,
                    y_pos: max_y / 2,
                },
                Direction::Right,
            ),
            apples: HashMap::new(),
            events: vec![],
            max_x,
            max_y,
            done: false,
        };

        world.snake.grow(max_x, max_y);
        world.add_random_apple();
        world.add_random_apple();
        world.add_random_apple();

        world
    }

    pub fn tick(&mut self, action: UserAction) {
        self.events.clear();

        if let Some(new_direction) = match action {
            UserAction::MoveUp => Some(Direction::Up),
            UserAction::MoveRight => Some(Direction::Right),
            UserAction::MoveDown => Some(Direction::Down),
            UserAction::MoveLeft => Some(Direction::Left),
            _ => None,
        } {
            if !self.snake.direction().opposite_of(&new_direction) {
                self.snake.change_direction(new_direction);
            }
        }

        self.snake.slither(self.max_x, self.max_y);
        self.events.push(Event::SimpleMove);

        self.check_apple_eaten();
        self.check_crash();
    }

    pub fn check_tile_in_position(&self, coordinate: Coordinate) -> TileType {
        if self.apples.get::<String>(&coordinate.into()).is_some() {
            return TileType::Apple;
        }

        if let Some(snake_part) = self.snake.occupies_coordinates(coordinate) {
            return match snake_part {
                SnakePart::Head => TileType::SnakeHead,
                SnakePart::Body => TileType::SnakeBody,
            };
        }

        TileType::Empty
    }

    pub fn events(&self) -> Vec<Event> {
        self.events.clone()
    }

    pub fn max_x(&self) -> usize {
        self.max_x
    }

    pub fn max_y(&self) -> usize {
        self.max_y
    }

    pub fn game_ended(&self) -> bool {
        self.done
    }

    fn check_apple_eaten(&mut self) {
        let head_position: String = self.snake.head_position().into();
        if self.apples.get(&head_position).is_some() {
            self.snake.grow(self.max_x, self.max_y);
            self.events.push(Event::AppleEaten);
            self.apples.remove(&head_position);
            self.add_random_apple()
        }
    }

    fn add_random_apple(&mut self) {
        self.apples
            .insert(Coordinate::random(self.max_x, self.max_y).into(), true);
    }

    fn check_crash(&mut self) {
        let head_position = self.snake.head_position();
        if let Some(part) = self.snake.occupies_coordinates(head_position) {
            if part == SnakePart::Body {
                self.events.push(Event::Crash);
                self.done = true;
            }
        }
    }
}
