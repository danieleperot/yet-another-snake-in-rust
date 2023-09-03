use crate::interact::UserAction;
use crate::snake::{Direction, Snake};
use crate::Coordinate;

#[derive(Clone)]
pub enum Event {
    Welcome,
    SimpleMove,
    AppleEaten,
    Exit,
}

pub struct World {
    snake: Snake,
    apples: Vec<Coordinate>,
    events: Vec<Event>,
    max_x: usize,
    max_y: usize,
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
            apples: vec![],
            events: vec![],
            max_x,
            max_y,
        };

        world.snake.grow();
        world.apples.push(Coordinate::random(max_x, max_y));
        world.apples.push(Coordinate::random(max_x, max_y));
        world.apples.push(Coordinate::random(max_x, max_y));

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

        for apple in self.apples.iter_mut() {
            if *apple == self.snake.head_position() {
                self.snake.grow();
                self.events.push(Event::AppleEaten);
                *apple = Coordinate::random(self.max_x, self.max_y);
            }
        }
    }

    pub fn check_tile_in_position(&self, coordinate: Coordinate) -> TileType {
        for apple in &self.apples {
            if apple == &coordinate {
                return TileType::Apple;
            }
        }

        for (position, part_position) in self.snake.parts().iter().enumerate() {
            if part_position == &coordinate {
                return match position {
                    0 => TileType::SnakeHead,
                    _ => TileType::SnakeBody,
                };
            }
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
}
