use std::thread::sleep;
use std::time::Duration;

const MAX_Y: usize = 15;
const MAX_X: usize = 40;
const PADDING: usize = 3;

#[derive(PartialEq, Copy, Clone, Debug)]
struct Coordinate {
    x_pos: usize,
    y_pos: usize
}

fn main() {
    let mut snake: Vec<Coordinate> = vec![];
    snake.push(Coordinate { x_pos: 4, y_pos: 4 });
    snake.push(Coordinate { x_pos: 5, y_pos: 4 });
    snake.push(Coordinate { x_pos: 5, y_pos: 5 });
    snake.push(Coordinate { x_pos: 6, y_pos: 5 });
    snake.push(Coordinate { x_pos: 6, y_pos: 6 });
    snake.push(Coordinate { x_pos: 6, y_pos: 7 });
    snake.push(Coordinate { x_pos: 6, y_pos: 8 });

    loop {
        let current_snake = snake.clone();
        for (position, part) in snake.iter_mut().enumerate() {
            if position == 0 {
                part.x_pos = if part.x_pos == 0 { MAX_X - 1 } else { part.x_pos - 1 };
            } else {
                part.x_pos = current_snake.get(position - 1).unwrap().x_pos;
                part.y_pos = current_snake.get(position - 1).unwrap().y_pos;
            }
        }

        draw_screen(&snake);
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        // sleep(Duration::from_millis(1000));
    }
}

fn draw_screen(snake: &Vec<Coordinate>) {
    // Clear screen
    print!("\x1B[2J\x1B[1;1H");

    draw_padding();
    for y in 0..MAX_Y {
        for x in (0 - PADDING as isize)..(MAX_X) as isize {
            draw_x(snake, y, x);
        }

        println!();
    }
    draw_padding();
}

fn draw_padding() {
    for _ in 0..(MAX_X + 2 * PADDING) { print!("=") }
    println!();
}

fn draw_x(snake: &Vec<Coordinate>, y: usize, x: isize) {
    if x < 0 {
        print!(" ");
        return;
    }

    for (position, piece) in snake.iter().enumerate() {
        if piece.x_pos == x as usize && piece.y_pos == y {
            if position == 0 { print!("@") } else { print!("{}", position) }
            return;
        }
    }

    print!(".");
}
