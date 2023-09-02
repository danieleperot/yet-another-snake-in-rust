use crate::Coordinate;

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left
}

#[derive(PartialEq, Clone, Debug)]
pub struct Snake {
    parts: Vec<Coordinate>,
    direction: Direction
}

impl Snake {
    pub fn new(head: Coordinate, direction: Direction) -> Self {
        Snake { parts: vec![head], direction }
    }

    pub fn grow(&mut self) -> () {
        let last_part = self.parts.last().unwrap().clone();
        // Check edge conditions!
        self.parts.push(Coordinate { x_pos: last_part.x_pos, y_pos: last_part.y_pos });
    }

    pub fn slither(&mut self, max_x: usize, max_y: usize) -> () {
        let current_snake = self.clone();
        for (position, part) in self.parts.iter_mut().enumerate() {
            *part = if position == 0 {
                match self.direction {
                    Direction::Up => move_up(part, &max_x, &max_y),
                    Direction::Right => move_right(part, &max_x, &max_y),
                    Direction::Down => move_down(part, &max_x, &max_y),
                    Direction::Left => move_left(part, &max_x, &max_y)
                }
            } else {
                Coordinate::new(
                    current_snake.parts.get(position - 1).unwrap().x_pos,
                    current_snake.parts.get(position - 1).unwrap().y_pos
                )
            }
        }
    }

    pub fn parts(&self) -> Vec<Coordinate> {
        self.parts.clone()
    }

    pub fn head_position(&self) -> Coordinate {
        self.parts.first().unwrap().clone()
    }

}

fn move_up(coordinate: &Coordinate, _max_x: &usize, max_y: &usize) -> Coordinate {
    Coordinate::new(
        coordinate.x_pos,
        if coordinate.y_pos == 0 { max_y - 1 } else { coordinate.y_pos - 1 },
    )
}

fn move_right(coordinate: &Coordinate, max_x: &usize, _max_y: &usize) -> Coordinate {
    Coordinate::new(
        if coordinate.x_pos == max_x - 1 { 0 } else { coordinate.x_pos + 1 },
        coordinate.y_pos
    )
}

fn move_down(coordinate: &Coordinate, _max_x: &usize, max_y: &usize) -> Coordinate {
    Coordinate::new(
        coordinate.x_pos,
        if coordinate.y_pos == max_y - 1 { 0 } else { coordinate.y_pos + 1 },
    )
}

fn move_left(coordinate: &Coordinate, max_x: &usize, _max_y: &usize) -> Coordinate {
    Coordinate::new(
        if coordinate.x_pos == 0 { max_x - 1 } else { coordinate.x_pos - 1 },
        coordinate.y_pos
    )
}
