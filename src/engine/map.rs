use crate::engine::usefulmath::Vec2;

#[derive(Default)]
pub struct Map {
    size: Vec2<u32>,
    square_size: Vec2<u32>,
}

impl Map {
    pub fn size(&self) -> Vec2<u32> {
        self.size
    }

    pub fn square_size(&self) -> Vec2<u32> {
        self.square_size
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        self.size.x = width;
        self.size.y = height;
    }

    pub fn set_square_size(&mut self, width: u32, height: u32) {
        self.square_size.x = width;
        self.square_size.y = height;
    }
}
