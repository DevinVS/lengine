#![allow(dead_code)]
//! Functions to parse a world file
//!
//! A world file is a yaml file with the following structure:
//!
//! ```yaml
//! graphics:           # Configuration for GraphicsSystem
//!   camera:           # World Camera
//!     rect:           # Rect defining the position of the camera
//!       x: f32        # x position of camera in world coords (default 0)
//!       y: f32        # y position of camera in world coords (default 0)
//!       w: u32        # width of camera viewport in screen pixels (default 800)
//!       h: u32        # height of camera viewport in screen pixels (default 600)
//!     player_box:     # Area within the player can move without the camera moving
//!       x: f32        # x position of player box
//!       y: f32        # y position of player box
//!       w: u32        # width of player box in screen pixels
//!       h: u32        # height of player box in screen pixels
//!     zoom: u32       # camera zoom, scalar factor of world units to screen pixels (default 5)
//!   dialog:           # Configuration for rendering a dialog
//!     path: string    # Path to dialog texture
//!     font: string    # Path to dialog font
//!     fontsize: u32   # Fontsize
//!     renderbox:      # Box to render dialog texture into
//!       x: i32        # x position in screen coordinates
//!       y: i32        # y position in screen coordinates
//!       w: u32        # width in screen coordinates
//!       h: u32        # height in scren coordinates
//!     textbox:        # Box to render text into
//!       x: i32        # x position in screen coordinates
//!       y: i32        # y position ins screen coordinates
//!       w: u32        # width in screen coordinates
//!       h: u32        # height in screen coordinates
//! inputs:             # List of player inputs and the effects they cause
//!   - add:            # List of states added by input
//!     - string        # Individual state added
//!     remove:         # List of states removed by input
//!     - string        # Individual state removed
//!     key: string     # key name that causes effect
//!     button: string  # button name that causes effect
//!     rect:           # Rectangle for the effect
//!       x: f32        # x offset from hitbox (default -2)
//!       y: f32        # y offset from hitbox (default -2)
//!       w: u32        # width offset from hitbox (default 4)
//!       h: u32        # height offset from hitbox (default 4)
//! dialogs:            # List of dialogs that can be displayed to the scren
//!   - name: string    # Name of the dialog
//!     messages:       # List of messages to be displayed sequentially
//!       - string      # A single message
//! background:         # Background of the world
//!   path: string      # path to the texture
//!   color:            # Color for the rest of the window
//!     r: u8           # Red component
//!     g: u8           # Green component
//!     b: u8           # Blue component
//!   renderbox:        # Rectangle to render texture
//!     x: f32          # x position in the world (default 0)
//!     y: f32          # y position in the world (default 0)
//!     w: u32          # Width in world coordinates
//!     h: u32          # Height in world coordinates
//! entitites:          # List of all entities in the world
//!   - state: string   # Default starting state (default none)
//!     player: bool    # Whether this entity is a player (default false)
//!     position:       # Position component for a single entity
//!       x: f32        # x position in world coords
//!       y: f32        # y position in world coords
//!     physics:        # Physics component (requires position)
//!       hitbox:       # Hitbox of player, acts as an offset of the position
//!         x: f32      # x offset of hitbox (default 0)
//!         y: f32      # y offset of hitbox (default 0)
//!         w: u32      # width of hitbox
//!         h: u32      # height of hitbox
//!       depth: u32    # Depth in the world of the player, replaces height in hitbox (default height)
//!     graphics:       # Graphics Component (requres position)
//!       path: string  # Path of the default texture
//!       renderbox:    # Box to render to the world, acts as offset on position
//!         x: f32      # x offset of renderbox in world coordinates (default 0)
//!         y: f32      # y offset of renderbox in world coordinates (default 0)
//!         w: u32      # width of renderbox in world coordinates
//!         h: u32      # height of renderbox in world coordinates
//!       srcbox:       # Rectangle to read from the texture
//!         x: i32      # x position in texture
//!         y: i32      # y position in texture
//!         w: u32      # width of texture
//!         h: u32      # height of texture
//!     animations:     # List of animations that the entity can have
//!       - state: string   # State which triggers the animation
//!         period: f32     # Time until the animation switches to the next texture
//!         path: string    # Path to the animation texture
//!         srcbox:         # Rectangle source for the first frame of the texture (default none)
//!           x: i32        # x coordinate of srcbox
//!           y: i32        # y coordinate of srcbox
//!           w: u32        # height of srcbox
//!           h: u32        # width of srcbox
//!         frame_width: u32    # width of a single frame
//!         frame_count: u32    # Number of animation frames
//!     events:         # List of events that can occur for this entity
//!       - states:     # List of necessary states which trigger the event
//!         - string    # A state string
//!         actions:    # list of actions which will run once triggered
//!           - type: string    # Type of action to run, options: add_state, remove_state, show_dialog
//!             state: string   # State to add/remove
//!             dialog: string  # dialog to show
//!             delay: f32      # delay after the last action until this runs (default 0)
//! ```

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use sdl2::pixels::Color;
use yaml_rust::{Yaml, YamlLoader};

