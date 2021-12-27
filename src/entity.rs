use std::collections::HashMap;

use crate::geometry::EntityGeometryState;
use crate::graphics::EntityGraphicsState;
use crate::physics::EntityPhysicsState;

// Struct which represents a given entity in the game world.
// A requirement is that they exist inside the world and take up space.
// Optionally an entity is able to be defined with certain system states
// Which allow different engine subsystems to query for appropriate entities
// And store state inside of them
pub struct Entity {
    graphics_state: Option<EntityGraphicsState>,
    physics_state: Option<EntityPhysicsState>,
    geometry_state: Option<EntityGeometryState>,
}

impl Entity {

    pub fn new(
        graphics_state: Option<EntityGraphicsState>,
        physics_state: Option<EntityPhysicsState>,
        geometry_state: Option<EntityGeometryState>
    ) -> Entity {

        Entity {
            graphics_state,
            physics_state,
            geometry_state
        }

    }

    pub fn has_geometry(&self) -> bool { self.geometry_state.is_some() }
    pub fn has_physics(&self) -> bool { self.physics_state.is_some() }
    pub fn has_graphics(&self) -> bool { self.graphics_state.is_some() }

    pub fn geometry(&self) -> Option<&EntityGeometryState> { self.geometry_state.as_ref() }
    pub fn geometry_mut(&mut self) -> Option<&mut EntityGeometryState> { self.geometry_state.as_mut() }

    pub fn physics(&self) -> Option<&EntityPhysicsState> { self.physics_state.as_ref() }
    pub fn physics_mut(&mut self) -> Option<&mut EntityPhysicsState> { self.physics_state.as_mut() }

    pub fn graphics(&self) -> Option<&EntityGraphicsState> { self.graphics_state.as_ref() }
    pub fn graphics_mut(&self) -> Option<&EntityGraphicsState> { self.graphics_state.as_ref() }
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
    // Next id for an entity
    next_entity_id: usize,

    // A hashmapof all game entities. These can be
    // drawable, have physical properties, or be
    // interactable in some way
    entities: HashMap<usize, Entity>,
}

impl EntityManager {
    // Create a new entity manager containing no entities
    pub fn new() -> EntityManager {
        EntityManager {
            next_entity_id: 0,
            entities: HashMap::new()
        }
    }

    // Add an entity to the entity manager
    pub fn add_entity(&mut self, entity: Entity) -> usize {
        let id = self.next_entity_id;
        self.next_entity_id += 1;
        self.entities.insert(id, entity);
        id
    }

    // Get an entity by its id
    pub fn get_entity(&self, id: usize) -> Option<&Entity> {
        self.entities.get(&id)
    }

    // Get a mutable entity by its id
    pub fn get_entity_mut(&mut self, id: usize) -> Option<&mut Entity> {
        self.entities.get_mut(&id)
    }

    // Query the entity manager for an iterator of entities conforming
    // to the query flags
    pub fn query(&self, query: Query) -> impl Iterator<Item = &Entity> {
        self.entities.values().filter(move |e| {
            (query & QueryFlag::PHYSICS == 0 || e.physics_state.is_some()) &&
            (query & QueryFlag::GRAPHICS == 0 || e.graphics_state.is_some())
        })
    }

    // Query the entity manager for a mutable iterator of entities conforming
    // to the query flags
    pub fn query_mut(&mut self, query: Query) -> impl Iterator<Item = &mut Entity> {
        self.entities.values_mut().filter(move |e| {
            (query & QueryFlag::PHYSICS == 0 || e.physics_state.is_some()) &&
            (query & QueryFlag::GRAPHICS == 0 || e.graphics_state.is_some())
        })
    }
}