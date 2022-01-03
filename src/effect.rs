use std::time::Instant;
use crate::{geometry::Rect, world::World};

#[derive(Debug, Clone)]
pub struct Effect {
    pub name: String,
    created: Instant,
    ttl: Option<f32>,
    pub rect: Rect
}

impl Effect {
    pub fn new(name: String, rect: Rect, ttl: Option<f32>) -> Effect {
        Effect {
            name,
            ttl,
            rect,
            created: Instant::now()
        }
    }

    pub fn finished(&self) -> bool {
        if self.ttl.is_none() { return false; }
        self.created.elapsed().as_secs_f32() > self.ttl.unwrap()
    }
}

pub struct EffectSystem;

impl EffectSystem {
    pub fn new() -> EffectSystem {
        EffectSystem {}
    }

    pub fn run(&mut self, world: &mut World) {
        world.effects = world.effects.iter()
            .filter(|e| !e.finished())
            .map(|e| e.clone())
            .collect();

        world.apply_effects();
    }
}
