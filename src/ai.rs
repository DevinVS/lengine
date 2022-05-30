use std::ops::Mul;
use std::time::Instant;
use std::collections::HashSet;

use crate::world::World;
use crate::physics::PhysicsComponent;
use crate::geometry::PositionComponent;

static PID: usize = 0;
static MID: usize = 1;

pub struct AISystem {
    last_aggro: Instant,
    idle_path: Vec<(f32, f32, f32)>,
    next_idle: usize,
    last_idle_time: Instant,
    aggro_distance: f32,
    lost_delay: f32,
    last_pathfind: Instant,
    monster_world: String,
    teleport_timer: Instant,
    awaiting_teleport: bool,
    teleport_location: (f32, f32),
    monster_lake_pos: (f32, f32)
}


impl AISystem {
    pub fn new(idle_path: Vec<(f32, f32, f32)>, aggro_distance: f32, lost_delay: f32) -> Self {
        Self {
            last_aggro: Instant::now(),
            idle_path,
            next_idle: 0,
            last_idle_time: Instant::now(),
            aggro_distance,
            lost_delay,
            last_pathfind: Instant::now(),
            monster_world: "lake".into(),
            teleport_timer: Instant::now(),
            awaiting_teleport: false,
            teleport_location: (0.0, 0.0),
            monster_lake_pos: (0.0, 0.0)
        }
    }

    pub fn run(&mut self, world: &mut World) {
        // Check if monster needs to be loaded back into lake world
        if self.monster_world == "lake" && world.current_world == "lake" && world.positions[MID].is_none() {
            world.positions[MID] = Some(PositionComponent::new(self.monster_lake_pos.0, self.monster_lake_pos.1));
        }

        // Check if player has moved to new world
        if self.monster_world != world.current_world {
            // If the new world is the lake, restore the monsters position
            if world.current_world == "lake" {
                self.monster_world = "lake".into();
                world.positions[MID] = Some(PositionComponent::new(self.monster_lake_pos.0, self.monster_lake_pos.1));
            } else if world.positions[MID].is_some() {
                // Save monster lake position
                self.monster_lake_pos = {
                    let pos = world.positions[MID].as_ref().unwrap();
                    (pos.x, pos.y)
                };

                // If we are aggroed then we teleport after them,
                // else teleport to nearest idle location
                if world.states[MID].contains("aggro") {
                    self.awaiting_teleport = true;
                    self.teleport_location = {
                        let rect = world.physics[PID].as_ref().unwrap().hitbox
                            .after_position(world.positions[PID].as_ref().unwrap())
                            .after_depth(world.physics[PID].as_ref().unwrap().depth);
                        (rect.x, rect.y)
                    };
                    self.teleport_timer = Instant::now();
                    self.monster_world = world.current_world.clone();
                } else {
                    let mindex = self.idle_path.iter()
                        .enumerate()
                        .map(|(i, (x, y, _))| {
                            (i, ((y-self.monster_lake_pos.1).powi(2) + (x-self.monster_lake_pos.0).powi(2)).sqrt())
                        })
                    .min_by(|a, b| {
                        a.1.partial_cmp(&b.1).unwrap()
                    }).unwrap().0;

                    self.monster_lake_pos.0 = self.idle_path[mindex].0;
                    self.monster_lake_pos.1 = self.idle_path[mindex].1;
                    self.next_idle = (mindex + 1) % self.idle_path.len();
                    self.last_idle_time = Instant::now();
                }

                // Remove monster from the world (temporarily)
                world.positions[MID] = None;
                println!("Remove position");
            }
        }

        // If we are awaiting a teleport skip ahead,
        // else teleport the monster to the teleport location
        if self.awaiting_teleport && self.teleport_timer.elapsed().as_secs_f32() < 5.0 {
            return;
        } else if self.awaiting_teleport {
            self.awaiting_teleport = false;

            let height = world.physics[MID].as_ref().unwrap().hitbox.h;

            // Add monster back into the world at the correct location
            world.positions[MID] = Some(PositionComponent::new(self.teleport_location.0, self.teleport_location.1 - height as f32));
        }

        // Check if can see player, if so set aggro to true, if aggro, then lost
        if world.current_world == self.monster_world {
            if self.player_visible(world) {
                let (x, y) = {
                    let pos = world.positions[PID].as_ref().unwrap();
                    (pos.x, pos.y)
                };

                if self.dist(world, x, y) < self.aggro_distance {
                    world.states[MID].remove("lost");
                    world.states[MID].remove("idle");
                    world.states[MID].insert("aggro".into());
                }
            } else if world.states[MID].contains("aggro") {
                self.last_aggro = Instant::now();
                world.states[MID].remove("aggro");
                world.states[MID].insert("lost".into());
            }
        }


        if world.states[MID].contains("idle") {
            if world.current_world == "lake" && self.monster_world == "lake" {
                // Normal idle movement in the lake world
                let (dest_x, dest_y, _) = self.idle_path[self.next_idle];
                if self.dist(world, dest_x, dest_y) < 2.0 {
                    self.next_idle += 1;
                    self.next_idle %= self.idle_path.len();
                    self.last_idle_time = Instant::now();
                    return;
                }

                self.goto(world, dest_x, dest_y, 60.0);
            } else if world.current_world != "lake" && self.monster_world != "lake" {
                // Monster move back to teleport point, then deload
                let dist = self.dist(world, self.teleport_location.0, self.teleport_location.1);

                if dist < 2.0 {
                    world.positions[MID] = None;
                    self.monster_world = "lake".into();
                } else {
                    self.goto(world, self.teleport_location.0, self.teleport_location.1, 60.0);
                }
            } else {
                // Monster is in lake world while player is in room, simulate idle movement
                if self.sim_dist() < 2.0 {
                    self.next_idle += 1;
                    self.next_idle %= self.idle_path.len();
                    self.last_idle_time = Instant::now();
                }

                // Linear interpolation between idle points based on idle time
                let t = self.last_idle_time.elapsed().as_secs_f32() / self.idle_path[self.next_idle].2;

                let last_index = (self.next_idle - 1 + self.idle_path.len()) % self.idle_path.len();

                let last_x = self.idle_path[last_index].0;
                let last_y = self.idle_path[last_index].1;
                let x = self.idle_path[self.next_idle].0;
                let y = self.idle_path[self.next_idle].1;

                let delta_x = x-last_x;
                let delta_y = y-last_y;

                self.monster_lake_pos.0 = last_x + delta_x*t;
                self.monster_lake_pos.1 = last_y + delta_y*t;
            }
        } else if world.states[MID].contains("aggro") {
            let (x, y) = {
                let rect = world.physics[PID].as_ref().unwrap().hitbox
                    .after_position(world.positions[PID].as_ref().unwrap())
                    .after_depth(world.physics[PID].as_ref().unwrap().depth);
                (rect.x, rect.y)
            };

            let speed = 54.0 + 20.0 * self.last_pathfind.elapsed().as_secs_f32().mul(5.0).sin();

            self.goto(world, x, y, speed);
        } else if world.states[MID].contains("lost") {
            // Wait, and then return to idle
            self.stop(world);
            if self.last_aggro.elapsed().as_secs_f32() > self.lost_delay {
                world.states[MID].remove("lost");
                world.states[MID].insert("idle".into());

                let mindex = self.idle_path.iter()
                    .enumerate()
                    .map(|(i, (x, y, _))| {
                        (i, ((y-self.monster_lake_pos.1).powi(2) + (x-self.monster_lake_pos.0).powi(2)).sqrt())
                    })
                .min_by(|a, b| {
                    a.1.partial_cmp(&b.1).unwrap()
                }).unwrap().0;

                self.next_idle = (mindex + 1) % self.idle_path.len();
                self.last_idle_time = Instant::now();
            }
        }
    }

