use std::time;

use rand::{
    distributions::{Distribution, Uniform},
    rngs::StdRng,
    Rng, SeedableRng,
};

pub const FPS: i32 = 30;
pub const BOARD_W: i32 = 9;
pub const BOARD_H: i32 = 9;
pub const BOMB_COUNT: i32 = 10;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Command {
    None,
    Open(usize, usize),
    Flag(usize, usize),
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Cell {
    pub is_bomb: bool,
    pub is_open: bool,
    pub is_flagged: bool,
    pub number: i32,
}

impl Cell {}

pub struct Game {
    pub is_over: bool,
    pub is_clear: bool,
    pub rng: StdRng,
    pub board: [[Cell; BOARD_W as usize]; BOARD_H as usize],
    pub requested_sounds: Vec<&'static str>,
}

impl Game {
    pub fn new() -> Self {
        let now = time::SystemTime::now();
        let timestamp = now
            .duration_since(time::UNIX_EPOCH)
            .expect("SystemTime before UNIX EPOCH!")
            .as_secs();
        // let rng = StdRng::seed_from_u64(timestamp);
        // println!("random seed = {}", timestamp);
        let rng = StdRng::seed_from_u64(0);

        let mut game = Game {
            is_over: false,
            is_clear: false,
            rng: rng,
            board: [[Cell::default(); BOARD_W as usize]; BOARD_H as usize],
            requested_sounds: Vec::new(),
        };
        game.init();
        game
    }

    pub fn init(&mut self) {
        println!("game init");
        let distribution = Uniform::from(0..(BOARD_W * BOARD_H) as usize);
        for _ in 0..BOMB_COUNT {
            loop {
                let r: usize = distribution.sample(&mut self.rng);
                let x = r % BOARD_W as usize;
                let y = r / BOARD_W as usize;
                if !self.board[y][x].is_bomb {
                    self.board[y][x].is_bomb = true;
                    println!("bomb {} {}", x, y);
                    break;
                }
            }
        }
    }

    pub fn update(&mut self, command: Command) {
        if self.is_over || self.is_clear {
            return;
        }

        match command {
            Command::Open(x, y) => {
                println!("open {} {}", x, y);
                self.open(x, y);
            }
            Command::Flag(x, y) => {
                println!("flag {} {}", x, y);
                self.flag(x, y);
            }
            Command::None => {}
        }
    }

    pub fn open(&mut self, x: usize, y: usize) {
        self.board[y][x].is_open = true;
        if self.board[y][x].is_bomb {
            self.is_over = true;
            self.requested_sounds.push("crash.wav");
        }
    }
    pub fn flag(&mut self, x: usize, y: usize) {
        self.board[y][x].is_flagged = true;
    }
}

pub fn clamp<T: PartialOrd>(min: T, value: T, max: T) -> T {
    if value < min {
        return min;
    }
    if value > max {
        return max;
    }
    value
}
