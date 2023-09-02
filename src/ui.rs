use crate::Coordinate;
use crate::world::{TileType, World};

const PADDING: usize = 3;

pub fn draw_screen(world: &World) {
    clear_screen();

    draw_padding(world.max_x());
    for y in 0..world.max_y() {
        for x in (0 - PADDING as isize)..(world.max_x()) as isize {
            draw_x(world, y, x);
        }

        println!();
    }
    draw_padding(world.max_x());
}

pub fn draw_intro() {
    clear_screen();

    for line in INTRO_SCREEN {
        println!("{}", line);
    }
}

pub fn user_input() {
    let mut s = "".to_string();
    std::io::stdin().read_line(&mut s).unwrap();
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
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

const INTRO_SCREEN: [&str; 8] = [
    "",
    "      ====================================================",
    "      ||               Welcome to Snake!                ||",
    "      ||                                                ||",
    "      ||            Daniele Perot (c) 2023              ||",
    "      ====================================================",
    "",
    "                --  Press ANY KEY to start --"
];