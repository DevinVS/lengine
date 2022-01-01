use std::collections::HashSet;

use itertools::izip;

use crate::map::WorldMap;
use crate::geometry::PositionComponent;
use crate::physics::PhysicsComponent;
use crate::graphics::GraphicsComponent;
use crate::animation::AnimationComponent;

// Struct containing all game data and current state
pub struct World {
    // Definition of the bounds and rendering for the world
    map: WorldMap,
    // Player / Currently controlled entity id
    pub player_id: Option<usize>,

    // Properties for each entity
    pub states: Vec<HashSet<String>>,
    pub positions: Vec<Option<PositionComponent>>,
    pub physics: Vec<Option<PhysicsComponent>>,
    pub graphics: Vec<Option<GraphicsComponent>>,
    pub animations: Vec<Option<AnimationComponent>>
}

impl World {
    pub fn new(map: WorldMap) -> World {
        World {
            map,
            player_id: None,
            states: Vec::new(),
            positions: Vec::new(),
            physics: Vec::new(),
            graphics: Vec::new(),
            animations: Vec::new()
        }
    }
    // Add an entity to the entity manager
    pub fn add_entity(&mut self,
        position: Option<PositionComponent>,
        physics: Option<PhysicsComponent>,
        graphics: Option<GraphicsComponent>,
        animation: Option<AnimationComponent>,
    ) -> usize {
        self.states.push(HashSet::new());
        self.positions.push(position);
        self.physics.push(physics);
        self.graphics.push(graphics);
        self.animations.push(animation);

        self.states.len()-1
    }

    // Iterators over common properties of entities
    pub fn states(&self) -> impl Iterator<Item = &HashSet<String>> {
        self.states.iter()
    }

    pub fn states_mut(&mut self) -> impl Iterator<Item = &mut HashSet<String>> {
        self.states.iter_mut()
    }

    pub fn get_states_mut(&mut self, id: usize) -> &mut HashSet<String> {
        &mut self.states[id]
    }

    pub fn positions(&self) -> impl Iterator<Item = (usize, (&HashSet<String>, &PositionComponent))> {
        izip!(self.states.iter(), self.positions.iter()).enumerate()
            .filter(|e| e.1.1.is_some())
            .map(|e| (e.0, (e.1.0, e.1.1.as_ref().unwrap())))
    }

    pub fn positions_mut(&mut self) -> impl Iterator<Item = (usize, (&mut HashSet<String>, &mut PositionComponent))> {
        izip!(self.states.iter_mut(), self.positions.iter_mut()).enumerate()
            .filter(|e| e.1.1.is_some())
            .map(|e| (e.0, (e.1.0, e.1.1.as_mut().unwrap())))
    }

    pub fn physics(&self) -> impl Iterator<Item = (usize, (&HashSet<String>, &PositionComponent, &PhysicsComponent))> {
        izip!(self.states.iter(), self.positions.iter(), self.physics.iter()).enumerate()
            .filter(|e| e.1.1.is_some() && e.1.2.is_some())
            .map(|e| (e.0, (e.1.0, e.1.1.as_ref().unwrap(), e.1.2.as_ref().unwrap())))
    }

    pub fn physics_mut(&mut self) -> impl Iterator<Item = (usize, (&mut HashSet<String>, &mut PositionComponent, &mut PhysicsComponent))> {
        izip!(self.states.iter_mut(), self.positions.iter_mut(), self.physics.iter_mut()).enumerate()
            .filter(|e| e.1.1.is_some() && e.1.2.is_some())
            .map(|e| (e.0, (e.1.0, e.1.1.as_mut().unwrap(), e.1.2.as_mut().unwrap())))
    }

    pub fn graphics(&self) -> impl Iterator<Item = (usize, (&HashSet<String>, &PositionComponent, &GraphicsComponent))> {
        izip!(self.states.iter(), self.positions.iter(), self.graphics.iter()).enumerate()
            .filter(|e| e.1.1.is_some() && e.1.2.is_some())
            .map(|e| (e.0, (e.1.0, e.1.1.as_ref().unwrap(), e.1.2.as_ref().unwrap())))
    }

