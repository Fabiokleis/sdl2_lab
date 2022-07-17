use specs::{Join, System, WriteStorage};

use crate::components::{Asteroid, Position, Renderable};

use super::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub struct AsteroidMover;

impl<'a> System<'a> for AsteroidMover {
    type SystemData = (
        WriteStorage<'a, Position>,
        WriteStorage<'a, Renderable>,
        WriteStorage<'a, Asteroid>,
    );

    fn run(&mut self, (mut pos, mut ren, mut ast): Self::SystemData) {
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
