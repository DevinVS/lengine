use std::collections::HashSet;
use std::collections::HashMap;

use itertools::izip;
use sdl2::pixels::Color;

use crate::geometry::PositionComponent;
use crate::physics::PhysicsComponent;
use crate::graphics::GraphicsComponent;
use crate::animation::AnimationComponent;
use crate::state::ActionComponent;
use crate::effect::Effect;
use crate::dialog::Dialog;
use crate::graphics::TextureManager;
use crate::parser::parse_world_file;

/// Struct containing all game data and current state
pub struct World<'a> {
    /// Texture Manager
    pub texture_manager: TextureManager<'a>,

    /// Possible world files, Name -> Path
    pub worlds: HashMap<String, String>,

    /// All effects in the game world
    pub effects: Vec<Effect>,

    /// All Dialogs
    pub dialogs: HashMap<String, Dialog>,

    /// Currently selected dialog index
    pub curr_dialog: Option<String>,

    /// Background texture and renderbox
    pub background: Option<GraphicsComponent>,
    pub background_color: Color,

    // Entity Components
    /// Array of sets of all the current active states for an entity
    pub states: Vec<HashSet<String>>,
    /// Array of optional position data for an entity
    pub positions: Vec<Option<PositionComponent>>,
    /// Array of optional physicis data for an entity
    pub physics: Vec<Option<PhysicsComponent>>,
    /// Array of optional graphics data for an entity
    pub graphics: Vec<Option<GraphicsComponent>>,
    /// Array of optional animation data for an entity
    pub animations: Vec<Option<AnimationComponent>>,
    /// Array of optoin actions data for an entity
    pub actions: Vec<Option<ActionComponent>>,
}

impl<'a> World<'a> {
    /// Create a new world
    pub fn new(texture_manager: TextureManager, worlds: HashMap<String, String>) -> World {
        World {
            texture_manager,
            worlds,
            states: Vec::new(),
            positions: Vec::new(),
            physics: Vec::new(),
            graphics: Vec::new(),
            animations: Vec::new(),
            actions: Vec::new(),
            effects: Vec::new(),
            dialogs: HashMap::new(),
            curr_dialog: None,
            background: None,
            background_color: Color::RGB(0, 0, 0)
        }
    }

    /// Add an entity to the entity manager
    pub fn add_entity(&mut self,
        position: Option<PositionComponent>,
        physics: Option<PhysicsComponent>,
        graphics: Option<GraphicsComponent>,
        animation: Option<AnimationComponent>,
        actions: Option<ActionComponent>
    ) -> usize {
        self.states.push(HashSet::new());
        self.positions.push(position);
        self.physics.push(physics);
        self.graphics.push(graphics);
        self.animations.push(animation);
        self.actions.push(actions);

        self.states.len()-1
    }

    /// Deload the current world
    pub fn deload(&mut self) {
        while self.states.len() > 1 {
            self.states.pop();
            self.positions.pop();
            self.physics.pop();
            self.graphics.pop();
            self.animations.pop();
            self.actions.pop();
        }

        self.dialogs.clear();
        self.effects.clear();
    }

    /// Load a world from a world file
    pub fn load(&mut self, name: &str, entrance: &str) {
        let path = self.worlds[name].clone();
        parse_world_file(&path, self, entrance);
    }

    /// Add a new Dialog to display
    pub fn add_dialog(&mut self, name: String, dialog: Dialog) {
        self.dialogs.insert(name, dialog);
    }

    /// Apply all effects to the objects who lie inside them
    pub fn apply_effects(&mut self) {
        for i in 0..self.states.len() {
            if self.positions[i].is_some() && self.physics[i].is_some() {
                let footprint = self.physics[i].as_ref().unwrap().hitbox
                    .after_position(&self.positions[i].as_ref().unwrap())
                    .after_depth(self.physics[i].as_ref().unwrap().depth);

                for j in 0..self.effects.len() {
                    let effect_rect = self.effects[j].rect;
                    let add = &self.effects[j].adds;
                    let remove = &self.effects[j].removes;

                    if footprint.has_intersection(effect_rect) {
                        for state in add {
                            self.states[i].insert(state.clone());
                        }

                        for state in remove {
                            self.states[i].remove(state);
                        }

                    }
                }
            }
        }
    }