    pub fn graphics_mut(&mut self) -> impl Iterator<Item = (usize, (&mut HashSet<String>, &mut PositionComponent, &mut GraphicsComponent))> {
        izip!(self.states.iter_mut(), self.positions.iter_mut(), self.graphics.iter_mut()).enumerate()
            .filter(|e| e.1.1.is_some() && e.1.2.is_some())
            .map(|e| (e.0, (e.1.0, e.1.1.as_mut().unwrap(), e.1.2.as_mut().unwrap())))
    }

    pub fn animations(&self) -> impl Iterator<Item = (usize, (&HashSet<String>,&PositionComponent, &GraphicsComponent, &AnimationComponent))> {
        izip!(self.states.iter(), self.positions.iter(), self.graphics.iter(), self.animations.iter()).enumerate()
            .filter(|e| e.1.1.is_some() && e.1.2.is_some() && e.1.3.is_some())
            .map(|e| (e.0, (e.1.0, e.1.1.as_ref().unwrap(), e.1.2.as_ref().unwrap(), e.1.3.as_ref().unwrap())))
    }

    pub fn animations_mut(&mut self) -> impl Iterator<Item = (usize, (&mut HashSet<String>, &mut PositionComponent, &mut GraphicsComponent, &mut AnimationComponent))> {
        izip!(self.states.iter_mut(), self.positions.iter_mut(), self.graphics.iter_mut(), self.animations.iter_mut())
            .enumerate()
            .filter(|e| e.1.1.is_some() && e.1.2.is_some() && e.1.3.is_some())
            .map(|e| (e.0, (e.1.0, e.1.1.as_mut().unwrap(), e.1.2.as_mut().unwrap(), e.1.3.as_mut().unwrap())))
    }

    // Individual selectors for common properties of entities
    pub fn get_entity_states(&self, id: usize) -> &HashSet<String> {
        &self.states[id]
    }

    pub fn get_entity_states_mut(&mut self, id: usize) -> &mut HashSet<String> {
        &mut self.states[id]
    }

    pub fn get_entity_positions(&self, id: usize) -> Option<&PositionComponent> {
        self.positions[id].as_ref()
    }

    pub fn get_entity_positions_mut(&mut self, id: usize) -> Option<&mut PositionComponent> {
        self.positions[id].as_mut()
    }

    pub fn get_entity_physics(&self, id: usize) -> (Option<&PositionComponent>, Option<&PhysicsComponent>) {
        (self.positions[id].as_ref(), self.physics[id].as_ref())
    }

    pub fn get_entity_physics_mut(&mut self, id: usize) -> (Option<&mut PositionComponent>, Option<&mut PhysicsComponent>) {
        (self.positions[id].as_mut(), self.physics[id].as_mut())
    }

    pub fn get_entity_graphics(&self, id: usize) -> (Option<&PositionComponent>, Option<&GraphicsComponent>) {
        (self.positions[id].as_ref(), self.graphics[id].as_ref())
    }

    pub fn get_entity_graphics_mut(&mut self, id: usize) -> (Option<&mut PositionComponent>, Option<&mut GraphicsComponent>) {
        (self.positions[id].as_mut(), self.graphics[id].as_mut())
    }

    pub fn get_entity_animations(&self, id: usize) -> (Option<&PositionComponent>, Option<&GraphicsComponent>, Option<&AnimationComponent>) {
        (self.positions[id].as_ref(), self.graphics[id].as_ref(), self.animations[id].as_ref())
    }

    pub fn get_entity_animations_mut(&mut self, id: usize) -> (Option<&mut PositionComponent>, Option<&mut GraphicsComponent>, Option<&mut AnimationComponent>) {
        (self.positions[id].as_mut(), self.graphics[id].as_mut(), self.animations[id].as_mut())
    }

    // Control Entity State
    pub fn add_entity_state(&mut self, id: usize, state: String) {
        self.states[id].insert(state);
    }

    pub fn remove_entity_state(&mut self, id: usize, state: &String) {
        self.states[id].remove(state);
    }
}
