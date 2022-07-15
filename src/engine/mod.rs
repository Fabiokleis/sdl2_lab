use std::collections::HashMap; 
use specs::{Builder, World, WorldExt, Join};
use crate::components::{Player, Position, Renderable};

pub mod utils;

const ROT_SPEED: f64 = 1.5;

pub struct GameSkel {
    ecs: World,
}

impl GameSkel {
    pub fn new() -> Self {
        GameSkel {
            ecs: World::new(),
        }
    }

    pub fn ecs(&self) -> &World {
        &self.ecs
    }

    pub fn setup(&mut self) {
        self.ecs.register::<Renderable>();
        self.ecs.register::<Position>();
        self.ecs.register::<Player>();
    }

    pub fn update(&mut self, key_manager: &mut HashMap<String, bool>) {

        let mut positions = self.ecs.write_storage::<Position>();
        let players = self.ecs.write_storage::<Player>(); 

        for (_, pos) in (&players, &mut positions).join() {
            if utils::is_key_pressed(&key_manager, "D") {
                pos.rot += ROT_SPEED;
            }
            if utils::is_key_pressed(&key_manager, "A") {
                pos.rot -= ROT_SPEED;
            }
 
            if pos.rot > 360.0 {
                pos.rot -= 360.0;
            }
            
            if pos.rot < 0.0 {
                pos.rot += 360.0;
            }

        }
    }

    pub fn load_world(&mut self) {
        self.ecs
            .create_entity()
            .with(Position {
                x: 350.0,
                y: 250.0,
                rot: 0.0,
            })
            .with(Renderable {
                texture_name: String::from("imgs/space_ship.png"),
                src_width: 100,
                src_height: 100,
                dest_width: 100,
                dest_height: 100,
                frame: 0,
                total_frames: 1,
                rot: 0.0,
            })
            .with(Player {})
            .build();
    }
}
