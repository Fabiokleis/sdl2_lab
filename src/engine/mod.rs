use crate::components::{Asteroid, Missile, Player, Position, Renderable};
use specs::{Builder, Join, World, WorldExt};
use std::collections::HashMap;
use vector2d::Vector2D;

pub mod asteroid;
pub mod missile;
pub mod utils;

pub const SCREEN_WIDTH: u32 = 800;
pub const SCREEN_HEIGHT: u32 = 600;

const ROT_SPEED: f64 = 1.5;
const PLAYER_SPEED: f64 = 4.5;
const FRICTION: f64 = 0.99;
const MAX_SPEED: f64 = 3.5;
const MAX_MISSILE: usize = 3;

#[derive(Default)]
pub struct GameSkel {
    pub ecs: World,
}

pub fn update(ecs: &mut World, key_manager: &mut HashMap<String, bool>) {
    let mut must_reload_world = false;
    let mut current_player_position = Position {
        x: 0.0,
        y: 0.0,
        rot: 0.0,
    };
    {
        let player = ecs.read_storage::<Player>();
        let positions = ecs.read_storage::<Position>();

        for (pos, player) in (&positions, &player).join() {
            current_player_position.x = pos.x;
            current_player_position.y = pos.y;
        }

        if player.join().count() < 1 {
            must_reload_world = true;
        }
    }

    if must_reload_world {
        ecs.delete_all();
        load_world(ecs);
    }

    let mut must_create_asteroid = false;
    {
        let asteroids = ecs.read_storage::<Asteroid>();
        if asteroids.join().count() < 1 {
            must_create_asteroid = true;
        }
    }

    if must_create_asteroid {
        if current_player_position.x > (SCREEN_WIDTH / 2).into()
            && current_player_position.y < (SCREEN_HEIGHT / 2).into()
        {
            current_player_position.x = SCREEN_WIDTH as f64 / 4.0;
            current_player_position.y = SCREEN_HEIGHT as f64 / 4.0;
            current_player_position.rot = 225.0;
        } else if current_player_position.x < (SCREEN_WIDTH / 2).into()
            && current_player_position.y < (SCREEN_HEIGHT / 2).into()
        {
            current_player_position.x = SCREEN_WIDTH as f64 - (SCREEN_WIDTH as f64 / 4.0);
            current_player_position.y = SCREEN_HEIGHT as f64 - (SCREEN_HEIGHT as f64 / 4.0);
            current_player_position.rot = 135.0;
        } else if current_player_position.x > (SCREEN_WIDTH / 2).into()
            && current_player_position.y < (SCREEN_HEIGHT / 2).into() 
        {
            current_player_position.x = SCREEN_WIDTH as f64 / 4.0;
            current_player_position.y = SCREEN_HEIGHT as f64 / 4.0;
            current_player_position.rot = 315.0;
        } else if current_player_position.x < (SCREEN_WIDTH / 2).into()
            && current_player_position.y > (SCREEN_HEIGHT / 2).into()
        {
            current_player_position.x = SCREEN_WIDTH as f64 - (SCREEN_WIDTH as f64 / 4.0);
            current_player_position.y = SCREEN_HEIGHT as f64 / 4.0;
            current_player_position.rot = 45.0;
        }



        create_asteroid(ecs, current_player_position);
    }

    let mut player_pos = Position {
        x: 0.0,
        y: 0.0,
        rot: 0.0,
    };
    let mut must_fire_missile = false;
    {
        let mut positions = ecs.write_storage::<Position>();
        let mut players = ecs.write_storage::<Player>();
        let mut renderable = ecs.write_storage::<Renderable>();

        for (player, pos, renderable) in (&mut players, &mut positions, &mut renderable).join() {
            if utils::is_key_pressed(&key_manager, "D") {
                pos.rot += ROT_SPEED;
            }
            if utils::is_key_pressed(&key_manager, "A") {
                pos.rot -= ROT_SPEED;
            }

            update_movement(pos, player);

            if utils::is_key_pressed(&key_manager, "W") {
                let radians = pos.rot.to_radians();

                let move_x = PLAYER_SPEED * radians.sin();
                let move_y = PLAYER_SPEED * radians.cos();
                let move_vec = Vector2D::<f64>::new(move_x, move_y);
                player.impulse += move_vec;
            }

            if pos.rot > 360.0 {
                pos.rot -= 360.0;
            }

            if pos.rot < 0.0 {
                pos.rot += 360.0;
            }

            if pos.x > SCREEN_WIDTH.into() {
                pos.x -= SCREEN_WIDTH as f64;
            }
            if pos.x < 0.0 {
                pos.x += SCREEN_WIDTH as f64;
            }

            if pos.y > SCREEN_HEIGHT.into() {
                pos.y -= SCREEN_HEIGHT as f64;
            }

            if pos.y < 0.0 {
                pos.y += SCREEN_HEIGHT as f64;
            }

            if utils::is_key_pressed(&key_manager, " ") {
                utils::key_up(key_manager, " ".to_string());
                must_fire_missile = true;
                player_pos.x = pos.x;
                player_pos.y = pos.y;
                player_pos.rot = pos.rot;
            }
            renderable.rot = pos.rot;
        }
    }

    if must_fire_missile {
        fire_missile(ecs, player_pos);
    }
}

pub fn update_movement(pos: &mut Position, player: &mut Player) {
    player.cur_speed *= FRICTION;

    player.cur_speed += player.impulse;
    if player.cur_speed.length() > MAX_SPEED {
        player.cur_speed = player.cur_speed.normalise();
        player.cur_speed = player.cur_speed * MAX_SPEED;
    }

    pos.x += player.cur_speed.x;
    pos.y -= player.cur_speed.y;

    player.impulse = Vector2D::new(0.0, 0.0);
}

fn fire_missile(ecs: &mut World, postion: Position) {
    {
        let missiles = ecs.read_storage::<Missile>();
        if missiles.count() > MAX_MISSILE - 1 {
            return;
        }
    }

    ecs.create_entity()
        .with(postion)
        .with(Renderable {
            texture_name: String::from("imgs/missile.png"),
            src_width: 50,
            src_height: 100,
            dest_width: 10,
            dest_height: 20,
            frame: 0,
            total_frames: 1,
            rot: 0.0,
        })
        .with(Missile { speed: 5.0 })
        .build();
}

fn create_asteroid(ecs: &mut World, postion: Position) {
    ecs.create_entity()
        .with(postion)
        .with(Renderable {
            texture_name: String::from("imgs/asteroid.png"),
            src_width: 100,
            src_height: 100,
            dest_width: 50,
            dest_height: 50,
            frame: 0,
            total_frames: 1,
            rot: 0.0,
        })
        .with(Asteroid {
            speed: 2.5,
            rot_speed: 0.5,
        })
        .build();
}

pub fn load_world(ecs: &mut World) {
    ecs.create_entity()
        .with(Position {
            x: 350.0,
            y: 250.0,
            rot: 0.0,
        })
        .with(Renderable {
            texture_name: String::from("imgs/space_ship.png"),
            src_width: 100,
            src_height: 100,
            dest_width: 50,
            dest_height: 50,
            frame: 0,
            total_frames: 1,
            rot: 0.0,
        })
        .with(Player {
            impulse: Vector2D::new(0.0, 0.0),
            cur_speed: Vector2D::new(0.0, 0.0),
        })
        .build();
    create_asteroid(ecs, Position { x: 400.0, y: 235.0, rot: 45.0 });
}