use crate::effect::EffectSpawner;
use crate::input::InputConfig;
use crate::world::World;
use crate::geometry::{Rect, PositionComponent};
use crate::physics::PhysicsComponent;
use crate::graphics::{GraphicsComponent, GraphicsConfig, TextureManager, Camera};
use crate::animation::{AnimationComponent, Animation};
use crate::state::{ActionComponent, Sequence};
use crate::actions::{Action, AddState, RemoveState, ShowDialog, AddEffect};
use crate::dialog::Dialog;


/// Parse yaml into an f32
/// Acceps either an integer or a floating point as input
fn parse_f32(yaml: &Yaml) -> Option<f32> {
    if yaml.as_i64().is_some() {
        yaml.as_i64().map(|i| i as f32)
    } else {
        yaml.as_f64().map(|f| f as f32)
    }
}

/// Parse yaml into a f32 with a default
fn parse_f32_or(yaml: &Yaml, default: f32) -> f32 {
    parse_f32(yaml).unwrap_or(default)
}

/// Parse yaml into a u32
fn parse_u32(yaml: &Yaml) -> Option<u32> {
    yaml.as_i64().map(|i| i as u32)
}

/// Parse yaml into a u32 with a default
fn parse_u32_or(yaml: &Yaml, default: u32) -> u32 {
    parse_u32(yaml).unwrap_or(default)
}

/// Parse yaml into a i32
fn parse_i32(yaml: &Yaml) -> Option<i32> {
    yaml.as_i64().map(|i| i as i32)
}

/// Parse yaml into a i32 with default
fn parse_i32_or(yaml: &Yaml, default: i32) -> i32 {
    parse_i32(yaml).unwrap_or(default)
}

/// Parse yaml into a string
fn parse_string(yaml: &Yaml) -> Option<String> {
    yaml.as_str().map(|s| s.to_string())
}

/// Parse yaml into a string with default
fn parse_string_or(yaml: &Yaml, default: &str) -> String {
    parse_string(yaml).unwrap_or(default.to_string())
}

/// Parse yaml into bool
fn parse_bool(yaml: &Yaml) -> Option<bool> {
    yaml.as_bool()
}

/// Parse yaml into bool with default
fn parse_bool_or(yaml: &Yaml, default: bool) -> bool {
    parse_bool(yaml).unwrap_or(default)
}

/// Parse yaml into world rect with default components
fn parse_world_rect_with_defaults(yaml: &Yaml, default: (Option<f32>, Option<f32>, Option<u32>, Option<u32>)) -> Option<Rect> {
    let x = parse_f32(&yaml["x"]).map(|e| Some(e)).unwrap_or(default.0);
    let y = parse_f32(&yaml["y"]).map(|e| Some(e)).unwrap_or(default.1);
    let w = parse_u32(&yaml["w"]).map(|e| Some(e)).unwrap_or(default.2);
    let h = parse_u32(&yaml["h"]).map(|e| Some(e)).unwrap_or(default.3);

    if x.is_none() || y.is_none() || w.is_none() || h.is_none() {
        None
    } else {
        Some(Rect::new(x.unwrap(), y.unwrap(), w.unwrap(), h.unwrap()))
    }
}

/// Parse yaml into world rect
fn parse_world_rect(yaml: &Yaml) -> Option<Rect> {
    parse_world_rect_with_defaults(yaml, (None, None, None, None))
}

/// Parse yaml into world rect with default rect
fn parse_world_rect_or(yaml: &Yaml, default: (f32, f32, u32, u32)) -> Rect {
    parse_world_rect_with_defaults(yaml, (Some(default.0), Some(default.1), Some(default.2), Some(default.3))).unwrap()
}

