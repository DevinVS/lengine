use std::process::exit;
use sdl2::{EventPump, event::Event, keyboard::Keycode};
use crate::world::World;

pub struct InputSystem<'a> {
    event_pump: &'a mut EventPump
}

impl<'a> InputSystem<'a> {
    pub fn new(event_pump: &'a mut EventPump) -> InputSystem {
        InputSystem {
            event_pump
        }
    }

    pub fn run(&mut self, engine: &mut World) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    exit(0);
                },
                _ => {}
            }
        }
    }
}