use specs::prelude::*;
use specs::{Entities, Join};

use crate::components::{
    Asteroid, Missile, PendingAsteroid, Player, Position, Renderable,
};

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
        let mut asteroid_creation = Vec::<PendingAsteroid>::new();

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

                    let new_size = asteroid_rend.dest_width / 2;
                    if new_size >= 25 {
                        asteroid_creation.push(PendingAsteroid {
                            x: asteroid_pos.x,
                            y: asteroid_pos.y,
                            rot: asteroid_pos.rot - 90.0,
                            size: new_size,
                        });
                        asteroid_creation.push(PendingAsteroid {
                            x: asteroid_pos.x,
                            y: asteroid_pos.y,
                            rot: asteroid_pos.rot + 90.0,
                            size: new_size,
                        });
                    }
                }
            }
        }

        let (mut positions, mut rends, _, mut asteroids, _, entities) = data;
        for new_asteroid in asteroid_creation {
            let new_ast =  entities.create();
            positions.insert(new_ast, Position { x: new_asteroid.x, y: new_asteroid.y, rot: new_asteroid.rot }).ok();
            asteroids.insert(new_ast, Asteroid { speed: 2.5, rot_speed: 0.5 }).ok();
            rends.insert(new_ast, Renderable {
                texture_name: String::from("imgs/asteroid.png"),
                src_width: 100,
                src_height: 100,
                dest_width: new_asteroid.size,
                dest_height: new_asteroid.size,
                frame: 0,
                total_frames: 1,
                rot: 0.0,
            }).ok();
        }

    }
}
