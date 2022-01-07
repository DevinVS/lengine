use std::time::Instant;
use crate::{geometry::Rect, world::World};

#[derive(Debug, Clone)]
pub struct EffectSpawner {
    adds: Vec<String>,
    removes: Vec<String>,
    ttl: Option<f32>,
    rect: Rect
}

impl EffectSpawner {
    pub fn new(adds: Vec<String>, removes: Vec<String>, rect: Rect, ttl: Option<f32>) -> EffectSpawner {
        EffectSpawner {
            adds,
            removes,
            ttl,
            rect
        }
    }

    pub fn spawn(&self) -> Effect {
        Effect::new(self.adds.clone(), self.removes.clone(), self.rect, self.ttl)
    }
}

#[derive(Debug, Clone)]
/// An area effect inside the world
/// Any entity inside this effect will gain
/// the state which is the effect's name
pub struct Effect {
    /// Name of the states this effect adds
    pub adds: Vec<String>,
    /// Name of the states this effect removes
    pub removes: Vec<String>,
    /// Time effect was created
    created: Instant,
    /// Time that the effect lasts, in seconds
    ttl: Option<f32>,
    /// Rectangle for which the effect is affective
    pub rect: Rect
}

impl Effect {
    /// Create a new Effect
    pub fn new(adds: Vec<String>, removes: Vec<String>, rect: Rect, ttl: Option<f32>) -> Effect {
        Effect {
            adds,
            removes,
            ttl,
            rect,
            created: Instant::now()
        }
    }

    /// Check if the effect has finished
    pub fn finished(&self) -> bool {
        if self.ttl.is_none() { return false; }
        self.created.elapsed().as_secs_f32() > self.ttl.unwrap()
    }
}

pub struct EffectSystem;

impl EffectSystem {
    /// Create a new EffectSystem
    pub fn new() -> EffectSystem {
        EffectSystem {}
    }

    /// Remove any effects in the world which
    /// have finished, then apply the appropriate states
    /// to every entity inside each effect
    pub fn run(&mut self, world: &mut World) {
        world.apply_effects();

        world.effects = world.effects.iter()
            .filter(|e| !e.finished())
            .map(|e| e.clone())
            .collect();
    }
}
