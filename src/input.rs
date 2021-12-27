use std::{collections::HashSet, f32::consts::FRAC_PI_2, f32::consts::{FRAC_PI_4, PI}};
use sdl2::{event::Event, keyboard::Keycode, controller::Button};
use crate::{vector::Vector, world::World};
use sdl2::{GameControllerSubsystem, JoystickSubsystem};
use sdl2::controller::GameController;
use sdl2::controller::Axis;

pub struct InputSystem {
    key_state: HashSet<Keycode>,
    button_state: HashSet<Button>,
    controller_system: GameControllerSubsystem,
    joystick_system: JoystickSubsystem,
    controller: Option<GameController>,
    controller_id: u32
}

impl InputSystem {
    pub fn new(gs: GameControllerSubsystem, js: JoystickSubsystem) -> InputSystem {
        InputSystem {
            key_state: HashSet::new(),
            button_state: HashSet::new(),
            controller_system: gs,
            joystick_system: js,
            controller: None,
            controller_id: 0
        }
    }

    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::KeyDown{ keycode: Some(k), .. } => {
                self.key_state.insert(k);
            }
            Event::KeyUp { keycode: Some(k), ..} => {
                self.key_state.remove(&k);
            }
            Event::ControllerButtonDown { button, .. } => {
                self.button_state.insert(button);
            }
            Event::ControllerButtonUp { button, .. } => {
                self.button_state.remove(&button);
            }
            Event::ControllerDeviceAdded { which, .. } => {
                self.controller = Some(self.controller_system.open(which).unwrap());
            }
            Event::ControllerDeviceRemoved { which, .. } => {
                if which==self.controller_id {
                    self.controller = None;
                }
            }
            _ => {}
        }
    }

    pub fn run(&mut self, world: &mut World) {
        // Act based up on current key state

        // Player movement
        if let Some(player) = world.get_player_mut() {
            if let Some(physics_state) = player.physics_mut() {

                // If joystick connected and its values beyond the deadzone use it, otherwise
                // buttons and keys
                let max_mag = 500.0;

                let mut vel = if self.controller.is_some() {
                    let c = self.controller.as_ref().unwrap();
                    let x = c.axis(Axis::LeftX) as f32 / 32768.0;
                    let y = c.axis(Axis::LeftY) as f32 / 32768.0;

                    let dead_zone = 10_000.0 / 32768.0;

                    if x.abs() > dead_zone || y.abs() > dead_zone {
                        let mag = (x.powi(2)+y.powi(2)).sqrt();
                        let dir = (-y/x).atan();

                        Vector::from_components(x, y)
                    } else {
                        let (dir, mag) = self.button_movement();
                        Vector::new(dir, mag)
                    }
                } else {
                    let (dir, mag) = self.button_movement();
                    Vector::new(dir, mag)
                };

                vel.mag *= max_mag;
                println!("{:?}", vel);
                physics_state.velocity = vel;
            }
        }
    }

    fn button_movement(&self) -> (f32, f32) {
        let north = self.key_state.contains(&Keycode::W) || self.button_state.contains(&Button::DPadUp);
        let west = self.key_state.contains(&Keycode::A) || self.button_state.contains(&Button::DPadLeft);
        let south = self.key_state.contains(&Keycode::S) || self.button_state.contains(&Button::DPadDown);
        let east = self.key_state.contains(&Keycode::D) || self.button_state.contains(&Button::DPadRight);

        let mut mag = 1.0;

        let dir = match (north, east, south, west) {
            (true, false, false, false) | (true, true, false, true) => -FRAC_PI_2,  // North
            (false, false, true, false) | (false, true, true, true) => FRAC_PI_2,   // South
            (false, true, false, false) | (true, true, true, false) => 0.0,         // East
            (false, false, false, true) | (true, false, true, true) => -PI,         // West
            (true, true, false, false) => -FRAC_PI_4,       // Northeast
            (true, false, false, true) => 5.0*FRAC_PI_4,    // Northwest
            (false, true, true, false) => FRAC_PI_4,        // Southeast
            (false, false, true, true) => 3.0*FRAC_PI_4,    // Southwest
            _ => {
                mag = 0.0;
                0.0
            }
        };

        (dir, mag)
    }
}
