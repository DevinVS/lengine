use std::{collections::HashSet, f32::consts::FRAC_PI_2, f32::consts::{FRAC_PI_4, PI}, process::exit};
use sdl2::{EventPump, event::Event, keyboard::Keycode};
use crate::{vector::Vector, world::World};


pub struct InputSystem<'a> {
    event_pump: &'a mut EventPump,
    key_state: HashSet<Keycode>,
}

impl<'a> InputSystem<'a> {
    pub fn new(event_pump: &'a mut EventPump) -> InputSystem {
        InputSystem {
            event_pump,
            key_state: HashSet::new()
        }
    }

    pub fn run(&mut self, world: &mut World) {
        // Log all events into the input state
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    exit(0);
                },
                Event::KeyDown{ keycode: Some(k), .. } => {
                    self.key_state.insert(k);
                }
                Event::KeyUp { keycode: Some(k), ..} => {
                    self.key_state.remove(&k);
                }
                _ => {}
            }
        }

        // Act based up on current key state

        // Player movement
        if let Some(player) = world.get_player_mut() {
            if let Some(physics_state) = player.physics_state.as_mut() {

                let max_accel = 5.0;

                let north = self.key_state.contains(&Keycode::W);
                let west = self.key_state.contains(&Keycode::A);
                let south = self.key_state.contains(&Keycode::S);
                let east = self.key_state.contains(&Keycode::D);

                let mut mag = 5.0;

                let dir = match (north, east, south, west) {
                    (true, false, false, false) | (true, true, false, true) => -FRAC_PI_2,   // North
                    (false, false, true, false) | (false, true, true, true) => FRAC_PI_2,  // South
                    (false, true, false, false) | (true, true, true, false) => 0.0,         // East
                    (false, false, false, true) | (true, false, true, true) => -PI,         // West
                    (true, true, false, false) => -FRAC_PI_4,        // Northeast
                    (true, false, false, true) => 5.0*FRAC_PI_4,    // Northwest
                    (false, true, true, false) => FRAC_PI_4,       // Southeast
                    (false, false, true, true) => 3.0*FRAC_PI_4,    // Southwest
                    _ => {
                        mag = 0.0;
                        0.0
                    }
                };

                // Since people dont walk by sliding their feet against the floor,
                // It doesn't make sense to move a person by applying a force to them.
                // Instead that person has a maximum velocity they can walk at
                
                physics_state.velocity = Vector::new(dir, mag);
            }
        }
    }
}
