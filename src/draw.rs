use crate::{MAX_Y, MAX_X, PADDING, Coordinate};
use crate::world::{TileType, World};

pub fn draw_screen(world: &World) {
    // Clear screen
    print!("\x1B[2J\x1B[1;1H");

    draw_padding();
    for y in 0..MAX_Y {
        for x in (0 - PADDING as isize)..(MAX_X) as isize {
            draw_x(world, y, x);
        }

        println!();
    }
    draw_padding();
}

fn draw_padding() {
    for _ in 0..(MAX_X + 2 * PADDING) { print!("=") }
    println!();
}

fn draw_x(world: &World, y: usize, x: isize) {
    if x < 0 {
        print!(" ");
        return;
    }

    match world.check_tile_in_position(Coordinate::new(x as usize, y)) {
        TileType::SnakeHead => print!("@"),
        TileType::SnakeBody => print!("#"),
        TileType::Apple => print!("A"),
        TileType::Empty => print!("."),
    }
}
