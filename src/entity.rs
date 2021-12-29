use std::collections::{HashMap, HashSet};

use crate::animation::AnimationComponent;
use crate::geometry::GeometryComponent;
use crate::graphics::GraphicsComponent;
use crate::physics::PhysicsComponent;

// Struct which represents a given entity in the game world.
// A requirement is that they exist inside the world and take up space.
// Optionally an entity is able to be defined with certain system states
// Which allow different engine subsystems to query for appropriate entities
// And store state inside of them


pub struct Entity<'a> {
    states: &'a mut HashSet<String>,
    graphics_component: &'a mut Option<GraphicsComponent>,
    physics_component: &'a mut Option<PhysicsComponent>,
    geometry_component: &'a mut Option<GeometryComponent>,
    animation_component: &'a mut Option<AnimationComponent>
}

impl<'a> Entity<'a> {
    pub fn states(&self) -> &HashSet<String> {
        self.states
    }

    pub fn add_state(&mut self, state: String) {
        self.states.insert(state);
    }

    pub fn remove_state(&mut self, state: String) {
        self.states.remove(&state);
    }

    pub fn contains_state(&self, state: String) -> bool {
        self.states.contains(&state)
    }

    pub fn has_geometry(&self) -> bool { self.geometry_component.is_some() }
    pub fn has_physics(&self) -> bool { self.physics_component.is_some() }
    pub fn has_graphics(&self) -> bool { self.graphics_component.is_some() }
    pub fn has_animation(&self) -> bool { self.graphics_component.is_some() }

    pub fn geometry(&self) -> Option<&GeometryComponent> { self.geometry_component.as_ref() }
    pub fn geometry_mut(&mut self) -> Option<&mut GeometryComponent> { self.geometry_component.as_mut() }

    pub fn physics(&self) -> Option<&PhysicsComponent> { self.physics_component.as_ref() }
    pub fn physics_mut(&mut self) -> Option<&mut PhysicsComponent> { self.physics_component.as_mut() }

    pub fn graphics(&self) -> Option<&GraphicsComponent> { self.graphics_component.as_ref() }
    pub fn graphics_mut(&mut self) -> Option<&mut GraphicsComponent> { self.graphics_component.as_mut() }

    pub fn animation(&self) -> Option<&AnimationComponent> { self.animation_component.as_ref() }
    pub fn animation_mut(&mut self) -> Option<&mut AnimationComponent> { self.animation_component.as_mut() }
}

// Numeric constants that we use as flags to
// Query entities inside of world
pub mod QueryFlag {
    pub const GEOMETRY: u8      = 0b00001;   // Entity has geometric properties
    pub const PHYSICS: u8       = 0b00011;   // Entity has physical and geometric properties
    pub const GRAPHICS: u8      = 0b00101;   // Entity has rendering properties and geometric properties
    pub const ANIMATIONS: u8    = 0b01101;   // Entity has animation properties and graphics properties
    pub const EFFECTS: u8       = 0b10001;   // Entity has effect properties and geometric properties
}

// Type to query objects out of the world
pub type Query = u8;

// Object for managing entities, presents query interface
pub struct EntityManager {
    // Properties for each entity
    states: Vec<HashSet<String>>,
    geometry: Vec<Option<GeometryComponent>>,
    physics: Vec<Option<PhysicsComponent>>,
    graphics: Vec<Option<GraphicsComponent>>,
    animation: Vec<Option<AnimationComponent>>
}

impl EntityManager {
    // Create a new entity manager containing no entities
    pub fn new() -> EntityManager {
        EntityManager {
            states: Vec::new(),
            geometry: Vec::new(),
            physics: Vec::new(),
            graphics: Vec::new(),
            animation: Vec::new()
        }
    }

    // Add an entity to the entity manager
    pub fn add_entity(&mut self,
        geometry: Option<GeometryComponent>,
        physics: Option<PhysicsComponent>,
        graphics: Option<GraphicsComponent>,
        animation: Option<AnimationComponent>,
    ) -> usize {
        self.states.push(HashSet::new());
        self.geometry.push(geometry);
        self.physics.push(physics);
        self.graphics.push(graphics);
        self.animation.push(animation);

        self.states.len()-1
    }

    // Get an entity by its id
    pub fn get_entity(&mut self, id: usize) -> Option<Entity> {
        if id >= self.states.len() {
            return None;
        }

        Some(Entity {
            states: self.states.get_mut(id).unwrap(),
            geometry_component: self.geometry.get_mut(id).unwrap(),
            physics_component: self.physics.get_mut(id).unwrap(),
            graphics_component: self.graphics.get_mut(id).unwrap(),
            animation_component: self.animation.get_mut(id).unwrap()
        })
    }

    // Get a mutable entity by its id
    pub fn get_entity_mut(&mut self, id: usize) -> Option<Entity> {
        if id >= self.states.len() {
            return None;
        }

        Some(Entity {
            states: self.states.get_mut(id).unwrap(),
            geometry_component: self.geometry.get_mut(id).unwrap(),
            physics_component: self.physics.get_mut(id).unwrap(),
            graphics_component: self.graphics.get_mut(id).unwrap(),
            animation_component: self.animation.get_mut(id).unwrap()
        })
    }

    // Query the entity manager for an iterator of entities conforming
    // to the query flags
    pub fn query<'a>(&'a mut self, query: Query) -> impl Iterator<Item = Entity<'a>> {
        (0..self.states.len()).filter(move |i| {
            (query & 0b00001 == 0 || self.geometry[*i].is_some()) &&
            (query & 0b00010 == 0 || self.physics[*i].is_some()) &&
            (query & 0b00100 == 0 || self.graphics[*i].is_some()) &&
            (query & 0b01000 == 0 || self.animation[*i].is_some())
        }).map(|i| {
            Entity {
                states: self.states.get_mut(i).unwrap(),
                geometry_component: &mut None,
                physics_component: &mut None,
                graphics_component: &mut None,
                animation_component: &mut None,
            }
        })
    }

    // Query the entity manager for a mutable iterator of entities conforming
    // to the query flags
    pub fn query_mut(&self, query: Query) -> impl Iterator<Item = Entity> {
        (0..self.states.len()).filter(move |i| {
            (query & 0b00001 == 0 || self.geometry[*i].is_some()) &&
            (query & 0b00010 == 0 || self.physics[*i].is_some()) &&
            (query & 0b00100 == 0 || self.graphics[*i].is_some()) &&
            (query & 0b01000 == 0 || self.animation[*i].is_some())
        }).map(|i| {
            self.get_entity(i).unwrap()
        })
    }
}
