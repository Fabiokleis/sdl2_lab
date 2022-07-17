use specs::prelude::*;
use vector2d::Vector2D;
use specs::Component;

#[derive(Debug, Component)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub rot: f64,
}

#[derive(Debug, Component)]
pub struct Renderable {
    pub texture_name: String,
    pub src_width: u32,
    pub src_height: u32,
    pub dest_width: u32,
    pub dest_height: u32,
    pub frame: u32,
    pub total_frames: u32,
    pub rot: f64,
}

#[derive(Debug, Component)]
pub struct Player {
    pub impulse: Vector2D<f64>, 
    pub cur_speed: Vector2D<f64>,
}

#[derive(Debug, Component)]
pub struct Asteroid {
    pub speed: f64,
    pub rot_speed: f64,
}
