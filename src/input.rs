use std::collections::HashMap;
use std::collections::HashSet;

use sdl2::GameControllerSubsystem;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::controller::Button;
use sdl2::controller::GameController;
use sdl2::controller::Axis;

use crate::vector::Vector;
use crate::world::World;
use crate::effect::EffectSpawner;

/// user defined key and button mappings to states
#[derive(Debug)]
pub struct InputConfig {
    keymap: HashMap<Keycode, EffectSpawner>,
    buttonmap: HashMap<Button, EffectSpawner>
}

impl InputConfig {
    /// Create a new InputConfig
    pub fn new() -> InputConfig {
        InputConfig {
            keymap: HashMap::new(),
            buttonmap: HashMap::new()
        }
    }

    /// Add a key mapping from its name
    pub fn add_keymap(&mut self, key: &str, es: EffectSpawner) {
        let key = Keycode::from_name(key);

        if let Some(key) = key {
            self.keymap.insert(key, es);
        }
    }

    /// Add button mapping from its name
    pub fn add_buttonmap(&mut self, button: &str, es: EffectSpawner) {
        let button = Button::from_string(button);

        if let Some(button) = button {
            self.buttonmap.insert(button, es);
        }
    }
}


/// System to handle input devices such as keyboards, joysticks, and controllers
pub struct InputSystem {
    config: InputConfig,
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
    pub fn new(config: InputConfig, gs: GameControllerSubsystem) -> InputSystem {
        InputSystem {
            config,
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
        if world.curr_dialog.is_some() {
            let dialog = world.dialogs.get_mut(world.curr_dialog.as_ref().unwrap()).unwrap();

            if self.key_state.contains(&Keycode::E) || self.button_state.contains(&Button::A) {
                if dialog.finished() {
                    dialog.next();
                    dialog.run_after(&mut world.effects, &mut world.curr_dialog);
                    world.curr_dialog = None;
                } else {
                    dialog.next();
                }


                self.key_state.remove(&Keycode::E);
                self.button_state.remove(&Button::A);
            }

            return;
        }

        // Player movement
        let player = 0;
        if let (Some(pos), Some(physics_state)) = (world.positions[player].as_mut(), world.physics[player].as_mut()) {
            // If the interact key is pressed try to interact with the object that is in front of us
            let player_rect = physics_state.hitbox
                .after_position(&pos)
                .after_depth(physics_state.depth);

            for key in self.config.keymap.keys() {
                if self.key_state.contains(key) {
                    let mut effect = self.config.keymap[key].spawn();

                    effect.rect.x += player_rect.x;
                    effect.rect.y += player_rect.y;
                    effect.rect.w += player_rect.w;
                    effect.rect.h += player_rect.h;

                    world.effects.push(effect);
                    self.key_state.remove(key);
                }
            }

            for button in self.config.buttonmap.keys() {
                if self.button_state.contains(button) {
                    let mut effect = self.config.buttonmap[button].spawn();

                    effect.rect.x += player_rect.x;
                    effect.rect.y += player_rect.y;
                    effect.rect.w += player_rect.w;
                    effect.rect.h += player_rect.h;

                    world.effects.push(effect);
                    self.button_state.remove(button);
                }
            }

            // If joystick connected and its values beyond the deadzone use it, otherwise
            // buttons and keys
            let max_mag = 80.0;

            let (x, y) = if self.controller.is_some() {
                let (x, y) = self.joystick_velocity();

                if x.abs() < 0.01 && y.abs() < 0.01 {
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
