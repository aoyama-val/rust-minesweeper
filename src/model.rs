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

        for y in 0..BOARD_H {
            for x in 0..BOARD_W {
                self.board[y as usize][x as usize].number =
                    self.count_bombs(x as usize, y as usize);
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
        if self.board[y][x].is_open {
            return;
        }
        if self.board[y][x].is_bomb {
            self.board[y][x].is_open = true;
            self.is_over = true;
            self.requested_sounds.push("crash.wav");
        } else {
            self.auto_open(x, y);
        }
    }

    pub fn auto_open(&mut self, center_x: usize, center_y: usize) {
        if self.board[center_y][center_x].is_open || self.board[center_y][center_x].is_bomb {
            return;
        }
        self.board[center_y][center_x].is_open = true;
        if self.board[center_y][center_x].number == 0 {
            for yi in -1..=1 {
                let y = center_y as i32 + yi;
                for xi in -1..=1 {
                    let x = center_x as i32 + xi;
                    if x < 0 || x >= BOARD_W || y < 0 || y >= BOARD_H {
                        continue;
                    }
                    if x == 0 && y == 0 {
                        return;
                    }
                    self.auto_open(x as usize, y as usize);
                }
            }
        }
    }

    pub fn flag(&mut self, x: usize, y: usize) {
        if self.board[y][x].is_open {
            return;
        }
        self.board[y][x].is_flagged = !self.board[y][x].is_flagged;
    }

    pub fn count_bombs(&self, center_x: usize, center_y: usize) -> i32 {
        let mut result = 0;
        for yi in -1..=1 {
            let y = center_y as i32 + yi;
            for xi in -1..=1 {
                let x = center_x as i32 + xi;
                if x < 0 || x >= BOARD_W || y < 0 || y >= BOARD_H {
                    continue;
                }
                if self.board[y as usize][x as usize].is_bomb {
                    result += 1;
                }
            }
        }
        result
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
