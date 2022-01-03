use std::collections::HashMap;
use std::time::Instant;

use crate::world::World;
use crate::actions::Action;

/// A sequence of actions, to be run in order after specified delays
pub struct Sequence {
    /// Last time the curr_index switched
    last_switch: Instant,
    /// List of (delay, action) pairs.
    ///
    /// Each action is only be run after a certain delay
    /// from the previous action. Thus if we had
    ///     [(1.0, act1), (1.0, act2)]
    /// we would expect act1 to run after 1 seconds and
    /// act2 to run after 2 seconds
    actions: Vec<(f32, Box<dyn Action>)>,
    /// Current selected action
    curr_index: usize
}


impl Sequence {
    /// Create a new Sequence
    pub fn new(actions: Vec<(f32, Box<dyn Action>)>) -> Sequence {
        Sequence {
            last_switch: Instant::now(),
            actions,
            curr_index: 0
        }
    }

    /// Get the current action
    pub fn current(&mut self) -> &mut Box<dyn Action> {
        &mut self.actions[self.curr_index].1
    }

    /// Check the current delay, moving the index only after the delay has passed
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


/// Data about triggers and actions for a specific entity
pub struct ActionComponent {
    /// Map of states and the sequence of actions to run for that state
    actions: HashMap<String, Sequence>
}

impl ActionComponent {
    /// Create a new ActionComponent
    pub fn new(actions: HashMap<String, Sequence>) -> ActionComponent {
        ActionComponent {
            actions
        }
    }

    /// Get an action for a specific state
    pub fn get_mut(&mut self, state: &String) -> Option<&mut Sequence> {
        self.actions.get_mut(state)
    }
}

/// Handles pairing specific states to triggers
pub struct StateSystem {}

impl StateSystem {
    /// Create a new StateSystem
    pub fn new() -> StateSystem {
        StateSystem {}
    }

    /// For each entity in the world, run the sequences that correspond to their current states
    pub fn run(&mut self, world: &mut World) {
        for i in 0..world.states.len() {
            if world.actions[i].is_some() {
                for state in world.states[i].clone().iter() {
                    if let Some(sequence) = world.actions[i].as_mut().unwrap().get_mut(state) {
                        sequence.current().tick(&mut world.states[i], &mut world.effects, &mut world.curr_dialog);
                        sequence.tick();
                    }
                }
            }
        }
    }
}
