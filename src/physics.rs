use sdl2::rect::Rect;
use std::f32::consts::FRAC_PI_2;
use std::f32::consts::PI;

use crate::{entity::Entity, vector::Vector, world::World};
use std::time::Instant;

pub struct EntityPhysicsState {
    pub depth: u32,
    pub velocity: Vector    // Velocity on an object, do not set directly
}

impl EntityPhysicsState {
    pub fn new(depth: u32) -> EntityPhysicsState {
        EntityPhysicsState {
            depth,
            velocity: Vector::zero()
        }
    }
}

// We keep track of everything physics related with forces,
// meaning we need to know the mass of every object to determine
// its acceleration and thus its velocity and its position for
// the next frame.
pub struct PhysicsSystem {
    last_tick: Instant
}

impl PhysicsSystem {
    pub fn new() -> PhysicsSystem {
        PhysicsSystem {
            last_tick: Instant::now()
        }
    }

    pub fn run(&mut self, world: &mut World) {
        // Sum all forces and calculate velocities
        let mut entities: Vec<&mut Entity> = world.physical_mut().collect();

        for i in 0..entities.len() {
            // Apply final velocities
            let t = self.last_tick.elapsed().as_secs_f32();
            let mut delta_vec = entities.get_mut(i).unwrap().physics().unwrap().velocity;


            let rect = entities.get_mut(i).unwrap().geometry().unwrap().rect();
            let mut new_rect = rect.clone();

            new_rect.x += delta_vec.x() as i32;
            new_rect.y += delta_vec.y() as i32;

            let new_left = new_rect.x();
            let new_right = new_rect.x() + new_rect.width() as i32;

            let new_top = new_rect.y();
            let new_bottom = new_rect.y() + new_rect.height() as i32;

            let rect_left = rect.x();
            let rect_right = rect.x() + rect.width() as i32;

            let rect_top = rect.y();
            let rect_bottom = rect.y() + rect.height() as i32;

            // Check and handle collisions
            for j in 0..entities.len() {
                if i==j {continue;}
                let other_rect = entities[j].geometry().unwrap().rect();

                if new_rect.has_intersection(other_rect) {
                    println!("crash");
                    let other_rect_left = other_rect.x();
                    let other_rect_right = other_rect.x() + other_rect.width() as i32;

                    let other_rect_top = other_rect.y();
                    let other_rect_bottom = other_rect.y() + other_rect.height() as i32;

                    let collide_left = rect_right < other_rect_left && new_right > other_rect_left;
                    let collide_right = rect_left > other_rect_right && new_left < other_rect_right;
                    let collide_top = rect_bottom < other_rect_top && new_bottom > other_rect_top;
                    let collide_bottom = rect_top > other_rect_bottom && new_top < other_rect_bottom;

                    if (collide_left || collide_right) && (collide_top || collide_bottom) {
                        delta_vec.mag = 0.0
                    }

                    if collide_left {
                        let y = delta_vec.y();
                        delta_vec.dir = y.signum() * FRAC_PI_2;
                        delta_vec.mag = y.abs();

                        entities.get_mut(i).unwrap().geometry_mut().unwrap().x = other_rect_left - rect.width() as i32 - 1;
                    }

                    if collide_right {
                        let y = delta_vec.y();
                        delta_vec.dir = y.signum() * FRAC_PI_2;
                        delta_vec.mag = y.abs();

                        entities.get_mut(i).unwrap().geometry_mut().unwrap().x = other_rect_right + 1;
                    }

                    if collide_top {
                        let x = delta_vec.x();
                        delta_vec.dir = (1.0-x.signum()) * PI;
                        delta_vec.mag = x.abs();

                        entities.get_mut(i).unwrap().geometry_mut().unwrap().y = other_rect_top - rect.height() as i32 - 1;
                    } else if collide_bottom {
                        let x = delta_vec.x();
                        delta_vec.dir = (1.0-x.signum()) * PI;
                        delta_vec.mag = x.abs();

                        entities.get_mut(i).unwrap().geometry_mut().unwrap().y = other_rect_bottom + 1;
                    }
                }

                entities.get_mut(i).unwrap().geometry_mut().unwrap().x += delta_vec.x() as i32;
                entities.get_mut(i).unwrap().geometry_mut().unwrap().y += delta_vec.y() as i32;
            }
        }

        self.last_tick = Instant::now();
    }
}
