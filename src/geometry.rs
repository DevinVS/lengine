use crate::vector::Vector;

#[derive(Copy, Clone)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: u32,
    pub h: u32
}

impl Rect {
    pub fn new(x: f32, y: f32, w: u32, h: u32) -> Rect {
        Rect {x, y, w, h}
    }

    pub fn sdl2(&self) -> sdl2::rect::Rect {
        sdl2::rect::Rect::new(self.x as i32, self.y as i32, self.w, self.h)
    }

    pub fn has_intersection(&self, other: Rect) -> bool {
        if self.x >= other.x || other.x >= self.x {
            return false;
        }

        if self.y >= other.y || other.y >= self.y {
            return false;
        }

        return true;
    }

    pub fn after_vector(&mut self, v: Vector) {
        self.x += v.x();
        self.y += v.y();
    }

    pub fn after_depth(&mut self, d: u32) {
        self.y += self.h as f32 - d as f32;
        self.h = d;
    }
}

pub struct GeometryComponent {
    rect: Rect
}

impl GeometryComponent {
    pub fn new(x: f32, y: f32, width: u32, height: u32) -> GeometryComponent {
        GeometryComponent {
            rect: Rect::new(x, y, width, height)
        }
    }

    pub fn rect(&self) -> &Rect {
        &self.rect
    }

    pub fn rect_mut(&mut self) -> &mut Rect {
        &mut self.rect
    }
}
