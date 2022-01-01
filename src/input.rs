use std::{collections::HashSet, f32::consts::FRAC_PI_2, f32::consts::{FRAC_PI_4, PI}};
use sdl2::{event::Event, keyboard::Keycode, controller::Button};
use crate::{vector::Vector, world::World};
use sdl2::{GameControllerSubsystem, JoystickSubsystem};
use sdl2::controller::GameController;
use sdl2::controller::Axis;

/// System to handle input devices such as keyboards, joysticks, and controllers
pub struct InputSystem {
    /// Set of all the currently pressed keys
    key_state: HashSet<Keycode>,
    /// Set of all the currently pressed buttons
    button_state: HashSet<Button>,
    /// Subsystem for enumerating, opening, and closing controllers
    controller_system: GameControllerSubsystem,
    /// Currently selected controller
    controller: Option<GameController>,
    /// Currently selected controller id
    controller_id: u32
}

impl InputSystem {
    /// Create a new Input System
    pub fn new(gs: GameControllerSubsystem) -> InputSystem {
        InputSystem {
            key_state: HashSet::new(),
            button_state: HashSet::new(),
            controller_system: gs,
            controller: None,
            controller_id: 0
        }
    }

    /// Process an event from the event pump
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

    /// Based on current input modify the world state
    pub fn run(&mut self, world: &mut World) {
        // Act based up on current key state

        // Player movement
        if let Some(player) = world.player_id {
            if let (_, Some(physics_state)) = world.get_entity_physics_mut(player) {

                // If joystick connected and its values beyond the deadzone use it, otherwise
                // buttons and keys
                let max_mag = 100.0;

                let mut vel = if self.controller.is_some() {
                    let c = self.controller.as_ref().unwrap();
                    let x = c.axis(Axis::LeftX) as f32 / 32768.0;
                    let y = c.axis(Axis::LeftY) as f32 / 32768.0;

                    let dead_zone = 10_000.0 / 32768.0;

                    if x.abs() > dead_zone || y.abs() > dead_zone {
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
                physics_state.velocity = vel;

                if vel.mag != 0.0 {
                    world.remove_entity_state(player, &"idle".to_string());
                    world.add_entity_state(player, "walking".into());
                } else {
                    world.remove_entity_state(player, &"walking".to_string());
                    world.add_entity_state(player, "idle".into());
                }

                if let (_, Some(graphics)) = world.get_entity_graphics_mut(player) {
                    if vel.x() > 0.1 {
                        graphics.flipped = false;
                    } else if vel.x() < -0.1 {
                        graphics.flipped = true;
                    }
                }
            }
        }
    }

    /// Move the player using the buttons as inputs
    fn button_movement(&self) -> (f32, f32) {
        let north = self.key_state.contains(&Keycode::W) || self.button_state.contains(&Button::DPadUp) || self.key_state.contains(&Keycode::Up);
        let west = self.key_state.contains(&Keycode::A) || self.button_state.contains(&Button::DPadLeft) || self.key_state.contains(&Keycode::Left);
        let south = self.key_state.contains(&Keycode::S) || self.button_state.contains(&Button::DPadDown) || self.key_state.contains(&Keycode::Down);
        let east = self.key_state.contains(&Keycode::D) || self.button_state.contains(&Button::DPadRight) || self.key_state.contains(&Keycode::Right);

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
