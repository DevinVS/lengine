use std::{fs::File, collections::HashMap};
use std::io::Read;

use yaml_rust::{Yaml, YamlLoader};
use crate::state::Sequence;
use crate::{actions::{Action, AddState, RemoveState, ShowDialog}, geometry::{Rect, PositionComponent}, physics::PhysicsComponent, graphics::{GraphicsComponent, TextureManager, Camera}, animation::{AnimationComponent, Animation}, state::ActionComponent, dialog::Dialog};
use crate::world::World;
use crate::graphics::GraphicsConfig;

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

    let textures: Vec<(usize, Option<sdl2::rect::Rect>)> = yaml["textures"].as_vec().unwrap_or(&Vec::new())
        .iter()
        .filter_map(|y| parse_texture(y, texture_manager))
        .collect();

    let after = parse_sequence(&yaml["after"]);

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
    let depth = parse_u32(&yaml["depth"]).map(|d| Some(d)).unwrap_or(hitbox.map(|h| h.y as u32));

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
                let (state, animation) = parse_animation(&y["animation"], texture_manager).unwrap();
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

/// Parse yaml into graphics config
fn parse_graphics_config(yaml: &Yaml) -> GraphicsConfig {
    let debug = parse_bool_or(&yaml["debug"], false);

    let dialog_tex_path = parse_string(&yaml["dialog"]["path"]);
    let dialog_font_path = parse_string(&yaml["dialog"]["font"]);
    let dialog_font_size = parse_u32(&yaml["dialog"]["fontsize"]).map(|u| u as u16);
    let dialog_renderbox = parse_sdl2_rect(&yaml["dialog"]["renderbox"]);
    let dialog_textbox = parse_sdl2_rect(&yaml["dialog"]["textbox"]);

    let cam_rect = parse_world_rect_with_defaults(&yaml["camera"], (Some(0.0), Some(0.0), Some(800), Some(600))).unwrap();
    let cam_zoom = parse_u32_or(&yaml["camera"]["zoom"], 5);

    GraphicsConfig {
        debug,
        dialog_tex_path,
        dialog_font_path,
        dialog_font_size,
        dialog_renderbox,
        dialog_textbox,
        camera: Camera {
            x: cam_rect.x,
            y: cam_rect.y,
            w: cam_rect.w,
            h: cam_rect.h,
            zoom: cam_zoom
        }
    }
}

/// Parse Game File
pub fn parse_game_file(path: &str, texture_manager: &mut TextureManager) -> (World, GraphicsConfig) {
    let mut file = File::open(path).unwrap();
    let file_size = file.metadata().unwrap().len();
    let mut contents = String::with_capacity(file_size as usize);
    file.read_to_string(&mut contents).unwrap();

    parse_game_string(&contents, texture_manager)
}

/// Parse Game String
pub fn parse_game_string(contents: &str, texture_manager: &mut TextureManager) -> (World, GraphicsConfig) {
    let docs = YamlLoader::load_from_str(contents).unwrap();
    let doc = &docs[0];

    // World
    let background = parse_graphics_component(&doc["world"]["background"], texture_manager);
    let mut world = World::new(background);

    // Parse the System Configs
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

    (world, graphics_config)
}
