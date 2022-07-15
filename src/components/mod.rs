use specs::prelude::*;
use specs::Component;

#[derive(Component)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub rot: f64,
}

#[derive(Component)]
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

#[derive(Component)]
pub struct Player {}
