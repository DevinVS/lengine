use crate::world::World;
use std::time::Instant;
use std::collections::HashMap;

pub struct Animation {
    textures: Vec<usize>,
    period: f32,
    curr_tex_index: usize,
    last_switch: Instant
}

impl Animation {
    pub fn new(textures: Vec<usize>, period: f32) -> Animation {
        Animation {
            textures,
            period,
            curr_tex_index: 0,
            last_switch: Instant::now()
        }
    }

    fn tick(&mut self) {
        if self.last_switch.elapsed().as_secs_f32() > self.period {
            if self.curr_tex_index == self.textures.len()-1 {
                self.curr_tex_index = 0;
            } else {
                self.curr_tex_index += 1;
            }

            self.last_switch = Instant::now();
        }
    }

    fn current_texture(&self) -> usize {
        self.textures[self.curr_tex_index]
    }
}

pub struct AnimationComponent {
    animations: HashMap<String, Animation>,
    curr_key: Option<String>
}

impl AnimationComponent {
    pub fn new(animations: HashMap<String, Animation>) -> AnimationComponent {
        AnimationComponent {
            animations,
            curr_key: None
        }
    }

    pub fn get(&self, key: &String) -> Option<&Animation> {
        self.animations.get(key)
    }

    pub fn get_mut(&mut self, key: &String) -> Option<&mut Animation> {
        self.animations.get_mut(key)
    }

    pub fn current(&self) -> Option<&Animation> {
        if self.curr_key.is_none() { return None; }

        self.get(self.curr_key.as_ref().unwrap())
    }

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

    pub fn run(&mut self, world: &mut World) {
        for (_, (states, _, graphics, animations)) in world.animations_mut() {
            // If an animation is currently playing, play it
            if let Some(animation) = animations.current_mut() {
                animation.tick();
                graphics.texture_id = animation.current_texture();

                if animation.curr_tex_index == 0 {
                    animations.curr_key = None;
                }

                continue;
            }

            // Else find the state which determines the animation
            for state in states.iter() {
                if let Some(animation) = animations.get_mut(state) {

                    animation.tick();
                    graphics.texture_id = animation.current_texture();
                    animations.curr_key = Some(state.clone());
                    break;
                }
            }
        }
    }
}
