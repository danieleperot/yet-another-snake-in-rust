use crate::Coordinate;
use crate::world::{TileType, World};

use std::io;
use std::io::{Error, Stdout, Write};
use std::thread;
use std::time;

use termion;
use termion::AsyncReader;
use termion::event::Key;
use termion::input::{Keys, TermRead};
use termion::raw::{IntoRawMode, RawTerminal};

const PADDING: usize = 3;

#[derive(PartialEq)]
pub enum UserAction {
    Close,
    Other,
    NoKey
}

pub struct UserInteraction {
    stdout: RawTerminal<Stdout>,
    stdin: Keys<AsyncReader>
}

impl UserInteraction {
    pub fn new () -> Self {
        UserInteraction {
            // Set terminal to raw mode to allow reading stdin one key at a time
            stdout: io::stdout().into_raw_mode().unwrap(),
            // Use asynchronous stdin
            stdin: termion::async_stdin().keys()
        }
    }

    pub fn draw_screen(&mut self, world: &World) {
        self.clear_screen();

        self.draw_padding(world.max_x());
        for y in 0..world.max_y() {
            for x in (0 - PADDING as isize)..(world.max_x()) as isize {
                self.draw_x(world, y, x);
            }

            self.println("");
        }
        self.draw_padding(world.max_x());

        for line in BOTTOM_TEXT {
            self.println(line);
        }

        self.done_drawing();
    }

    pub fn draw_intro(&mut self) {
        self.clear_screen();

        for line in INTRO_SCREEN {
            self.println(line);
        }

        self.done_drawing();
    }

    pub fn user_input(&mut self) -> UserAction {
        return match self.stdin.next() {
            None => UserAction::NoKey,
            Some(input) => match input {
                Err(_) => UserAction::NoKey,
                Ok(key) => match key {
                    Key::Char('q') => UserAction::Close,
                    Key::Ctrl('C') => UserAction::Close,
                    _ => UserAction::Other
                }
            }
        }
    }

    fn clear_screen(&mut self) {
        write!(
            self.stdout,
            "{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1)
        ).unwrap();
    }

    fn draw_padding(&mut self, size: usize) {
        for _ in 0..(size + 2 * PADDING) { self.print("=") }
        self.println("");
    }

    fn draw_x(&mut self, world: &World, y: usize, x: isize) {
        if x < 0 {
            self.print(" ");
            return;
        }

        match world.check_tile_in_position(Coordinate::new(x as usize, y)) {
            TileType::SnakeHead => self.print("@"),
            TileType::SnakeBody => self.print("#"),
            TileType::Apple => self.print("A"),
            TileType::Empty => self.print("."),
        }
    }

    fn print(&mut self, string: &str) {
        write!(self.stdout, "{}", string.replace("\n", "\r\n")).unwrap();
    }

    fn println(&mut self, string: &str) {
        self.print(format!("{}\n", string).as_str());
    }

    fn done_drawing(&mut self) {
        self.stdout.lock().flush().unwrap();
    }

    fn drop(&mut self) {
        self.stdout.suspend_raw_mode().unwrap();
        self.stdout.lock().flush().unwrap();
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

const BOTTOM_TEXT: [&str; 2] = [
    "",
    "Press Q to exit. Press W,A,S or D to move the snake."
];