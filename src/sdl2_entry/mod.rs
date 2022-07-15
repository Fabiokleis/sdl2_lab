extern crate sdl2;
use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::time::Duration;

use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::TextureCreator;
use sdl2::render::WindowCanvas;
use sdl2::video::WindowContext;
use sdl2::{event::Event, rect::Point, rect::Rect};

mod texture_manager;
/// sdl2 bindings mods
mod utils;

/// engine conf
use crate::engine::GameSkel;

use crate::components;
/// ecs
use specs::{World, WorldExt};

/// consts
const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

const BOO_WIDTH: u32 = 150;
const BOO_HEIGHT: u32 = 200;
const OUT_BOO_WIDTH: u32 = 150;
const OUT_BOO_HEIGHT: u32 = 200;

fn render(
    canvas: &mut WindowCanvas,
    texture_manager: &mut texture_manager::TextureManager<WindowContext>,
    texture_creator: &TextureCreator<WindowContext>,
    font: &sdl2::ttf::Font,
) -> Result<(), String> {
    let color = Color::RGB(60, 60, 60);
    canvas.set_draw_color(color);
    canvas.clear();

    let src = Rect::new(0, 0, BOO_WIDTH, BOO_HEIGHT);
    let x = SCREEN_WIDTH as i32 / 2;
    let y = SCREEN_HEIGHT as i32 / 2;

    let dest = Rect::new(
        x - (OUT_BOO_WIDTH / 2) as i32,
        y - (OUT_BOO_HEIGHT / 2) as i32,
        OUT_BOO_WIDTH,
        OUT_BOO_HEIGHT,
    );
    let center = Point::new(OUT_BOO_WIDTH as i32 / 2, OUT_BOO_HEIGHT as i32 / 2);
    let f = env::current_dir()
        .unwrap()
        .join("assets/imgs/fishingboo.png");

    let texture = texture_manager.load(f.to_str().unwrap()).unwrap();
    canvas
        .copy_ex(&texture, src, dest, 0.0, center, true, false)
        .unwrap();
    canvas.present();
    Ok(())
}

/// run sdl2 with a game instance
pub fn run(game: GameSkel) {
    let sdl_context = sdl2::init().expect("Failed to initialize sdl2!");

    let mut canvas = {
        let video = sdl_context
            .video()
            .expect("Failed to initialize video subsystem!");

        let window = video
            .window("sdl2_rust lab", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .build()
            .expect("Failed to initialize window");

        window
            .into_canvas()
            .build()
            .expect("Failed to initialize canvas")
    };

    let texture_creator = canvas.texture_creator();
    let mut texture_manager = texture_manager::TextureManager::new(&texture_creator);

    // load image
    let f = env::current_dir()
        .unwrap()
        .join("assets/imgs/fishingboo.png");
    texture_manager
        .load(&f.to_str().unwrap())
        .expect("Failed to load image file!");

    // loading fonts
    let ttf_ctxt = sdl2::ttf::init().expect("Failed to initialize font!");
    let p = env::current_dir()
        .unwrap()
        .join("assets/fonts/free_pixel.ttf");
    let font_path = Path::new(&p);

    let mut font = ttf_ctxt
        .load_font(font_path, 128)
        .expect("Failed to load font!");
    font.set_style(sdl2::ttf::FontStyle::BOLD);

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut key_manager: HashMap<String, bool> = HashMap::new();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown { keycode, .. } => match keycode {
                    None => {}
                    Some(key) => {
                        utils::key_down(&mut key_manager, key.to_string());
                    }
                },
                Event::KeyUp { keycode, .. } => match keycode {
                    None => {}
                    Some(key) => {
                        utils::key_up(&mut key_manager, key.to_string());
                    }
                },
                _ => {}
            }

            render(&mut canvas, &mut texture_manager, &texture_creator, &font)
                .expect("Failed to render!");
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
