use std::collections::{HashMap, HashSet};

use crate::{effect::Effect, entity::Entity, map::WorldMap};

// Numeric constants that we use as flags to 
// Query entities inside of world
mod QueryFlag {
    pub const PHYSICS: u8       = 0b0001;   // Entity has physical properties
    pub const GRAPHICS: u8      = 0b0010;   // Entity has rendering properties
    pub const ANIMATIONS: u8    = 0b0100;   // Entity has animation properties
    pub const EFFECTS: u8       = 0b1000;   // Entity has effect properties
}

// Type to query objects out of the world
type Query = u8;

// Object for managing entities, presents query interface
struct EntityManager {
    // Next id for an entity
    next_entity_id: usize,

    // A hashmapof all game entities. These can be
    // drawable, have physical properties, or be
    // interactable in some way
    entities: HashMap<usize, Entity>,
}

impl EntityManager {
    // Create a new entity manager containing no entities
    fn new() -> EntityManager {
        EntityManager {
            next_entity_id: 0,
            entities: HashMap::new()
        }
    }

    // Add an entity to the entity manager
    fn add_entity(&mut self, entity: Entity) -> usize {
        let id = self.next_entity_id;
        self.next_entity_id += 1;
        self.entities.insert(id, entity);
        id
    }

    // Get an entity by its id
    fn get_entity(&self, id: usize) -> Option<&Entity> {
        self.entities.get(&id)
    }

    // Get a mutable entity by its id
    fn get_entity_mut(&mut self, id: usize) -> Option<&mut Entity> {
        self.entities.get_mut(&id)
    }

    // Query the entity manager for an iterator of entities conforming
    // to the query flags
    fn query(&self, query: Query) -> impl Iterator<Item = &Entity> {
        self.entities.values().filter(move |e| {
            (query & QueryFlag::PHYSICS == 0 || e.physics_state.is_some()) &&
            (query & QueryFlag::GRAPHICS == 0 || e.graphics_state.is_some())
        })
    }

    // Query the entity manager for a mutable iterator of entities conforming
    // to the query flags
    fn query_mut(&mut self, query: Query) -> impl Iterator<Item = &mut Entity> {
        self.entities.values_mut().filter(move |e| {
            (query & QueryFlag::PHYSICS == 0 || e.physics_state.is_some()) &&
            (query & QueryFlag::GRAPHICS == 0 || e.graphics_state.is_some())
        })
    }
}

// Struct containing all game data and current state
pub struct World {
    // Stores and manages all entities
    entities: EntityManager,

    // Definition of the bounds and rendering for
    // the world
    map: WorldMap,

    // Currently enabled effects, each entity is responsible for
    // Handling its own effects based on this state
    effects: HashSet<Effect>,

    // Player / Currently controlled entity id
    pub player_id: Option<usize>
}

impl World {
    pub fn new(map: WorldMap) -> World {
        World { 
            entities: EntityManager::new(),
            map,
            effects: HashSet::new(),
            player_id: None
        }
    }

    // Wrapper for adding an entity to the EntityManager
    pub fn add_entity(&mut self, entity: Entity) -> usize {
        self.entities.add_entity(entity)
    }

    // Get the currenly controlled player
    pub fn get_player(&self) -> Option<&Entity> {
        if self.player_id.is_none() {
            return None;
        }

        self.entities.get_entity(self.player_id.unwrap())
    }

    // Get the currently controlled player mutably
    pub fn get_player_mut(&mut self) -> Option<&mut Entity> {
        if self.player_id.is_none() {
            return None;
        }

        self.entities.get_entity_mut(self.player_id.unwrap())
    }

    // Return an iterator of drawable entities in the world
    pub fn drawables(&self) -> impl Iterator<Item = &Entity> {
        self.entities.query(QueryFlag::GRAPHICS)
    }

    // Return a mutable iterator of drawable entities in the world
    pub fn drawables_mut(&mut self) -> impl Iterator<Item = &mut Entity> {
        self.entities.query_mut(QueryFlag::GRAPHICS)
    }

    // Return an iterator of physical entites in the world
    pub fn physical(&self) -> impl Iterator<Item = &Entity> {
        self.entities.query(QueryFlag::PHYSICS)
    }

    // Return a mutable itearator of physical entities in the world
    pub fn physical_mut(&mut self) -> impl Iterator<Item = &mut Entity> {
        self.entities.query_mut(QueryFlag::PHYSICS)
    }

    // Wrapper for query
    pub fn query(&mut self, query: Query) -> impl Iterator<Item = &Entity> {
        self.entities.query(query)
    }

    // Wrapper for query_mut
    pub fn query_mut(&mut self, query: Query) -> impl Iterator<Item = &mut Entity> {
        self.entities.query_mut(query)
    }
}