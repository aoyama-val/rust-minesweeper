use sdl2::event::Event;
// use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::mixer;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{BlendMode, Canvas, Texture, TextureCreator};
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::{Window, WindowContext};
use std::collections::HashMap;
use std::fs;
use std::time::{Duration, SystemTime};
mod model;
use crate::model::*;

pub const WINDOW_TITLE: &str = "rust-minesweeper";
pub const CELL_SIZE: i32 = 40;
pub const SCREEN_WIDTH: i32 = BOARD_W * CELL_SIZE;
pub const SCREEN_HEIGHT: i32 = BOARD_H * CELL_SIZE;

struct Image<'a> {
    texture: Texture<'a>,
    #[allow(dead_code)]
    w: u32,
    h: u32,
}

impl<'a> Image<'a> {
    fn new(texture: Texture<'a>) -> Self {
        let q = texture.query();
        let image = Image {
            texture,
            w: q.width,
            h: q.height,
        };
        image
    }
}

struct Resources<'a> {
    images: HashMap<String, Image<'a>>,
    chunks: HashMap<String, sdl2::mixer::Chunk>,
    fonts: HashMap<String, sdl2::ttf::Font<'a, 'a>>,
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;

    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window(WINDOW_TITLE, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    init_mixer();

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    canvas.set_blend_mode(BlendMode::Blend);

    let texture_creator = canvas.texture_creator();
    let mut resources = load_resources(&texture_creator, &mut canvas, &ttf_context);

    let mut event_pump = sdl_context.event_pump()?;

    let mut game = Game::new();
    game.init();

    'running: loop {
        let started = SystemTime::now();

        let mut command = Command::None;
        // let ks = event_pump.keyboard_state();
        // if ks.is_scancode_pressed(sdl2::keyboard::Scancode::Left) {
        //     command = Command::Left;
        //     is_keydown = true;
        // } else if ks.is_scancode_pressed(sdl2::keyboard::Scancode::Right) {
        //     command = Command::Right;
        //     is_keydown = true;
        // } else if ks.is_scancode_pressed(sdl2::keyboard::Scancode::Up) {
        //     command = Command::Up;
        //     is_keydown = true;
        // } else if ks.is_scancode_pressed(sdl2::keyboard::Scancode::Down) {
        //     command = Command::Down;
        //     is_keydown = true;
        // }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(code),
                    ..
                } => {
                    match code {
                        Keycode::Escape => {
                            break 'running;
                        }
                        _ => {}
                    };
                }
                Event::MouseButtonDown {
                    x, y, mouse_btn, ..
                } => {
                    let x = (x as u32) / CELL_SIZE as u32;
                    let y = (y as u32) / CELL_SIZE as u32;
                    if mouse_btn == MouseButton::Left {
                        println!("left click {} {}", x, y);
                    } else if mouse_btn == MouseButton::Right {
                        println!("right click {} {}", x, y);
                    }
                }
                _ => {}
            }
        }
        render(&mut canvas, &game, &mut resources)?;

        play_sounds(&mut game, &resources);

        let finished = SystemTime::now();
        let elapsed = finished.duration_since(started).unwrap();
        let frame_duration = Duration::new(0, 1_000_000_000u32 / model::FPS as u32);
        if elapsed < frame_duration {
            ::std::thread::sleep(frame_duration - elapsed)
        }
    }

    Ok(())
}

fn init_mixer() {
    let chunk_size = 1_024;
    mixer::open_audio(
        mixer::DEFAULT_FREQUENCY,
        mixer::DEFAULT_FORMAT,
        mixer::DEFAULT_CHANNELS,
        chunk_size,
    )
    .expect("cannot open audio");
    let _mixer_context = mixer::init(mixer::InitFlag::MP3).expect("cannot init mixer");
}

