use crate::world::World;
use std::time::Instant;
use std::collections::HashMap;

pub struct Animation {
    textures: Vec<usize>,
    period: f32,
    curr_tex_index: usize,
    last_switch: Instant,
    priority: usize
}

impl Animation {
    pub fn new(textures: Vec<usize>, period: f32, priority: usize) -> Animation {
        Animation {
            textures,
            period,
            curr_tex_index: 0,
            last_switch: Instant::now(),
            priority
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
}

pub struct AnimationComponent {
    animations: HashMap<String, Animation>
}

impl AnimationComponent {
    pub fn new(animations: HashMap<String, Animation>) -> AnimationComponent {
        AnimationComponent {
            animations
        }
    }
}

pub struct AnimationSystem {}

impl AnimationSystem {
    pub fn new() -> AnimationSystem {
        AnimationSystem {}
    }

    pub fn run(&mut self, world: &mut World) {
        for entity in world.animatable_mut() {
            // Finish the currently running animation
            for animation in entity.animation_mut().unwrap().animations.values_mut() {
                if animation.curr_tex_index != 0 {
                    animation.tick();
                    let tex_id = animation.textures[animation.curr_tex_index];
                    entity.graphics_mut().unwrap().texture_id = tex_id;
                    return;
                }
            }

            let states = entity.states().clone();

            let mut priority = None;
            let mut tex_id = None;

            // Run the animation with the highest priority
            for state in states {
                if let Some(animation) = entity.animation_mut().unwrap().animations.get_mut(&state) {
                    if priority.is_none() || animation.priority > priority.unwrap() {
                        priority = Some(animation.priority);
                        animation.tick();
                        tex_id = Some(animation.textures[animation.curr_tex_index]);
                    }
                }
            }

            // Apply next texture index
            if let Some(tex_id) = tex_id {
                entity.graphics_mut().unwrap().texture_id = tex_id;
            }
        }
    }
}
