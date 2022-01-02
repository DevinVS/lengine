use crate::{world::World, physics::PhysicsComponent, graphics::GraphicsComponent, animation::AnimationComponent, geometry::PositionComponent};
use std::{collections::{HashMap, HashSet}, time::Instant};
use crate::actions::Action;

pub struct Sequence {
    last_switch: Instant,
    actions: Vec<(f32, Box<dyn Action>)>,
    curr_index: usize
}


impl Sequence {
    pub fn new(actions: Vec<(f32, Box<dyn Action>)>) -> Sequence {
        Sequence {
            last_switch: Instant::now(),
            actions,
            curr_index: 0
        }
    }

    pub fn current(&mut self) -> &mut Box<dyn Action> {
        &mut self.actions[self.curr_index].1
    }

    pub fn tick(&mut self) {
        if self.last_switch.elapsed().as_secs_f32() >= self.actions[self.curr_index].0 {
            if self.curr_index == self.actions.len()-1 {
                self.curr_index = 0;
            } else {
                self.curr_index += 1;
            }
        }
    }
}


pub struct ActionComponent {
    actions: HashMap<String, Sequence>
}

impl ActionComponent {
    pub fn get_mut(&mut self, state: &String) -> Option<&mut Sequence> {
        self.actions.get_mut(state)
    }
}

/// Handles pairing specific states to triggers
pub struct StateSystem {}

impl StateSystem {
    pub fn new() -> StateSystem {
        StateSystem {}
    }

    pub fn tick(&mut self, world: &mut World) {
        for (_, (states, position, physics, graphics, animations, actions)) in world.all_mut() {
            if let Some(actions) = actions {
                for state in states.clone().iter() {
                    if let Some(sequence) = actions.get_mut(state) {
                        sequence.current().tick(states, &position, &physics, &graphics, &animations);
                        sequence.tick();
                    }
                }
            }
        }
    }
}
