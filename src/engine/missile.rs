use specs::prelude::*;
use specs::{Entities, Join};

use crate::components::{Position, Renderable, Missile};

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

            if pos.x > SCREEN_WIDTH.into() || pos.x < 0.0 || pos.y > SCREEN_HEIGHT.into() || pos.y < 0.0 {
                entities.delete(entity).expect("Could not delete missile!");
            }

            rend.rot = pos.rot;
        }
    }
}
