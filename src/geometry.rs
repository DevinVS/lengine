use crate::vector::Vector;

#[derive(Debug, Copy, Clone)]
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
        let left = self.x;
        let right = self.x + self.w as f32;
        let top = self.y;
        let bottom = self.y + self.h as f32;

        let o_left = other.x;
        let o_right = other.x + other.w as f32;
        let o_top = other.y;
        let o_bottom = other.y + other.h as f32;

        if right <= o_left || o_right <= left {
            return false;
        }

        if top >= o_bottom || o_top >= bottom {
            return false;
        }

        return true;
    }

    pub fn apply_vector(&mut self, v: Vector) {
        self.x += v.x();
        self.y += v.y();
    }

    pub fn after_position(mut self, pos: &PositionComponent) -> Rect {
        self.x += pos.x;
        self.y += pos.y;

        self
    }

    pub fn after_depth(mut self, d: u32) -> Rect {
        self.y += self.h as f32 - d as f32;
        self.h = d;

        self
    }
}

impl std::ops::Add<Rect> for Rect {
    type Output = Rect;

    fn add(self, other: Rect) -> Rect {
        Rect::new(self.x+other.x,self.y+other.y,self.w+other.w,self.h+other.h)
    }
}

#[derive(Debug)]
pub struct PositionComponent {
    x: f32,
    y: f32
}

impl PositionComponent {
    pub fn new(x: f32, y: f32) -> PositionComponent {
        PositionComponent {
            x, y
        }
    }

    pub fn apply_vector(&mut self, vec: Vector) {
        self.x += vec.x();
        self.y += vec.y();
    }
}