    // Iterators over common properties of entities

    /// Iterator of entity states
    pub fn states(&self) -> impl Iterator<Item = &HashSet<String>> {
        self.states.iter()
    }

    /// Iterator of mutable entity states
    pub fn states_mut(&mut self) -> impl Iterator<Item = &mut HashSet<String>> {
        self.states.iter_mut()
    }

    /// Get the mutable state of a single entity
    pub fn get_states_mut(&mut self, id: usize) -> &mut HashSet<String> {
        &mut self.states[id]
    }

    /// Iterator of all mutable entity data
    pub fn all_mut(&mut self) -> impl Iterator<Item = (usize, (&mut HashSet<String>, Option<&mut PositionComponent>, Option<&mut PhysicsComponent>, Option<&mut GraphicsComponent>, Option<&mut AnimationComponent>, Option<&mut ActionComponent>))> {
        izip!(self.states.iter_mut(), self.positions.iter_mut(), self.physics.iter_mut(), self.graphics.iter_mut(), self.animations.iter_mut(), self.actions.iter_mut())
            .enumerate()
            .map(|e| (e.0, (e.1.0, e.1.1.as_mut(), e.1.2.as_mut(), e.1.3.as_mut(), e.1.4.as_mut(), e.1.5.as_mut())))
    }

    /// Iterator of entity position data
    pub fn positions(&self) -> impl Iterator<Item = (usize, (&HashSet<String>, &PositionComponent))> {
        izip!(self.states.iter(), self.positions.iter()).enumerate()
            .filter(|e| e.1.1.is_some())
            .map(|e| (e.0, (e.1.0, e.1.1.as_ref().unwrap())))
    }

    /// Iterator of mutable entity position data
    pub fn positions_mut(&mut self) -> impl Iterator<Item = (usize, (&mut HashSet<String>, &mut PositionComponent))> {
        izip!(self.states.iter_mut(), self.positions.iter_mut()).enumerate()
            .filter(|e| e.1.1.is_some())
            .map(|e| (e.0, (e.1.0, e.1.1.as_mut().unwrap())))
    }

    /// Iterator of entity physics data
    pub fn physics(&self) -> impl Iterator<Item = (usize, (&HashSet<String>, &PositionComponent, &PhysicsComponent))> {
        izip!(self.states.iter(), self.positions.iter(), self.physics.iter()).enumerate()
            .filter(|e| e.1.1.is_some() && e.1.2.is_some())
            .map(|e| (e.0, (e.1.0, e.1.1.as_ref().unwrap(), e.1.2.as_ref().unwrap())))
    }

    /// Iterator of mutable entity physics data
    pub fn physics_mut(&mut self) -> impl Iterator<Item = (usize, (&mut HashSet<String>, &mut PositionComponent, &mut PhysicsComponent))> {
        izip!(self.states.iter_mut(), self.positions.iter_mut(), self.physics.iter_mut()).enumerate()
            .filter(|e| e.1.1.is_some() && e.1.2.is_some())
            .map(|e| (e.0, (e.1.0, e.1.1.as_mut().unwrap(), e.1.2.as_mut().unwrap())))
    }

    /// Iterator of entity graphics data
    pub fn graphics(&self) -> impl Iterator<Item = (usize, (&HashSet<String>, &PositionComponent, &GraphicsComponent))> {
        izip!(self.states.iter(), self.positions.iter(), self.graphics.iter()).enumerate()
            .filter(|e| e.1.1.is_some() && e.1.2.is_some())
            .map(|e| (e.0, (e.1.0, e.1.1.as_ref().unwrap(), e.1.2.as_ref().unwrap())))
    }

    /// Iterator of mutable entity graphics data
    pub fn graphics_mut(&mut self) -> impl Iterator<Item = (usize, (&mut HashSet<String>, &mut PositionComponent, &mut GraphicsComponent))> {
        izip!(self.states.iter_mut(), self.positions.iter_mut(), self.graphics.iter_mut()).enumerate()
            .filter(|e| e.1.1.is_some() && e.1.2.is_some())
            .map(|e| (e.0, (e.1.0, e.1.1.as_mut().unwrap(), e.1.2.as_mut().unwrap())))
    }

