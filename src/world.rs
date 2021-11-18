use std::{collections::{HashMap, HashSet}, path::Path};

use crate::{effect::Effect, entity::Entity, map::WorldMap};

// Contains all game data and state
pub struct World {
    // Next id for an entity
    next_entity_id: usize,

    // A list of all game entities. These can be
    // drawable, have physical properties, or be
    // interactable in some way
    entities: HashMap<usize, Entity>,

    // Definition of the bounds and rendering for
    // the world
    map: WorldMap,

    // Currently enabled effects, each entity is responsible for
    // Handling its own effects based on this state
    effects: HashSet<Effect>
}

impl World {
    pub fn new(map: WorldMap) -> World {
        World { 
            next_entity_id: 0,
            entities: HashMap::new(),
            map,
            effects: HashSet::new()
        }
    }

    // Add an entity to the world
    pub fn add_entity(&mut self, id: usize, entity: Entity) {
        self.entities.insert(id, entity);
        self.next_entity_id += 1;
    }

    // Remove an entity from the world
    pub fn remove_entity(&mut self, id: usize) {
        self.entities.remove(&id);
    }

    // Get an entity based on its id
    pub fn get_entity(&self, id: usize) -> Option<&Entity> {
        self.entities.get(&id)
    }

    // Get a mutable entiyt based on its id
    pub fn get_entity_mut(&mut self, id: usize) -> Option<&mut Entity> {
        self.entities.get_mut(&id)
    }

    // Return an iterator of entities in the world
    pub fn entities(&self) -> impl Iterator<Item = &Entity> {
        self.entities.values()
    }

    // Return an iterator of drawable entities in the world
    pub fn drawables(&self) -> impl Iterator<Item = &Entity> {
        self.entities.values().filter(|e| e.texture_id().is_some())
    }
}