use std::time;

use rand::{rngs::StdRng, Rng, SeedableRng};

pub const FPS: i32 = 30;
pub const BOARD_W: i32 = 9;
pub const BOARD_H: i32 = 9;
pub const BOMB_COUNT: i32 = 10;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Command {
    None,
    Open,
    Flag,
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
        let rng = StdRng::seed_from_u64(timestamp);
        println!("random seed = {}", timestamp);
        // let rng = StdRng::seed_from_u64(0);

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
        for _ in 0..BOMB_COUNT {
            let r: usize = self.rng.gen_range(0..(BOARD_W * BOARD_H) as usize);
            self.board[r / BOARD_W as usize][r % BOARD_W as usize].is_bomb = true;
        }
    }

    pub fn update(&mut self, command: Command) {
        match command {
            Command::Open => {}
            Command::Flag => {}
            Command::None => {}
        }
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
