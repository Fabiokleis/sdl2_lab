extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;

use std::collections::HashMap;

mod utils;

/// engine conf
use crate::engine::GameSkel;

/// consts
const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

/// run sdl2 with a game instance
pub fn run(game: GameSkel) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut canvas = {
        let video = sdl_context.video().unwrap();

        let window = video
            .window("rust-sdl2 lab", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .build()
            .unwrap();

        window.into_canvas().build().unwrap()
    };

    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, 256, 256)
        .unwrap();

    texture
        .with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for y in 0..256 {
                for x in 0..256 {
                    let offset = y * pitch + x * 3;
                    buffer[offset] = x as u8;
                    buffer[offset + 1] = y as u8;
                    buffer[offset + 2] = 0;
                }
            }
        })
        .unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 255));

    canvas.clear();
    canvas
        .copy(&texture, None, Some(Rect::new(100, 100, 256, 256)))
        .unwrap();
    canvas.present();

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
        }
    }
}
