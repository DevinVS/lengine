use std::f32::consts::FRAC_PI_2;

use crate::{vector::Vector, world::World, geometry::GeometryComponent};
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
        let mut entities: Vec<(usize, (_, &mut GeometryComponent, &mut PhysicsComponent))> = world.physics_mut().collect();

        for i in 0..entities.len() {
            // Apply final velocities
            let t = self.last_tick.elapsed().as_secs_f32();
            let mut delta_vec = entities[i].1.2.velocity * t;

            let rect = entities[i].1.1.rect().clone();

            let depth = entities[i].1.2.depth;
            let footprint = rect.clone().after_depth(depth);

            let mut after_x = footprint.clone();
            let mut after_y = footprint.clone();

            after_x.x += delta_vec.x();
            after_y.y += delta_vec.y();

            // Check and handle collisions
            for j in 0..entities.len() {
                // If we are compareing the same rectangle skip
                if i==j {continue;}

                let other_rect = entities[j].1.1.rect().clone();
                let other_depth = entities[j].1.2.depth;

                let other_footprint = other_rect.clone().after_depth(other_depth);

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

            entities[i].1.1.rect_mut().apply_vector(delta_vec);
        }

        self.last_tick = Instant::now();
    }
}
