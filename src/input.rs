use std::collections::HashSet;

use sdl2::GameControllerSubsystem;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::controller::Button;
use sdl2::controller::GameController;
use sdl2::controller::Axis;

use crate::vector::Vector;
use crate::world::World;
use crate::effect::Effect;
use crate::geometry::Rect;


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

        // If a dialog exists, process no future input and instead wait for the e key
        if let Some(dialog) = world.current_dialog() {
            if self.key_state.contains(&Keycode::E) {
                if dialog.finished() {
                    dialog.next();
                    world.curr_dialog = None;
                } else {
                    dialog.next();
                }


                self.key_state.remove(&Keycode::E);
            }

            return;
        }
        // Player movement
        if let Some(player) = world.player_id {
            if let (Some(pos), Some(physics_state)) = (world.positions[player].as_mut(), world.physics[player].as_mut()) {
                // If the interact key is pressed try to interact with the object that is in front of us
                if self.key_state.contains(&Keycode::E) {
                    let mut r = physics_state.hitbox
                        .after_position(&pos)
                        .after_depth(physics_state.depth);

                    r.x -= 2.0;
                    r.w += 4;
                    r.y -= 3.0;
                    r.h += 3;

                    world.effects.push(Effect::new(
                        "interact".to_string(),
                        Rect::new(r.x, r.y-5.0,r.w, r.h+5),
                        Some(0.0)
                    ));

                    self.key_state.remove(&Keycode::E);
                }

                // If joystick connected and its values beyond the deadzone use it, otherwise
                // buttons and keys
                let max_mag = 100.0;

                let (x, y) = if self.controller.is_some() {
                    let (x, y) = self.joystick_velocity();

                    if x < 0.01 && y < 0.01 {
                        self.button_velocity()
                    } else {
                        (x, y)
                    }
                } else {
                    self.button_velocity()
                };

                let mut vel = Vector::from_components(x, y);

                if vel.mag > 1.0 {vel.mag=1.0;}

                vel.mag *= max_mag;
                physics_state.velocity = vel;

                // Set appropriate states for idle and walking
                if vel.mag != 0.0 {
                    world.remove_entity_state(player, &"idle".to_string());
                    world.add_entity_state(player, "walking".into());
                } else {
                    world.remove_entity_state(player, &"walking".to_string());
                    world.add_entity_state(player, "idle".into());
                }

                // If the player is drawable, make sure to flip it when moving the other way
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
    /// Move the player using the joysticks
    fn joystick_velocity(&self) -> (f32, f32) {
        let c = self.controller.as_ref().unwrap();
        let x = c.axis(Axis::LeftX) as f32 / 32768.0;
        let y = c.axis(Axis::LeftY) as f32 / 32768.0;

        let dead_zone = 10_000.0 / 32768.0;

        if x.abs() > dead_zone || y.abs() > dead_zone {
            (x, y)
        } else {
            (0.0, 0.0)
        }
    }

    /// Move the player using the buttons as inputs
    fn button_velocity(&self) -> (f32, f32) {
        let north = self.key_state.contains(&Keycode::W) || self.button_state.contains(&Button::DPadUp) || self.key_state.contains(&Keycode::Up);
        let west = self.key_state.contains(&Keycode::A) || self.button_state.contains(&Button::DPadLeft) || self.key_state.contains(&Keycode::Left);
        let south = self.key_state.contains(&Keycode::S) || self.button_state.contains(&Button::DPadDown) || self.key_state.contains(&Keycode::Down);
        let east = self.key_state.contains(&Keycode::D) || self.button_state.contains(&Button::DPadRight) || self.key_state.contains(&Keycode::Right);


        let north = if north {1} else {0} as f32;
        let south = if south {1} else {0} as f32;
        let west = if west {1} else {0} as f32;
        let east = if east {1} else {0} as f32;

        (east-west, south-north)
    }
}
