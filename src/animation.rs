use crate::world::World;
use std::time::Instant;
use std::collections::HashMap;

/// A Graphical Animation across multiple textures
#[derive(Debug)]
pub struct Animation {
    /// List of animation states: (texture_id, texture rectangle)
    states: Vec<(usize, Option<sdl2::rect::Rect>)>,
    /// Time between each state change
    period: f32,
    /// Current state index
    curr_tex_index: usize,
    /// Time that the state last changed
    last_switch: Instant
}

impl Animation {
    /// Create a new Animation
    pub fn new(states: Vec<(usize, Option<sdl2::rect::Rect>)>, period: f32) -> Animation {
        Animation {
            states,
            period,
            curr_tex_index: 0,
            last_switch: Instant::now()
        }
    }

    /// Check if the time since the last switch has exceeded the period
    /// and switch to the next state if so
    fn tick(&mut self) {
        if self.last_switch.elapsed().as_secs_f32() > self.period {
            if self.curr_tex_index == self.states.len()-1 {
                self.curr_tex_index = 0;
            } else {
                self.curr_tex_index += 1;
            }

            self.last_switch = Instant::now();
        }
    }

    /// The current texture id
    fn current_texture(&self) -> usize {
        self.states[self.curr_tex_index].0
    }

    /// The current src rectangle
    fn current_srcbox(&self) -> Option<sdl2::rect::Rect> {
        self.states[self.curr_tex_index].1
    }
}

/// Animation state for a single Entity
#[derive(Debug)]
pub struct AnimationComponent {
    /// Dictionary of states to Animations.
    /// Each animation has to finish before another can be selected
    /// based on the states of the entity
    animations: HashMap<String, Animation>,
    /// Currently selected animation's key
    curr_key: Option<String>
}

impl AnimationComponent {
    /// Create a new AnimationComponent
    pub fn new(animations: HashMap<String, Animation>) -> AnimationComponent {
        AnimationComponent {
            animations,
            curr_key: None
        }
    }

    /// Get an animation by its state
    pub fn get(&self, key: &String) -> Option<&Animation> {
        self.animations.get(key)
    }

    /// Get a mutable animation by its state
    pub fn get_mut(&mut self, key: &String) -> Option<&mut Animation> {
        self.animations.get_mut(key)
    }

    /// Get the currently running animation
    pub fn current(&self) -> Option<&Animation> {
        if self.curr_key.is_none() { return None; }

        self.get(self.curr_key.as_ref().unwrap())
    }

    /// Get the currently running animation mutably
    pub fn current_mut(&mut self) -> Option<&mut Animation> {
        if self.curr_key.is_none() { return None; }

        let key = self.curr_key.as_ref().unwrap().clone();

        self.get_mut(&key)
    }
}

pub struct AnimationSystem {}

impl AnimationSystem {
    pub fn new() -> AnimationSystem {
        AnimationSystem {}
    }

    /// Play the most relevant animations based on state
    pub fn run(&mut self, world: &mut World) {
        for (_, (states, _, graphics, animations)) in world.animations_mut() {
            // Find the state which determines the animation
            for state in states.iter() {
                if let Some(animation) = animations.get_mut(state) {

                    animation.tick();
                    graphics.texture_id = animation.current_texture();
                    graphics.srcbox = animation.current_srcbox();
                    animations.curr_key = Some(state.clone());
                    break;
                }
            }
        }
    }
}
