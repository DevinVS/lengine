use std::ops::Mul;
use std::time::Instant;
use std::collections::HashSet;

use crate::pathfinding;
use crate::world::World;
use crate::physics::PhysicsComponent;
use crate::geometry::PositionComponent;

static PID: usize = 0;
static MID: usize = 1;

#[derive(PartialEq, Eq)]
enum MonsterState {
    Idle,
    Aggro,
    Lost
}

pub struct AISystem {
    monster_state: MonsterState,
    last_aggro: Instant,
    idle_path: Vec<(f32, f32, f32)>,
    next_idle: usize,
    last_idle_time: Instant,
    aggro_distance: f32,
    lost_delay: f32,
    last_pathfind: Instant
}


impl AISystem {
    pub fn new(idle_path: Vec<(f32, f32, f32)>, aggro_distance: f32, lost_delay: f32) -> Self {
        Self {
            monster_state: MonsterState::Idle,
            last_aggro: Instant::now(),
            idle_path,
            next_idle: 0,
            last_idle_time: Instant::now(),
            aggro_distance,
            lost_delay,
            last_pathfind: Instant::now()
        }
    }

    pub fn run(&mut self, world: &mut World) {
        // Check if can see player, if so set aggro to true, if aggro, then lost
        if self.player_visible(world) {
            let (x, y) = {
                let pos = world.positions[PID].as_ref().unwrap();
                (pos.x, pos.y)
            };

            if self.dist(world, x, y) < self.aggro_distance {
                self.monster_state = MonsterState::Aggro;
            }
        } else if self.monster_state == MonsterState::Aggro {
            self.last_aggro = Instant::now();
            self.monster_state = MonsterState::Lost;
        }

        match self.monster_state {
            MonsterState::Idle => {
                // Resume Idle Path
                let (dest_x, dest_y, _) = self.idle_path[self.next_idle];

                if self.dist(world, dest_x, dest_y) < 2.0 {
                    self.next_idle += 1;
                    self.next_idle %= self.idle_path.len();
                    return;
                }

                self.goto(world, dest_x, dest_y, 60.0);
            }
            MonsterState::Aggro => {
                // A* Pathfinding to the player
                    let (x, y) = {
                        let rect = world.physics[PID].as_ref().unwrap().hitbox
                            .after_position(world.positions[PID].as_ref().unwrap())
                            .after_depth(world.physics[PID].as_ref().unwrap().depth);
                        (rect.x, rect.y)
                    };

                    let speed = 57.0 + 20.0 * self.last_pathfind.elapsed().as_secs_f32().mul(5.0).sin();

                    self.goto(world, x, y, speed);
            }
            MonsterState::Lost => {
                // Wait, and then return to idle
                self.stop(world);
                if self.last_aggro.elapsed().as_secs_f32() > self.lost_delay {
                    self.monster_state = MonsterState::Idle;
                }
            }
        }
    }

    fn player_visible(&mut self, world: &mut World) -> bool {
        let entities: Vec<(usize, (&mut HashSet<String>, &mut PositionComponent, &mut PhysicsComponent))> = world.physics_mut().collect();
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

