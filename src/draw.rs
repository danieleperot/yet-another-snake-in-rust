use crate::Coordinate;
use crate::world::{TileType, World};

const PADDING: usize = 3;

pub fn draw_screen(world: &World) {
    // Clear screen
    print!("\x1B[2J\x1B[1;1H");

    draw_padding(world.max_x());
    for y in 0..world.max_y() {
        for x in (0 - PADDING as isize)..(world.max_x()) as isize {
            draw_x(world, y, x);
        }

        println!();
    }
    draw_padding(world.max_x());
}

fn draw_padding(size: usize) {
    for _ in 0..(size + 2 * PADDING) { print!("=") }
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
