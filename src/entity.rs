use sdl2::{rect::Rect, render::Texture};

pub struct Entity {
    id: usize,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    texture: Option<usize>
}

impl Entity {
    pub fn new(id: usize, x: i32, y: i32, width: u32, height: u32) -> Entity {
        Entity {
            id,
            x,
            y,
            width,
            height,
            texture: None
        }
    }

    pub fn new_drawable(id: usize, x: i32, y: i32, width: u32, height: u32, texture_id: usize) -> Entity {
        Entity {
            id,
            x,
            y,
            width,
            height,
            texture: Some(texture_id)
        }
    }

    pub fn rect(&self) -> Rect {
        Rect::new(self.x, self.y, self.width, self.height)
    }

    pub fn texture_id(&self) -> Option<usize> {
        self.texture
    }

}