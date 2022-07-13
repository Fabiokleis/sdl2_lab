mod map;
mod usefulmath;
use map::Map;

enum GameState {
    Created,
    Running,
    Finished,
}

pub struct GameSkel {
    tile_map: Map,
    game_state: GameState,
}

impl GameSkel {
    pub fn new() -> Self {
        GameSkel {
            tile_map: Default::default(),
            game_state: GameState::Created,
        }
    }
}