/// Parse yaml into sdl2 rect with default components
fn parse_sdl2_rect_with_defaults(yaml: &Yaml, default: (Option<i32>, Option<i32>, Option<u32>, Option<u32>)) -> Option<sdl2::rect::Rect> {
    let x = parse_i32(&yaml["x"]).map(|e| Some(e)).unwrap_or(default.0);
    let y = parse_i32(&yaml["y"]).map(|e| Some(e)).unwrap_or(default.1);
    let w = parse_u32(&yaml["w"]).map(|e| Some(e)).unwrap_or(default.2);
    let h = parse_u32(&yaml["h"]).map(|e| Some(e)).unwrap_or(default.3);

    if x.is_none() || y.is_none() || w.is_none() || h.is_none() {
        None
    } else {
        Some(sdl2::rect::Rect::new(x.unwrap(), y.unwrap(), w.unwrap(), h.unwrap()))
    }
}

/// Parse yaml into sdl2 rect
fn parse_sdl2_rect(yaml: &Yaml) -> Option<sdl2::rect::Rect> {
    parse_sdl2_rect_with_defaults(yaml, (None, None, None, None))
}

/// Parse yaml into sdl2 rect with default rect
fn parse_sdl2_rect_or(yaml: &Yaml, default: (i32, i32, u32, u32)) -> sdl2::rect::Rect {
    parse_sdl2_rect_with_defaults(yaml, (Some(default.0), Some(default.1), Some(default.2), Some(default.3))).unwrap()
}

/// Parse yaml into a sequence
fn parse_sequence(yaml: &Yaml) -> Option<Sequence> {
    let a_iter = yaml.as_vec();

    if a_iter.is_none() {
        None
    } else {
        let actions = a_iter.unwrap().iter()
            .filter_map(|y| {
                let delay = parse_f32_or(&y["delay"], 0.0);
                let actions = parse_action(y);

                if actions.is_none() {
                    None
                } else {
                    Some((delay, actions.unwrap()))
                }
            })
            .collect();

        Some(Sequence::new(actions))
    }
}

/// Parse yaml into state event
fn parse_event(yaml: &Yaml) -> Option<(Vec<String>, Sequence)> {
    let states: Vec<String> = yaml["states"].as_vec().unwrap_or(&Vec::new())
        .iter()
        .filter_map(|y| {
            parse_string(y)
        })
        .collect();

    let sequence = parse_sequence(&yaml["actions"]);

    if sequence.is_none() || states.len() == 0 {
        None
    } else {
        Some((states, sequence.unwrap()))
    }
}

/// Parse yaml into an action
fn parse_action(yaml: &Yaml) -> Option<Box<dyn Action>> {
    match yaml["type"].as_str() {
        Some("dialog") => {
            parse_string(&yaml["dialog"])
                .map(|s| Box::new(ShowDialog { dialog: s }) as Box<dyn Action>)
        }
        Some("add_state") => {
            parse_string(&yaml["state"])
                .map(|s| Box::new(AddState { state: s }) as Box<dyn Action>)
        }
        Some("remove_state") => {
            parse_string(&yaml["state"])
                .map(|s| Box::new(RemoveState { state: s.to_string() }) as Box<dyn Action>)
        }
        Some("add_effect") => {
            let e = parse_effect(&yaml["effect"]);
            Some(Box::new(AddEffect { effect: e }) as Box<dyn Action>)
        }
        _ => None
    }
}

/// Parse yaml into a dialog
fn parse_dialog(yaml: &Yaml) -> Option<(String, Dialog)> {
    let name = parse_string(&yaml["name"]);
    let messages: Vec<String> = yaml["messages"].as_vec().unwrap_or(&Vec::new())
        .iter()
        .map(|e| parse_string(e).unwrap())
        .collect();

    if name.is_none() {
        None
    } else {
        Some((name.unwrap(), Dialog::new(messages)))
    }
}

/// Parse yaml into animation
fn parse_animation(yaml: &Yaml, texture_manager: &mut TextureManager) -> Option<(String, Animation)> {
    let state = parse_string(&yaml["state"]);
    let period = parse_f32(&yaml["period"]);
    let after = parse_sequence(&yaml["after"]);

    let texture = parse_texture(&yaml, texture_manager);
    let frame_width = parse_u32_or(&yaml["frame_width"], 0);
    let frame_count = parse_u32_or(&yaml["frame_count"], 1);

    let textures: Vec<(usize, Option<sdl2::rect::Rect>)> = (0..frame_count)
        .filter_map(|frame_num| {
            if let Some(tex) = texture {
                Some((tex.0, tex.1.map(|mut b| {
                    b.x += frame_num as i32 * frame_width as i32;
                    b
                })))
            } else {
                None
            }
        })
        .collect();

    if state.is_none() || period.is_none() || textures.len() == 0 {
        None
    } else {
        Some((state.unwrap(), Animation::new(textures, period.unwrap(), after)))
    }
}


