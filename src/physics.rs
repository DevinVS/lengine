use std::f32::consts::FRAC_PI_2;

use crate::{entity::Entity, vector::Vector, world::World};
use std::time::Instant;

pub struct PhysicsComponent {
    pub depth: u32,
    pub velocity: Vector    // Velocity on an object, do not set directly
}

impl PhysicsComponent {
    pub fn new(depth: u32) -> PhysicsComponent {
        PhysicsComponent {
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
            let mut delta_vec = entities.get_mut(i).unwrap().physics().unwrap().velocity * t;

            let rect = entities.get_mut(i).unwrap().geometry().unwrap().rect().clone();

            let depth = entities.get(i).unwrap().physics().unwrap().depth;
            let mut footprint = rect.clone();
            footprint.y += footprint.h as f32 - depth as f32;
            footprint.h = depth;

            let mut after_x = footprint.clone();
            let mut after_y = footprint.clone();

            after_x.x += delta_vec.x();
            after_y.y += delta_vec.y();

            // Check and handle collisions
            for j in 0..entities.len() {
                // If we are compareing the same rectangle skip
                if i==j {continue;}

                let other_rect = entities[j].geometry().unwrap().rect();
                let other_depth = entities[j].physics().unwrap().depth;

                let mut other_footprint = other_rect.clone();
                other_footprint.y += other_footprint.h as f32 - other_depth as f32;
                other_footprint.h = other_depth;

                let x_collision = after_x.has_intersection(other_footprint);
                let y_collision = after_y.has_intersection(other_footprint);

                if x_collision && y_collision {
                    delta_vec.mag = 0.0;
                } else if x_collision {
                    delta_vec.mag *= delta_vec.dir.sin();
                    delta_vec.dir = FRAC_PI_2;
                } else if y_collision {
                    delta_vec.mag *= delta_vec.dir.cos();
                    delta_vec.dir = 0.0;
                }
            }

            entities.get_mut(i)
                .unwrap()
                .geometry_mut()
                .unwrap()
                .rect_mut()
                .after_vector(delta_vec);
        }

        self.last_tick = Instant::now();
    }
}
