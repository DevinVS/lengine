use std::collections::HashSet;

use itertools::izip;

use crate::map::WorldMap;
use crate::geometry::GeometryComponent;
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
    states: Vec<HashSet<String>>,
    geometry: Vec<Option<GeometryComponent>>,
    physics: Vec<Option<PhysicsComponent>>,
    graphics: Vec<Option<GraphicsComponent>>,
    animation: Vec<Option<AnimationComponent>>
}

impl World {
    pub fn new(map: WorldMap) -> World {
        World {
            map,
            player_id: None,
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

    pub fn geometry(&self) -> impl Iterator<Item = (usize, (&HashSet<String>, &GeometryComponent))> {
        izip!(self.states.iter(), self.geometry.iter()).enumerate()
            .filter(|e| e.1.1.is_some())
            .map(|e| (e.0, (e.1.0, e.1.1.as_ref().unwrap())))
    }

    pub fn geometry_mut(&mut self) -> impl Iterator<Item = (usize, (&mut HashSet<String>, &mut GeometryComponent))> {
        izip!(self.states.iter_mut(), self.geometry.iter_mut()).enumerate()
            .filter(|e| e.1.1.is_some())
            .map(|e| (e.0, (e.1.0, e.1.1.as_mut().unwrap())))
    }

    pub fn physics(&self) -> impl Iterator<Item = (usize, (&HashSet<String>, &GeometryComponent, &PhysicsComponent))> {
        izip!(self.states.iter(), self.geometry.iter(), self.physics.iter()).enumerate()
            .filter(|e| e.1.1.is_some() && e.1.2.is_some())
            .map(|e| (e.0, (e.1.0, e.1.1.as_ref().unwrap(), e.1.2.as_ref().unwrap())))
    }

    pub fn physics_mut(&mut self) -> impl Iterator<Item = (usize, (&mut HashSet<String>, &mut GeometryComponent, &mut PhysicsComponent))> {
        izip!(self.states.iter_mut(), self.geometry.iter_mut(), self.physics.iter_mut()).enumerate()
            .filter(|e| e.1.1.is_some() && e.1.2.is_some())
            .map(|e| (e.0, (e.1.0, e.1.1.as_mut().unwrap(), e.1.2.as_mut().unwrap())))
    }

    pub fn graphics(&self) -> impl Iterator<Item = (usize, (&HashSet<String>, &GeometryComponent, &GraphicsComponent))> {
        izip!(self.states.iter(), self.geometry.iter(), self.graphics.iter()).enumerate()
            .filter(|e| e.1.1.is_some() && e.1.2.is_some())
            .map(|e| (e.0, (e.1.0, e.1.1.as_ref().unwrap(), e.1.2.as_ref().unwrap())))
    }

    pub fn graphics_mut(&mut self) -> impl Iterator<Item = (usize, (&mut HashSet<String>, &mut GeometryComponent, &mut GraphicsComponent))> {
        izip!(self.states.iter_mut(), self.geometry.iter_mut(), self.graphics.iter_mut()).enumerate()
            .filter(|e| e.1.1.is_some() && e.1.2.is_some())
            .map(|e| (e.0, (e.1.0, e.1.1.as_mut().unwrap(), e.1.2.as_mut().unwrap())))
    }

    pub fn animations(&self) -> impl Iterator<Item = (usize, (&HashSet<String>,&GeometryComponent, &GraphicsComponent, &AnimationComponent))> {
        izip!(self.states.iter(), self.geometry.iter(), self.graphics.iter(), self.animation.iter()).enumerate()
            .filter(|e| e.1.1.is_some() && e.1.2.is_some() && e.1.3.is_some())
            .map(|e| (e.0, (e.1.0, e.1.1.as_ref().unwrap(), e.1.2.as_ref().unwrap(), e.1.3.as_ref().unwrap())))
    }

    pub fn animations_mut(&mut self) -> impl Iterator<Item = (usize, (&mut HashSet<String>, &mut GeometryComponent, &mut GraphicsComponent, &mut AnimationComponent))> {
        izip!(self.states.iter_mut(), self.geometry.iter_mut(), self.graphics.iter_mut(), self.animation.iter_mut())
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

    pub fn get_entity_geometry(&self, id: usize) -> Option<&GeometryComponent> {
        self.geometry[id].as_ref()
    }

    pub fn get_entity_geometry_mut(&mut self, id: usize) -> Option<&mut GeometryComponent> {
        self.geometry[id].as_mut()
    }

    pub fn get_entity_physics(&self, id: usize) -> (Option<&GeometryComponent>, Option<&PhysicsComponent>) {
        (self.geometry[id].as_ref(), self.physics[id].as_ref())
    }

    pub fn get_entity_physics_mut(&mut self, id: usize) -> (Option<&mut GeometryComponent>, Option<&mut PhysicsComponent>) {
        (self.geometry[id].as_mut(), self.physics[id].as_mut())
    }

    pub fn get_entity_graphics(&self, id: usize) -> (Option<&GeometryComponent>, Option<&GraphicsComponent>) {
        (self.geometry[id].as_ref(), self.graphics[id].as_ref())
    }

    pub fn get_entity_graphics_mut(&mut self, id: usize) -> (Option<&mut GeometryComponent>, Option<&mut GraphicsComponent>) {
        (self.geometry[id].as_mut(), self.graphics[id].as_mut())
    }

    pub fn get_entity_animations(&self, id: usize) -> (Option<&GeometryComponent>, Option<&GraphicsComponent>, Option<&AnimationComponent>) {
        (self.geometry[id].as_ref(), self.graphics[id].as_ref(), self.animation[id].as_ref())
    }

    pub fn get_entity_animations_mut(&mut self, id: usize) -> (Option<&mut GeometryComponent>, Option<&mut GraphicsComponent>, Option<&mut AnimationComponent>) {
        (self.geometry[id].as_mut(), self.graphics[id].as_mut(), self.animation[id].as_mut())
    }

    // Control Entity State
    pub fn add_entity_state(&mut self, id: usize, state: String) {
        self.states[id].insert(state);
    }

    pub fn remove_entity_state(&mut self, id: usize, state: &String) {
        self.states[id].remove(state);
    }
}