/// Parse yaml into texture
fn parse_texture(yaml: &Yaml, texture_manager: &mut TextureManager) -> Option<(usize, Option<sdl2::rect::Rect>)> {
    let path = parse_string(&yaml["path"]);
    let srcbox = parse_sdl2_rect(&yaml["srcbox"]);

    if path.is_none() {
        None
    } else {
        let tex_id = texture_manager.load_texture(&path.unwrap());
        Some((tex_id, srcbox))
    }
}

/// Parse yaml into an entity
fn parse_entity(yaml: &Yaml, texture_manager: &mut TextureManager) -> (
    Option<PositionComponent>,
    Option<PhysicsComponent>,
    Option<GraphicsComponent>,
    Option<AnimationComponent>,
    Option<ActionComponent>,
    Option<String>,
    bool
) {
    let position = parse_position_component(&yaml["position"]);
    let physics = parse_physics_component(&yaml["physics"]);
    let graphics = parse_graphics_component(&yaml["graphics"], texture_manager);
    let animation = parse_animations_component(&yaml["animations"], texture_manager);
    let actions = parse_actions_component(&yaml["events"]);

    let default_state = parse_string(&yaml["state"]);
    let is_player = parse_bool_or(&yaml["player"], false);

    (position, physics, graphics, animation, actions, default_state, is_player)
}

/// Parse yaml into a position component
fn parse_position_component(yaml: &Yaml) -> Option<PositionComponent> {
    let x = parse_f32(&yaml["x"]);
    let y = parse_f32(&yaml["y"]);

    if x.is_none() || y.is_none() {
        None
    } else {
        Some(PositionComponent::new(x.unwrap(), y.unwrap()))
    }
}

/// Parse yaml into a physics component
fn parse_physics_component(yaml: &Yaml) -> Option<PhysicsComponent> {
    let hitbox = parse_world_rect_with_defaults(&yaml["hitbox"], (Some(0.0), Some(0.0), None, None));
    let physical = parse_bool_or(&yaml["physical"], true);
    let depth = parse_u32(&yaml["depth"]).map(|d| Some(d)).unwrap_or(hitbox.map(|h| h.h as u32));

    if hitbox.is_none() {
        None
    } else {
        Some(PhysicsComponent::new(hitbox.unwrap(), depth.unwrap(), physical))
    }
}

/// Parse yaml into graphics component
fn parse_graphics_component(yaml: &Yaml, texture_manager: &mut TextureManager) -> Option<GraphicsComponent> {
    let path = parse_string(&yaml["path"]);
    let renderbox = parse_world_rect_with_defaults(&yaml["renderbox"], (Some(0.0), Some(0.0), None, None));
    let srcbox = parse_sdl2_rect(&yaml["srcbox"]);

    if path.is_none() || renderbox.is_none() {
        None
    } else {
        let tex_id = texture_manager.load_texture(&path.unwrap());
        Some(GraphicsComponent::new(tex_id, renderbox.unwrap(), srcbox))
    }
}

/// Parse yaml into animations component
fn parse_animations_component(yaml: &Yaml, texture_manager: &mut TextureManager) -> Option<AnimationComponent> {
    let mut animations = HashMap::new();

    let a_iter = yaml.as_vec();

    if a_iter.is_none() {
        None
    } else {
        a_iter.unwrap().iter()
            .for_each(|y| {
                let (state, animation) = parse_animation(&y, texture_manager).unwrap();
                animations.insert(state, animation);
            });

        Some(AnimationComponent::new(animations))
    }
}

/// Parse yaml into actions component
fn parse_actions_component(yaml: &Yaml) -> Option<ActionComponent> {
    let event_iter = yaml.as_vec();

    if event_iter.is_none() {
        None
    } else {
        let mut actions = HashMap::new();
        event_iter.unwrap().iter()
            .filter_map(|y| parse_event(y))
            .for_each(|(key, val)| {
                actions.insert(key, val);
            });

        Some(ActionComponent::new(actions))
    }
}

/// Parse yaml into effect
fn parse_effect(yaml: &Yaml) -> EffectSpawner {
    let added: Vec<String> = yaml["add"].as_vec().unwrap_or(&Vec::new()).iter()
        .filter_map(|y| parse_string(y))
        .collect();

    let removed: Vec<String> = yaml["remove"].as_vec().unwrap_or(&Vec::new()).iter()
        .filter_map(|y| parse_string(y))
        .collect();

    let ttl = parse_f32(&yaml["ttl"]);
    let rect = parse_world_rect_or(&yaml["rect"], (-2.0, -2.0, 4, 4));

    EffectSpawner::new(added, removed, rect, ttl)
}

