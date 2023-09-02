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
        world.apples.push(Coordinate::new(4, 7));

        world
    }

    pub fn tick(&mut self) -> () {
        self.snake.slither();

        for apple in self.apples.iter_mut() {
            if apple.clone() == self.snake.head_position() {
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