    fn sim_dist(&self) -> f32 {
        let x0 = self.idle_path[self.next_idle].0;
        let y0 = self.idle_path[self.next_idle].1;
        let x1 = self.monster_lake_pos.0;
        let y1 = self.monster_lake_pos.1;

        ((y1-y0).powi(2) + (x1-x0).powi(2)).sqrt()
    }

    fn player_visible(&mut self, world: &mut World) -> bool {
        let entities: Vec<(usize, (&mut HashSet<String>, &mut PositionComponent, &mut PhysicsComponent))> = world.physics_mut().0.collect();
        let m_rect = entities[MID].1.2.hitbox
            .after_position(entities[MID].1.1)
            .after_depth(entities[MID].1.2.depth);

        let p_rect = entities[PID].1.2.hitbox
            .after_position(entities[PID].1.1)
            .after_depth(entities[PID].1.2.depth);

        let my = m_rect.y + m_rect.h as f32/2.0;
        let mx = m_rect.x + m_rect.w as f32/2.0;
        let py = p_rect.y + p_rect.h as f32/2.0;
        let px = p_rect.x + p_rect.w as f32/2.0;

        for i in 2..entities.len() {
            let mut footprint = entities[i].1.2.hitbox
                .after_position(entities[i].1.1)
                .after_depth(entities[i].1.2.depth);

            if footprint.intersects_line(mx, my, px, py) {
                return false;
            }
        }

        true
    }

    fn goto(&mut self, world: &mut World, x: f32, y: f32, speed: f32) {
        let (curr_x, curr_y) = {
            let rect = world.physics[MID].as_ref().unwrap().hitbox
                .after_position(world.positions[MID].as_ref().unwrap())
                .after_depth(world.physics[MID].as_ref().unwrap().depth);
            (rect.x, rect.y)
        };

        let angle = (y-curr_y).atan2(x-curr_x);
        let mag = speed;

        world.physics[MID].as_mut().unwrap().velocity.dir = angle;
        world.physics[MID].as_mut().unwrap().velocity.mag = mag;

        world.states[MID].insert("walking".into());

        if world.physics[MID].as_mut().unwrap().velocity.x() > 0.1 {
            world.graphics[MID].as_mut().unwrap().flipped = false;
        } else {
            world.graphics[MID].as_mut().unwrap().flipped = true;
        }
    }

    fn dist(&mut self, world: &mut World, x: f32, y: f32) -> f32 {
        let (curr_x, curr_y) = {
            let rect = world.physics[MID].as_ref().unwrap().hitbox
                .after_position(world.positions[MID].as_ref().unwrap())
                .after_depth(world.physics[MID].as_ref().unwrap().depth);
            (rect.x, rect.y)
        };

        ((curr_y-y).powi(2) + (curr_x-x).powi(2)).sqrt()
    }

    fn stop(&mut self, world: &mut World) {
        world.physics[MID].as_mut().unwrap().velocity.mag = 0.0;
        world.states[MID].remove("walking");
    }
}