/// Parse yaml into input
fn parse_input(yaml: &Yaml) -> Option<(Option<String>, Option<String>, EffectSpawner)> {
    let effect = parse_effect(yaml);

    let key = parse_string(&yaml["key"]);
    let button = parse_string(&yaml["button"]);


    Some((key, button, effect))
}

/// Parse yaml into input config
fn parse_input_config(yaml: &Yaml) -> InputConfig {
    let mut config = InputConfig::new();

    yaml.as_vec().unwrap_or(&Vec::new())
        .iter()
        .filter_map(|y| {
            parse_input(y)
        })
        .for_each(|(key, button, effect)| {
            if key.is_some() {
                config.add_keymap(&key.unwrap(), effect.clone());
            }

            if button.is_some() {
                config.add_buttonmap(&button.unwrap(), effect.clone());
            }
        });

    config
}

/// Parse yaml into graphics config
fn parse_graphics_config(yaml: &Yaml) -> GraphicsConfig {
    let debug = parse_bool_or(&yaml["debug"], false);

    let dialog_tex_path = parse_string(&yaml["dialog"]["path"]);
    let dialog_font_path = parse_string(&yaml["dialog"]["font"]);
    let dialog_font_size = parse_u32(&yaml["dialog"]["fontsize"]).map(|u| u as u16);
    let dialog_renderbox = parse_sdl2_rect(&yaml["dialog"]["renderbox"]);
    let dialog_textbox = parse_sdl2_rect(&yaml["dialog"]["textbox"]);

    let cam_rect = parse_world_rect_with_defaults(&yaml["camera"]["rect"], (Some(0.0), Some(0.0), Some(800), Some(600))).unwrap();
    let cam_zoom = parse_u32_or(&yaml["camera"]["zoom"], 5);

    let cam_player_box = {
        let w = parse_u32(&yaml["camera"]["player_box"]["w"]).unwrap();
        let h = parse_u32(&yaml["camera"]["player_box"]["h"]).unwrap();
        let x = parse_f32(&yaml["camera"]["player_box"]["x"]).unwrap_or((cam_rect.w-w) as f32/2.0);
        let y = parse_f32(&yaml["camera"]["player_box"]["y"]).unwrap_or((cam_rect.h-h) as f32/2.0);

        Rect::new(x, y, w, h)
    };

    GraphicsConfig {
        debug,
        dialog_tex_path,
        dialog_font_path,
        dialog_font_size,
        dialog_renderbox,
        dialog_textbox,
        camera: Camera {
            rect: cam_rect,
            player_box: cam_player_box,
            zoom: cam_zoom
        }
    }
}

/// Parse Game File
pub fn parse_game_file(path: &str, texture_manager: &mut TextureManager) -> (World, InputConfig, GraphicsConfig) {
    let mut file = File::open(path).unwrap();
    let file_size = file.metadata().unwrap().len();
    let mut contents = String::with_capacity(file_size as usize);
    file.read_to_string(&mut contents).unwrap();

    parse_game_string(&contents, texture_manager)
}

/// Parse Game String
pub fn parse_game_string(contents: &str, texture_manager: &mut TextureManager) -> (World, InputConfig, GraphicsConfig) {
    let docs = YamlLoader::load_from_str(contents).unwrap();
    let doc = &docs[0];

    // World
    let background = parse_graphics_component(&doc["background"], texture_manager);

    let b_red = parse_u32_or(&doc["background"]["color"]["r"], 255);
    let b_blue = parse_u32_or(&doc["background"]["color"]["g"], 255);
    let b_green = parse_u32_or(&doc["background"]["color"]["b"], 255);

    let background_color = Color::RGB(b_red as u8, b_blue as u8, b_green as u8);

    let mut world = World::new(background, background_color);

    // Parse the System Configs
    let input_config = parse_input_config(&doc["inputs"]);
    let graphics_config = parse_graphics_config(&doc["graphics"]);

    // Parse the Entities
    for entity in doc["entities"].as_vec().unwrap() {
        let comps = parse_entity(entity, texture_manager);
        let id = world.add_entity(
            comps.0,
            comps.1,
            comps.2,
            comps.3,
            comps.4
        );

        if let Some(state) = comps.5 {
            world.add_entity_state(id, state);
        }

        if comps.6 {
            world.player_id = Some(id);
        }
    }

    // Parse Dialogs
    doc["dialogs"].as_vec().unwrap_or(&Vec::new())
        .iter()
        .filter_map(|y| parse_dialog(y))
        .for_each(|(name, dialog)| world.add_dialog(name, dialog));

    (world, input_config, graphics_config)
}
