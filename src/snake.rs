use crate::Coordinate;

#[derive(PartialEq, Clone, Debug)]
pub struct Snake {
    parts: Vec<Coordinate>
}

impl Snake {
    pub fn new(head: Coordinate) -> Self {
        Snake {
            parts: vec![head]
        }
    }

    pub fn grow(&mut self) -> () {
        let last_part = self.parts.last().unwrap().clone();
        // Check edge conditions!
        self.parts.push(Coordinate { x_pos: last_part.x_pos, y_pos: last_part.y_pos });
    }

    pub fn slither(&mut self, max_x: usize) -> () {
        let current_snake = self.clone();

        for (position, part) in self.parts.iter_mut().enumerate() {
            if position == 0 {
                part.x_pos = if part.x_pos == 0 { max_x - 1 } else { part.x_pos - 1 };
            } else {
                part.x_pos = current_snake.parts.get(position - 1).unwrap().x_pos;
                part.y_pos = current_snake.parts.get(position - 1).unwrap().y_pos;
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
