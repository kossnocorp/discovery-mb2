#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::{board::Board, display::blocking::Display, hal::Timer};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

enum Direction {
    Right,
    Down,
    Left,
    Up,
}

struct Image {
    pub pos: (usize, usize),
    pub dir: Direction,
}

impl Image {
    pub fn new() -> Self {
        Image {
            pos: (0, 0),
            dir: Direction::Right,
        }
    }

    pub fn tick(&mut self) {
        match self.dir {
            Direction::Right => {
                if self.pos.1 == 4 {
                    self.dir = Direction::Down;
                }
            }

            Direction::Down => {
                if self.pos.0 == 4 {
                    self.dir = Direction::Left;
                }
            }

            Direction::Left => {
                if self.pos.1 == 0 {
                    self.dir = Direction::Up;
                }
            }

            Direction::Up => {
                if self.pos.0 == 0 {
                    self.dir = Direction::Right;
                }
            }
        }

        match self.dir {
            Direction::Right => self.pos.1 += 1,
            Direction::Down => self.pos.0 += 1,
            Direction::Left => self.pos.1 -= 1,
            Direction::Up => self.pos.0 -= 1,
        }
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let mut buffer = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];
    let mut image = Image::new();

    loop {
        buffer[image.pos.0][image.pos.1] = 1;
        display.show(&mut timer, buffer, 50);
        display.clear();
        buffer[image.pos.0][image.pos.1] = 0;
        image.tick();
    }
}
