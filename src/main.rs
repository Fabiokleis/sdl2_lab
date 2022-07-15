#![allow(dead_code)]

mod engine;
mod components;
mod sdl2_entry;

fn main() {
    let game = engine::GameSkel::new();
    sdl2_entry::run(game);
}
