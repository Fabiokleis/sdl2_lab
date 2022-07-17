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
use specs::WorldExt;
use specs::{DispatcherBuilder, Join, World};

/// ecs
use crate::components;

/// sdl2 bindings mods
mod texture_manager;

/// engine conf
use crate::engine;
use crate::engine::utils;
use crate::engine::{GameSkel, SCREEN_HEIGHT, SCREEN_WIDTH};

/// consts
const ASSETS_PATH: &'static str = "/home/urameshi/sdl2_rust/sdl2_lab/assets/";

fn render(
    canvas: &mut WindowCanvas,
    texture_manager: &mut texture_manager::TextureManager<WindowContext>,
    _texture_creator: &TextureCreator<WindowContext>,
    _font: &sdl2::ttf::Font,
    ecs: &World,
) -> Result<(), String> {
    let color = Color::RGB(0, 0, 0);
    canvas.set_draw_color(color);
    canvas.clear();

    let positions = ecs.read_storage::<components::Position>();
    let renderables = ecs.read_storage::<components::Renderable>();

    for (pos, renderable) in (&positions, &renderables).join() {
        let src = Rect::new(0, 0, renderable.src_width, renderable.src_height);
        let x = pos.x as i32;
        let y = pos.y as i32;
        let dest = Rect::new(
            x - (renderable.dest_width as i32 / 2),
            y - (renderable.dest_height as i32 / 2),
            renderable.dest_width,
            renderable.dest_height,
        );

        let center = Point::new(
            renderable.dest_width as i32 / 2,
            renderable.dest_height as i32 / 2,
        );
        let path = Path::new(ASSETS_PATH).join(Path::new(&renderable.texture_name));
        let texture = texture_manager.load(&path.to_str().unwrap())?;

        canvas.copy_ex(&texture, src, dest, renderable.rot, center, false, false)?;
    }

    canvas.present();
    Ok(())
}

/// run sdl2
pub fn run() {
    let sdl_context = sdl2::init().expect("Failed to initialize sdl2!");
    let video_subsys = sdl_context.video().expect("Coult not initialize video subsystem!");
    let window = video_subsys
        .window("sdl2_rust lab", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .expect("Failed to initialize window");
    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Coult not make a canvas");

    //    let mut canvas = {
    //        let video = sdl_context
    //            .video()
    //            .expect("Failed to initialize video subsystem!");
    //
    //        let window = video
    //            .window("sdl2_rust lab", SCREEN_WIDTH, SCREEN_HEIGHT)
    //            .position_centered()
    //            .build()
    //            .expect("Failed to initialize window");
    //
    //        window
    //            .into_canvas()
    //            .build()
    //            .expect("Failed to initialize canvas")
    //    };

    let texture_creator = canvas.texture_creator();
    let mut texture_manager = texture_manager::TextureManager::new(&texture_creator);

    // loading images
    let ship = Path::new(ASSETS_PATH).join(Path::new("imgs/space_ship.png"));
    let asteroid = Path::new(ASSETS_PATH).join(Path::new("imgs/asteroid.png"));
    texture_manager
        .load(&ship.to_str().unwrap())
        .expect("Could not load space ship image");
    texture_manager
        .load(&asteroid.to_str().unwrap())
        .expect("Could not load asteroid image");

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

    let mut game = GameSkel { ecs: World::new() };
    // game
    game.ecs.register::<components::Position>();
    game.ecs.register::<components::Renderable>();
    game.ecs.register::<components::Player>();
    game.ecs.register::<components::Asteroid>();

    let mut dispatcher = DispatcherBuilder::new()
        .with(engine::asteroid::AsteroidMover, "asteroid_mover", &[])
        .with(engine::asteroid::AsteroidCollider, "asteroid_collider", &[])
        .build();
    engine::load_world(&mut game.ecs);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    utils::key_down(&mut key_manager, " ".to_string());
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    utils::key_up(&mut key_manager, " ".to_string());
                }
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
        engine::update(&mut game.ecs, &mut key_manager);
        dispatcher.dispatch(&game.ecs);
        game.ecs.maintain();
        render(
            &mut canvas,
            &mut texture_manager,
            &texture_creator,
            &font,
            &game.ecs,
        )
        .expect("Failed to render!");

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

    }
}