fn load_resources<'a>(
    texture_creator: &'a TextureCreator<WindowContext>,
    #[allow(unused_variables)] canvas: &mut Canvas<Window>,
    ttf_context: &'a Sdl2TtfContext,
) -> Resources<'a> {
    let mut resources = Resources {
        images: HashMap::new(),
        chunks: HashMap::new(),
        fonts: HashMap::new(),
    };

    let entries = fs::read_dir("resources/image").unwrap();
    for entry in entries {
        let path = entry.unwrap().path();
        let path_str = path.to_str().unwrap();
        if path_str.ends_with(".bmp") {
            let temp_surface = sdl2::surface::Surface::load_bmp(&path).unwrap();
            let texture = texture_creator
                .create_texture_from_surface(&temp_surface)
                .expect(&format!("cannot load image: {}", path_str));

            let basename = path.file_name().unwrap().to_str().unwrap();
            let image = Image::new(texture);
            resources.images.insert(basename.to_string(), image);
        }
    }

    let entries = fs::read_dir("./resources/sound").unwrap();
    for entry in entries {
        let path = entry.unwrap().path();
        let path_str = path.to_str().unwrap();
        if path_str.ends_with(".wav") {
            let chunk = mixer::Chunk::from_file(path_str)
                .expect(&format!("cannot load sound: {}", path_str));
            let basename = path.file_name().unwrap().to_str().unwrap();
            resources.chunks.insert(basename.to_string(), chunk);
        }
    }

    let entries = fs::read_dir("./resources/font").unwrap();
    for entry in entries {
        let path = entry.unwrap().path();
        let path_str = path.to_str().unwrap();
        if path_str.ends_with(".ttf") {
            let font = ttf_context
                .load_font(path_str, 40) // FIXME: サイズ固定になっちゃってる
                .expect(&format!("cannot load font: {}", path_str));
            let basename = path.file_name().unwrap().to_str().unwrap();
            resources.fonts.insert(basename.to_string(), font);
        }
    }

    resources
}

fn render(
    canvas: &mut Canvas<Window>,
    game: &Game,
    resources: &mut Resources,
) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(192, 192, 192));
    canvas.clear();

    let font = resources.fonts.get_mut("boxfont2.ttf").unwrap();

    for y in 0..BOARD_H {
        for x in 0..BOARD_W {
            render_font(
                canvas,
                font,
                // format!("{}", game.board[y as usize][x as usize].number),
                format!("{}", x % 10),
                CELL_SIZE * x + 11,
                CELL_SIZE * y - 2,
                Color::RGBA(255, 255, 255, 255),
            );
        }
    }

    // render borders
    canvas.set_draw_color(Color::RGB(64, 64, 64));
    for y in 1..BOARD_H {
        canvas.draw_line(
            Point::new(0, CELL_SIZE * y),
            Point::new(SCREEN_WIDTH, CELL_SIZE * y),
        )?;
    }
    for x in 1..BOARD_W {
        canvas.draw_line(
            Point::new(CELL_SIZE * x, 0),
            Point::new(CELL_SIZE * x, SCREEN_HEIGHT),
        )?;
    }

    if game.is_over {
        canvas.set_draw_color(Color::RGBA(255, 0, 0, 128));
        canvas.fill_rect(Rect::new(0, 0, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32))?;
    }

    if game.is_clear {
        render_font(
            canvas,
            font,
            "CLEAR!!".to_string(),
            140,
            240,
            Color::RGBA(255, 255, 0, 255),
        );
    }

    canvas.present();

    Ok(())
}

fn render_font(
    canvas: &mut Canvas<Window>,
    font: &sdl2::ttf::Font,
    text: String,
    x: i32,
    y: i32,
    color: Color,
) {
    let texture_creator = canvas.texture_creator();

    let surface = font.render(&text).blended(color).unwrap();
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .unwrap();
    canvas
        .copy(
            &texture,
            None,
            Rect::new(x, y, texture.query().width, texture.query().height),
        )
        .unwrap();
}

fn play_sounds(game: &mut Game, resources: &Resources) {
    for sound_key in &game.requested_sounds {
        let chunk = resources
            .chunks
            .get(&sound_key.to_string())
            .expect("cannot get sound");
        sdl2::mixer::Channel::all()
            .play(&chunk, 0)
            .expect("cannot play sound");
    }
    game.requested_sounds = Vec::new();
}
