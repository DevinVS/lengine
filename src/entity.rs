use sdl2::rect::Rect;

use crate::graphics::EntityGraphicsState;
use crate::physics::EntityPhysicsState;

// Struct which represents a given entity in the game world.
// A requirement is that they exist inside the world and take up space.
// Optionally an entity is able to be defined with certain system states
// Which allow different engine subsystems to query for appropriate entities
// And store state inside of them
pub struct Entity {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub graphics_state: Option<EntityGraphicsState>,
    pub physics_state: Option<EntityPhysicsState>
}

impl Entity {
    pub fn new(
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        graphics_state: Option<EntityGraphicsState>,
        physics_state: Option<EntityPhysicsState>
    ) -> Entity {
        Entity {
            x,
            y,
            width,
            height,
            graphics_state,
            physics_state
        }
    }

    pub fn builder() -> EntityBuilder {
        EntityBuilder::new()
    }

    // Return rect representing position inside the game world
    pub fn rect(&self) -> Rect {
        Rect::new(self.x, self.y, self.width, self.height)
    }
}

// Helper struct to build an entity
pub struct EntityBuilder {
    x: Option<i32>,
    y: Option<i32>,
    width: Option<u32>,
    height: Option<u32>,
    graphics_state: Option<EntityGraphicsState>,
    physics_state: Option<EntityPhysicsState>
}

impl EntityBuilder {
    pub fn new() -> EntityBuilder {
        EntityBuilder {
            x: None,
            y: None,
            width: None,
            height: None,
            graphics_state: None,
            physics_state: None
        }
    }

    pub fn x(mut self, x: i32) -> EntityBuilder {
        self.x = Some(x);
        self
    }

    pub fn y(mut self, y: i32) -> EntityBuilder {
        self.y = Some(y);
        self
    }

    pub fn width(mut self, width: u32) -> EntityBuilder {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: u32) -> EntityBuilder {
        self.height = Some(height);
        self
    }

    pub fn graphics_state(mut self, graphics_state: EntityGraphicsState) -> EntityBuilder {
        self.graphics_state = Some(graphics_state);
        self
    }

    pub fn physics_state(mut self, physics_state: EntityPhysicsState) -> EntityBuilder {
        self.physics_state = Some(physics_state);
        self
    }

    pub fn build(self) -> Result<Entity, String> {
        if self.x.is_none() || self.y.is_none() || self.width.is_none() || self.height.is_none() {
            return Err("Missing Required Fields for Entity".into());
        }

        Ok(Entity {
            x: self.x.unwrap(),
            y: self.y.unwrap(),
            width: self.width.unwrap(),
            height: self.height.unwrap(),
            graphics_state: self.graphics_state,
            physics_state: self.physics_state
        })
    }
}