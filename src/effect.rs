use std::time::Instant;
use crate::{geometry::Rect, world::World};

#[derive(Debug, Clone)]
/// An area effect inside the world
/// Any entity inside this effect will gain
/// the state which is the effect's name
pub struct Effect {
    /// Name of the effect and the state it gives
    pub name: String,
    /// Time effect was created
    created: Instant,
    /// Time that the effect lasts, in seconds
    ttl: Option<f32>,
    /// Rectangle for which the effect is affective
    pub rect: Rect
}

impl Effect {
    /// Create a new Effect
    pub fn new(name: String, rect: Rect, ttl: Option<f32>) -> Effect {
        Effect {
            name,
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
        world.effects = world.effects.iter()
            .filter(|e| !e.finished())
            .map(|e| e.clone())
            .collect();

        world.apply_effects();
    }
}
