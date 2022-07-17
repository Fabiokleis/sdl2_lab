use specs::prelude::*;
use specs::{Entities, Join};

use crate::components::{Asteroid, Missile, Player, Position, Renderable};

use super::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub struct MissileMove;

impl<'a> System<'a> for MissileMove {
    type SystemData = (
        WriteStorage<'a, Position>,
        WriteStorage<'a, Renderable>,
        ReadStorage<'a, Missile>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut positions, mut rends, missiles, entities) = data;

        for (pos, rend, miss, entity) in (&mut positions, &mut rends, &missiles, &entities).join() {
            let radian = pos.rot.to_radians();
            let move_x = miss.speed * radian.sin();
            let move_y = miss.speed * radian.cos();

            pos.x += move_x;
            pos.y -= move_y;

            if pos.x > SCREEN_WIDTH.into()
                || pos.x < 0.0
                || pos.y > SCREEN_HEIGHT.into()
                || pos.y < 0.0
            {
                entities.delete(entity).expect("Could not delete missile!");
            }

            rend.rot = pos.rot;
        }
    }
}

pub struct MissileStriker;

impl<'a> System<'a> for MissileStriker {
    type SystemData = (
        WriteStorage<'a, Position>,
        WriteStorage<'a, Renderable>,
        WriteStorage<'a, Missile>,
        WriteStorage<'a, Asteroid>,
        WriteStorage<'a, Player>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (positions, rends, missiles, asteroids, _players, entities) = &data;

        for (missile_pos, _, _, missile_entity) in (positions, rends, missiles, entities).join() {
            for (asteroid_pos, asteroid_rend, _, asteroid_entity) in
                (positions, rends, asteroids, entities).join()
            {
                let diff_x = (missile_pos.x - asteroid_pos.x).abs();
                let diff_y = (missile_pos.y - asteroid_pos.y).abs();
                let hyp = ((diff_x * diff_x) + (diff_y * diff_y)).sqrt();
                if hyp < asteroid_rend.dest_width as f64 / 2.0 {
                    entities
                        .delete(missile_entity)
                        .expect("Could not delete missile!");
                    entities
                        .delete(asteroid_entity)
                        .expect("Could not delete asteroid!");
                }
            }
        }
    }
}
