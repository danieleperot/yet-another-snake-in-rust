use crate::world::{TileType, World};
use crate::Coordinate;

use std::io;
use std::io::{Stdout, Write};

use termion::color::Color;
use termion::event::Key;
use termion::input::{Keys, TermRead};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{color, AsyncReader};

const PADDING: usize = 3;

#[derive(PartialEq)]
pub enum UserAction {
    MoveUp,
    MoveRight,
    MoveDown,
    MoveLeft,
    Close,
    Unsupported,
    None,
}

#[derive(PartialEq)]
enum ScreenType {
    NotStarted,
    Intro,
    Outro,
    Game,
}

pub struct UserInteraction {
    stdout: RawTerminal<Stdout>,
    stdin: Keys<AsyncReader>,
    current_screen: ScreenType,
}

impl UserInteraction {
    pub fn new() -> Self {
        UserInteraction {
            // Set terminal to raw mode to allow reading stdin one key at a time
            stdout: io::stdout().into_raw_mode().unwrap(),
            // Use asynchronous stdin
            stdin: termion::async_stdin().keys(),
            current_screen: ScreenType::NotStarted,
        }
    }

    //noinspection RsBorrowChecker
    pub fn draw_screen(&mut self, world: &World) {
        self.clear_screen(ScreenType::Game);

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
        self.clear_screen(ScreenType::Intro);

        for line in INTRO_SCREEN {
            self.println(line);
        }

        self.done_drawing();
    }

    pub fn draw_outro(&mut self) {
        self.clear_screen(ScreenType::Outro);

        for line in OUTRO_SCREEN {
            self.println(line);
        }

        self.done_drawing();
    }

    pub fn user_input(&mut self) -> UserAction {
        match self.stdin.next() {
            None => UserAction::None,
            Some(input) => match input {
                Err(_) => UserAction::None,
                Ok(key) => match key {
                    Key::Char('w') => UserAction::MoveUp,
                    Key::Char('a') => UserAction::MoveLeft,
                    Key::Char('s') => UserAction::MoveDown,
                    Key::Char('d') => UserAction::MoveRight,
                    Key::Char('q') => UserAction::Close,
                    Key::Ctrl('C') => UserAction::Close,
                    _ => UserAction::Unsupported,
                },
            },
        }
    }

    fn clear_screen(&mut self, new_screen: ScreenType) {
        if self.current_screen != new_screen {
            write!(self.stdout, "{}", termion::clear::All,).unwrap();
            self.current_screen = new_screen;
        }

        write!(self.stdout, "{}", termion::cursor::Goto(1, 1)).unwrap();
    }

    fn draw_padding(&mut self, size: usize) {
        for _ in 0..(size + 2 * PADDING) {
            self.print("==")
        }
        self.println("");
    }

    fn draw_x(&mut self, world: &World, y: usize, x: isize) {
        if x < 0 {
            self.print("  ");
            return;
        }

        match world.check_tile_in_position(Coordinate::new(x as usize, y)) {
            TileType::SnakeHead => self.print("👀"),
            TileType::SnakeBody => self.color_print("##", color::Green),
            TileType::Apple => self.print("🍎"),
            TileType::Empty => self.print(".."),
        }
    }

    fn color_print<C: Color>(&mut self, string: &str, color: C) {
        self.print(format!("{}{}{}", color::Fg(color), string, color::Fg(color::Reset)).as_str())
    }

    fn print(&mut self, string: &str) {
        write!(self.stdout, "{}", string.replace('\n', "\r\n")).unwrap();
    }

    fn println(&mut self, string: &str) {
        self.print(format!("{}\n", string).as_str());
    }

    fn done_drawing(&mut self) {
        self.stdout.lock().flush().unwrap();
    }
}

const INTRO_SCREEN: [&str; 9] = [
    "",
    "      ====================================================",
    "      ||               Welcome to Snake!                ||",
    "      ||                                                ||",
    "      ||            Daniele Perot (c) 2023              ||",
    "      ====================================================",
    "",
    "                --  Press ANY KEY to start --",
    "                      or press Q to exit",
];

const OUTRO_SCREEN: [&str; 7] = [
    "",
    "      ====================================================",
    "      ||                                                ||",
    "      ||            Thanks for playing Snake!           ||",
    "      ||                                                ||",
    "      ====================================================",
    "",
];

const BOTTOM_TEXT: [&str; 2] = ["", "Press Q to exit. Press W,A,S or D to move the snake."];
