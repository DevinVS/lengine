use sdl2::rect::Rect;

pub struct EntityGeometryState {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl EntityGeometryState {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> EntityGeometryState {
        EntityGeometryState {
            x, y, width, height
        }
    }

    pub fn rect(&self) -> Rect {
        Rect::new(self.x, self.y, self.width, self.height)
    }
}