use std::collections::HashSet;
use crate::effect::Effect;
use crate::entity::{Entity, EntityManager, Query, QueryFlag};
use crate::map::WorldMap;

use crate::geometry::GeometryComponent;
use crate::physics::PhysicsComponent;
use crate::graphics::GraphicsComponent;
use crate::animation::AnimationComponent;

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
    pub fn add_entity(&mut self,
        geometry: Option<GeometryComponent>,
        physics: Option<PhysicsComponent>,
        graphics: Option<GraphicsComponent>,
        animation: Option<AnimationComponent>
    ) -> usize {
        self.entities.add_entity(geometry, physics, graphics, animation)
    }

    // Get the currenly controlled player
    pub fn get_player(&self) -> Option<Entity> {
        if self.player_id.is_none() {
            return None;
        }

        self.entities.get_entity(self.player_id.unwrap())
    }

    // Get the currently controlled player mutably
    pub fn get_player_mut(&mut self) -> Option<Entity> {
        if self.player_id.is_none() {
            return None;
        }

        self.entities.get_entity_mut(self.player_id.unwrap())
    }

    // Return an iterator of drawable entities in the world
    pub fn drawables(&self) -> impl Iterator<Item = Entity> {
        self.entities.query(QueryFlag::GRAPHICS)
    }

    // Return a mutable iterator of drawable entities in the world
    pub fn drawables_mut(&mut self) -> impl Iterator<Item = Entity> {
        self.entities.query_mut(QueryFlag::GRAPHICS)
    }

    // Return an iterator of physical entites in the world
    pub fn physical(&self) -> impl Iterator<Item = Entity> {
        self.entities.query(QueryFlag::PHYSICS)
    }

    // Return a mutable itearator of physical entities in the world
    pub fn physical_mut(&mut self) -> impl Iterator<Item = Entity> {
        self.entities.query_mut(QueryFlag::PHYSICS)
    }

    pub fn animatable(&self) -> impl Iterator<Item = Entity> {
        self.entities.query(QueryFlag::ANIMATIONS)
    }

    pub fn animatable_mut(&mut self) -> impl Iterator<Item = Entity> {
        self.entities.query_mut(QueryFlag::ANIMATIONS)
    }

    // Wrapper for query
    pub fn query(&mut self, query: Query) -> impl Iterator<Item = Entity> {
        self.entities.query(query)
    }

    // Wrapper for query_mut
    pub fn query_mut(&mut self, query: Query) -> impl Iterator<Item = Entity> {
        self.entities.query_mut(query)
    }
}
