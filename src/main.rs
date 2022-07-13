mod engine;
mod sdl2_entry;

fn main() {
    let game = engine::GameSkel::new();
    sdl2_entry::run(game);
}