    /// Iterator of entity animations data
    pub fn animations(&self) -> impl Iterator<Item = (usize, (&HashSet<String>,&PositionComponent, &GraphicsComponent, &AnimationComponent))> {
        izip!(self.states.iter(), self.positions.iter(), self.graphics.iter(), self.animations.iter()).enumerate()
            .filter(|e| e.1.1.is_some() && e.1.2.is_some() && e.1.3.is_some())
            .map(|e| (e.0, (e.1.0, e.1.1.as_ref().unwrap(), e.1.2.as_ref().unwrap(), e.1.3.as_ref().unwrap())))
    }

    /// Iterator of mutable animations data
    pub fn animations_mut(&mut self) -> impl Iterator<Item = (usize, (&mut HashSet<String>, &mut PositionComponent, &mut GraphicsComponent, &mut AnimationComponent))> {
        izip!(self.states.iter_mut(), self.positions.iter_mut(), self.graphics.iter_mut(), self.animations.iter_mut())
            .enumerate()
            .filter(|e| e.1.1.is_some() && e.1.2.is_some() && e.1.3.is_some())
            .map(|e| (e.0, (e.1.0, e.1.1.as_mut().unwrap(), e.1.2.as_mut().unwrap(), e.1.3.as_mut().unwrap())))
    }

    // Individual selectors for common properties of entities
    /// Get set of states of a single entity
    pub fn get_entity_states(&self, id: usize) -> &HashSet<String> {
        &self.states[id]
    }

    /// Get a mutable set of states of a single entity
    pub fn get_entity_states_mut(&mut self, id: usize) -> &mut HashSet<String> {
        &mut self.states[id]
    }

    /// Get Position data for a single entity
    pub fn get_entity_positions(&self, id: usize) -> Option<&PositionComponent> {
        self.positions[id].as_ref()
    }

    /// Get mutable position data for a single entity
    pub fn get_entity_positions_mut(&mut self, id: usize) -> Option<&mut PositionComponent> {
        self.positions[id].as_mut()
    }

    /// Get physics data for a single entity
    pub fn get_entity_physics(&self, id: usize) -> (Option<&PositionComponent>, Option<&PhysicsComponent>) {
        (self.positions[id].as_ref(), self.physics[id].as_ref())
    }

    /// Get mutable physics data for a single entity
    pub fn get_entity_physics_mut(&mut self, id: usize) -> (Option<&mut PositionComponent>, Option<&mut PhysicsComponent>) {
        (self.positions[id].as_mut(), self.physics[id].as_mut())
    }

    /// Get graphics data for a single entity
    pub fn get_entity_graphics(&self, id: usize) -> (Option<&PositionComponent>, Option<&GraphicsComponent>) {
        (self.positions[id].as_ref(), self.graphics[id].as_ref())
    }

    /// Get mutable graphics data for single entity
    pub fn get_entity_graphics_mut(&mut self, id: usize) -> (Option<&mut PositionComponent>, Option<&mut GraphicsComponent>) {
        (self.positions[id].as_mut(), self.graphics[id].as_mut())
    }

    /// Get animation data for a single entity
    pub fn get_entity_animations(&self, id: usize) -> (Option<&PositionComponent>, Option<&GraphicsComponent>, Option<&AnimationComponent>) {
        (self.positions[id].as_ref(), self.graphics[id].as_ref(), self.animations[id].as_ref())
    }

    /// Get mutable animation data for a single entity
    pub fn get_entity_animations_mut(&mut self, id: usize) -> (Option<&mut PositionComponent>, Option<&mut GraphicsComponent>, Option<&mut AnimationComponent>) {
        (self.positions[id].as_mut(), self.graphics[id].as_mut(), self.animations[id].as_mut())
    }

    // Control Entity State

    /// Add a state to a single entity
    pub fn add_entity_state(&mut self, id: usize, state: String) {
        self.states[id].insert(state);
    }

    /// Remove a state from a single entity
    pub fn remove_entity_state(&mut self, id: usize, state: &String) {
        self.states[id].remove(state);
    }
}
