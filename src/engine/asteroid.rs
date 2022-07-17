use specs::{Entities, Join, System, WriteStorage};

use crate::components::{Asteroid, Player, Position, Renderable};

use super::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub struct AsteroidMover;

impl<'a> System<'a> for AsteroidMover {
    type SystemData = (
        WriteStorage<'a, Position>,
        WriteStorage<'a, Renderable>,
        WriteStorage<'a, Asteroid>,
    );

    fn run(&mut self, (mut pos, mut ren, ast): Self::SystemData) {
        for (pos, ren, ast) in (&mut pos, &mut ren, &ast).join() {
            let radians = pos.rot.to_radians();
            pos.x += ast.speed * radians.sin();
            pos.y -= ast.speed * radians.cos();

            let half_width = ren.dest_width as u32 / 2;
            let half_height = ren.dest_height as u32 / 2;

            if pos.x > (SCREEN_WIDTH - half_height).into() || pos.x < half_width.into() {
                pos.rot = 360.0 - pos.rot;
            } else if pos.y > (SCREEN_HEIGHT - half_height).into() || pos.y < half_height.into() {
                if pos.rot > 180.0 {
                    pos.rot = 540.0 - pos.rot;
                } else {
                    pos.rot = 180.0 - pos.rot;
                }
            }
            ren.rot += ast.rot_speed;
            if ren.rot > 360.0 {
                ren.rot -= 360.0;
            }
            if ren.rot < 0.0 {
                ren.rot += 360.0;
            }
        }
    }
}

pub struct AsteroidCollider;

impl<'a> System<'a> for AsteroidCollider {
    type SystemData = (
        WriteStorage<'a, Position>,
        WriteStorage<'a, Renderable>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Asteroid>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (positions, rends, players, asteroids, entities) = data;

        for (p_pos, p_rend, _, entity) in (&positions, &rends, &players, &entities).join() {
            for (a_pos, a_rend, _) in (&positions, &rends, &asteroids).join() {
                let diff_x = (p_pos.x - a_pos.x).abs();
                let diff_y = (p_pos.y - a_pos.y).abs();

                // h^2 = c1^2 + c2^2; h = sqrt(c1^2 + c2^2);
                let hyp = ((diff_x * diff_x) + (diff_y * diff_y)).sqrt();
                if hyp < (a_rend.dest_width + p_rend.dest_width) as f64 / 2.0 {
                    println!("Player has died!");
                    entities
                        .delete(entity)
                        .expect("Could not delete space ship!");
                }
            }
        }
    }
}
