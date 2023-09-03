use crate::Coordinate;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn opposite_of(&self, direction: &Direction) -> bool {
        let opposite = match direction {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        };

        *self == opposite
    }
}

#[derive(PartialEq, Clone, Debug, Copy)]
pub enum SnakePart {
    Head,
    Body,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Snake {
    parts: Vec<Coordinate>,
    map: HashMap<String, SnakePart>,
    direction: Direction,
}

impl Snake {
    pub fn new(head: Coordinate, direction: Direction) -> Self {
        Snake {
            parts: vec![head],
            map: HashMap::new(),
            direction,
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn grow(&mut self, max_x: usize, max_y: usize) {
        let last_part = *self.parts.last().unwrap();
        self.parts.push(match self.direction {
            Direction::Up => add_down(&last_part, &max_x, &max_y),
            Direction::Right => add_left(&last_part, &max_x, &max_y),
            Direction::Down => add_up(&last_part, &max_x, &max_y),
            Direction::Left => add_right(&last_part, &max_x, &max_y),
        });
    }

    pub fn slither(&mut self, max_x: usize, max_y: usize) {
        let current_snake = self.clone();
        self.map = HashMap::new();
        for (position, part) in self.parts.iter_mut().enumerate() {
            *part = if position == 0 {
                match self.direction {
                    Direction::Up => add_up(part, &max_x, &max_y),
                    Direction::Right => add_right(part, &max_x, &max_y),
                    Direction::Down => add_down(part, &max_x, &max_y),
                    Direction::Left => add_left(part, &max_x, &max_y),
                }
            } else {
                Coordinate::new(
                    current_snake.parts.get(position - 1).unwrap().x_pos,
                    current_snake.parts.get(position - 1).unwrap().y_pos,
                )
            };
            self.map.insert(
                (*part).into(),
                if position == 0 {
                    SnakePart::Head
                } else {
                    SnakePart::Body
                },
            );
        }
    }

    pub fn parts(&self) -> Vec<Coordinate> {
        self.parts.clone()
    }

    pub fn head_position(&self) -> Coordinate {
        *self.parts.first().unwrap()
    }

    pub fn direction(&self) -> Direction {
        self.direction.clone()
    }

    pub fn occupies_coordinates(&self, coordinate: Coordinate) -> Option<SnakePart> {
        self.map.get::<String>(&coordinate.into()).copied()
    }
}

fn add_up(coordinate: &Coordinate, _max_x: &usize, max_y: &usize) -> Coordinate {
    Coordinate::new(
        coordinate.x_pos,
        if coordinate.y_pos == 0 {
            max_y - 1
        } else {
            coordinate.y_pos - 1
        },
    )
}

fn add_right(coordinate: &Coordinate, max_x: &usize, _max_y: &usize) -> Coordinate {
    Coordinate::new(
        if coordinate.x_pos == max_x - 1 {
            0
        } else {
            coordinate.x_pos + 1
        },
        coordinate.y_pos,
    )
}

fn add_down(coordinate: &Coordinate, _max_x: &usize, max_y: &usize) -> Coordinate {
    Coordinate::new(
        coordinate.x_pos,
        if coordinate.y_pos == max_y - 1 {
            0
        } else {
            coordinate.y_pos + 1
        },
    )
}

fn add_left(coordinate: &Coordinate, max_x: &usize, _max_y: &usize) -> Coordinate {
    Coordinate::new(
        if coordinate.x_pos == 0 {
            max_x - 1
        } else {
            coordinate.x_pos - 1
        },
        coordinate.y_pos,
    )
}
