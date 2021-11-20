use crate::{entity::Entity, vector::Vector, world::World};
use std::{f32::consts::PI, time::Instant};

// Constants, all in metric
const PIX_PER_M: f32 = 10.0;  // Pixels per Meter
const G: f32 = 9.8; // gravity
const MU_K: f32 = 0.3;  // Coefficient of kinetic friction
const MU_S: f32 = 0.5;  // Coefficient of static friction

pub struct EntityPhysicsState {
    pub mass: f32,      // Mass of the current object
    pub forces: Vector, // Sum of forces being applied on this object
    pub velocity: Vector    // Velocity on an object, do not set directly
}

impl EntityPhysicsState {
    pub fn new(mass: f32) -> EntityPhysicsState {
        EntityPhysicsState {
            mass,
            forces: Vector::zero(),
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

    fn apply_friction(entity: &mut Entity) {
        let physics_state = entity.physics_state.as_mut().unwrap();

        let friction = if physics_state.velocity.mag < 0.1 {
            Vector::zero()
        } else if physics_state.velocity.mag.abs() < 0.1 {
            // Static Friction
            Vector::new(
                physics_state.velocity.dir - PI,
                G*physics_state.mass*MU_S
            )
        } else {
            // Kinetic Friction
            Vector::new(
                physics_state.velocity.dir - PI,
                G*physics_state.mass*MU_K
            )
        };

        physics_state.forces += friction;
    }

    fn calculate_velocity(&self, entity: &mut Entity) {
        let physics_state = entity.physics_state.as_mut().unwrap();
        let a = physics_state.forces / physics_state.mass;
        let t = self.last_tick.elapsed().as_secs_f32();

        physics_state.velocity = physics_state.velocity + a*t;
    }

    fn calculate_position(&self, entity: &mut Entity) {
        let physics_state = entity.physics_state.as_ref().unwrap();
        let t = self.last_tick.elapsed().as_secs_f32();
        
        entity.x += (physics_state.velocity.x()) as i32;
        entity.y += (physics_state.velocity.y()) as i32;
    }

    pub fn run(&mut self, world: &mut World) {
        // Sum all forces and calculate velocities
        let mut entities: Vec<&mut Entity> = world.physical_mut().collect();

        for i in 0..entities.len() {
            PhysicsSystem::apply_friction(entities[i]);
            self.calculate_velocity(entities[i]);
            self.calculate_position(entities[i]);

            // Check and handle collisions
            for j in 0..entities.len() {
                if i==j {continue;}

                if entities[i].rect().has_intersection(entities[j].rect()) {
                    // Undo velocity
                    let vel1 = entities[i].physics_state.as_ref().unwrap().velocity;
                    let vel2 = entities[j].physics_state.as_ref().unwrap().velocity;

                    entities[i].x -= vel1.x() as i32;
                    entities[i].y -= vel1.y() as i32;
                    entities[j].x -= vel2.x() as i32;
                    entities[j].y -= vel2.y() as i32;

                    // Apply Conservation of momentum
                    let vel = {
                        let physics_1 = entities[i].physics_state.as_ref().unwrap();
                        let physics_2 = entities[j].physics_state.as_ref().unwrap();

                        let total_mass = physics_1.mass + physics_2.mass;
                        let total_momentum = physics_1.velocity*physics_1.mass + physics_2.velocity*physics_2.mass;
                        total_momentum / total_mass
                    };
                    entities[i].physics_state.as_mut().unwrap().velocity = vel;
                    entities[j].physics_state.as_mut().unwrap().velocity = vel;

                    // redo velocity calculation
                    entities[i].x += vel.x() as i32;
                    entities[i].y += vel.y() as i32;
                    entities[j].x += vel.x() as i32;
                    entities[j].y += vel.y() as i32;
                }
            }

            entities[i].physics_state.as_mut().unwrap().forces = Vector::zero();
        }

        self.last_tick = Instant::now();
    }
}