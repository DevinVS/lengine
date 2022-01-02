use std::collections::HashMap;
use std::process::exit;
use std::time::Duration;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};
use std::sync::mpsc::channel;

use notify::RecursiveMode;
use notify::Watcher;
use notify::watcher;
use yaml_rust::YamlLoader;

use sdl2::event::Event;
use sdl2::image::InitFlag;
use sdl2::keyboard::Keycode;

use game::map::WorldMap;
use game::world::World;
use game::input::InputSystem;
use game::geometry::{PositionComponent, Rect};
use game::physics::{PhysicsSystem, PhysicsComponent};
use game::graphics::{GraphicsSystem, GraphicsComponent, TextureManager};
use game::animation::{AnimationSystem, AnimationComponent, Animation};
use game::effect::EffectsSystem;

fn main() {
    // Code for watching changes
    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();
    watcher.watch("./assets", RecursiveMode::Recursive).unwrap();

    // Create context and relevant subsystems
    let sdl2_context = sdl2::init().unwrap();
    let video_subsystem = sdl2_context.video().unwrap();
    let audio_subsystem = sdl2_context.audio().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();
    let controller_subsystem = sdl2_context.game_controller().unwrap();

    // Create graphics objects such as window, canvas, and texture manager
    let window = video_subsystem.window("title", 800, 600)
        .vulkan()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().accelerated().build().unwrap();
    let mut event_pump = sdl2_context.event_pump().unwrap();

    canvas.set_draw_color((255, 255, 255));

    let texture_creator = canvas.texture_creator();
    let mut texture_manager = TextureManager::new(&texture_creator);

    // Load Game Data
    let mut world = load_world_from_yaml("./game.yml", &mut texture_manager).unwrap();

    // Create Game Systems
    let mut input_system = InputSystem::new(controller_subsystem);
    let mut physics_system = PhysicsSystem::new();
    let mut effects_system = EffectsSystem::new();
    let mut animation_system = AnimationSystem::new();
    let mut graphics_system = GraphicsSystem::new(texture_manager, &mut canvas);

    // Run Game Loop
    loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    exit(0);
                },
                _ => {input_system.handle_event(event)}
            }
        }

        // Run all subsystems
        input_system.run(&mut world);
        physics_system.run(&mut world);
        effects_system.run(&mut world);
        animation_system.run(&mut world);
        graphics_system.run(&mut world);

        // Check for changes
        if rx.try_recv().is_ok() {
            texture_manager = TextureManager::new(&texture_creator);
            world = load_world_from_yaml("game.yml", &mut texture_manager).unwrap();
            graphics_system.texture_manager = texture_manager;
        }

        // Sleep
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn load_world_from_yaml(path: &str, texture_manager: &mut TextureManager) -> Result<World, Box<dyn Error>> {
    let world_map = WorldMap::new();
    let mut world = World::new(world_map);

    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    let docs = YamlLoader::load_from_str(&contents)?;
    let doc = &docs[0];

    let mut texture_map: HashMap<String, usize> = HashMap::new();

    for entity in doc["entities"].as_vec().unwrap_or(&Vec::new()) {
        // Default starting states
        let state = entity["state"].as_str();
        let player = entity["player"].as_bool().unwrap_or(false);

        // Parse position
        let position = {
            let x = entity["position"]["x"].as_f64().map(|e| e as f32);
            let y = entity["position"]["y"].as_f64().map(|e| e as f32);

            if x.is_none() || y.is_none() {
                None
            } else {
                Some(PositionComponent::new(x.unwrap(), y.unwrap()))
            }
        };

        // Parse physics
        let physics = {
            let depth = entity["physics"]["depth"].as_i64().map(|e| e as u32);
            let physical = entity["physics"]["physical"].as_bool();

            let hitbox_x = entity["physics"]["hitbox"]["x"].as_f64().map(|e| e as f32);
            let hitbox_y = entity["physics"]["hitbox"]["y"].as_f64().map(|e| e as f32);
            let hitbox_w = entity["physics"]["hitbox"]["w"].as_i64().map(|e| e as u32);
            let hitbox_h = entity["physics"]["hitbox"]["h"].as_i64().map(|e| e as u32);

            if hitbox_w.is_none() || hitbox_h.is_none() {
                None
            } else {
                let hitbox = Rect::new(hitbox_x.unwrap_or(0.0), hitbox_y.unwrap_or(0.0), hitbox_w.unwrap(), hitbox_h.unwrap());
                Some(PhysicsComponent::new(hitbox, depth.unwrap_or(0), physical.unwrap_or(true)))
            }
        };

        // Parse graphics
        let graphics = {
            let texture_path = entity["graphics"]["path"].as_str().map(|e| e.to_string());

            let renderbox_x = entity["graphics"]["renderbox"]["x"].as_f64().map(|e| e as f32);
            let renderbox_y = entity["graphics"]["renderbox"]["y"].as_f64().map(|e| e as f32);
            let renderbox_w = entity["graphics"]["renderbox"]["w"].as_i64().map(|e| e as u32);
            let renderbox_h = entity["graphics"]["renderbox"]["h"].as_i64().map(|e| e as u32);

            let srcbox_x = entity["graphics"]["srcbox"]["x"].as_i64().map(|e| e as i32);
            let srcbox_y = entity["graphics"]["srcbox"]["y"].as_i64().map(|e| e as i32);
            let srcbox_w = entity["graphics"]["srcbox"]["w"].as_i64().map(|e| e as u32);
            let srcbox_h = entity["graphics"]["srcbox"]["h"].as_i64().map(|e| e as u32);

            if let Some(texture_path) = texture_path {
                let tex_id = texture_map.get(&texture_path).map(|e| *e).unwrap_or_else(|| {
                    let id = &texture_manager.load_texture(&texture_path);
                    texture_map.insert(texture_path, *id);
                    *id
                });

                if renderbox_w.is_none() || renderbox_h.is_none() {
                    None
                } else {
                    let srcbox = if srcbox_x.is_none() || srcbox_y.is_none() || srcbox_w.is_none() || srcbox_h.is_none() {
                        None
                    } else {
                        Some(sdl2::rect::Rect::new(srcbox_x.unwrap(), srcbox_y.unwrap(), srcbox_w.unwrap(), srcbox_h.unwrap()))
                    };

                    let renderbox = Rect::new(renderbox_x.unwrap_or(0.0), renderbox_y.unwrap_or(0.0), renderbox_w.unwrap(), renderbox_h.unwrap());
                    Some(GraphicsComponent::new(tex_id, renderbox, srcbox))
                }
            } else {
                None
            }
        };

        // Parse animations
        let animations = {
            let mut animations = HashMap::new();

            let a_iter = entity["animations"].as_vec();
            if a_iter.is_none() {
                None
            } else {
                for animation in a_iter.unwrap() {
                    let state = animation["animation"]["state"].as_str();
                    let period = animation["animation"]["period"].as_f64().map(|e| e as f32);
                    let textures = animation["animation"]["textures"].as_vec();

                    if state.is_none() || period.is_none() || textures.is_none() {
                        continue;
                    }

                    let textures: Vec<(usize, Option<sdl2::rect::Rect>)> = textures.unwrap().iter()
                        .filter_map(|texture| {
                            let path = texture["path"].as_str();
                            let srcbox_x = texture["srcbox"]["x"].as_i64().map(|e| e as i32);
                            let srcbox_y = texture["srcbox"]["y"].as_i64().map(|e| e as i32);
                            let srcbox_w = texture["srcbox"]["w"].as_i64().map(|e| e as u32);
                            let srcbox_h = texture["srcbox"]["h"].as_i64().map(|e| e as u32);

                            let srcbox = if srcbox_x.is_none() || srcbox_y.is_none() || srcbox_w.is_none() || srcbox_h.is_none() {
                                None
                            } else {
                                Some(sdl2::rect::Rect::new(srcbox_x.unwrap(), srcbox_y.unwrap(), srcbox_w.unwrap(), srcbox_h.unwrap()))
                            };

                            let tex_id = path.map(|e| {
                                if let Some(tex_id) = texture_map.get(e) {
                                    *tex_id
                                } else {
                                    let tex_id = &texture_manager.load_texture(e);
                                    texture_map.insert(e.to_string(), *tex_id);
                                    *tex_id
                                }
                            });

                            if tex_id.is_none() {
                                None
                            } else {
                                Some((tex_id.unwrap(), srcbox))
                            }

                        })
                        .collect();

                    let a = Animation::new(textures, period.unwrap());
                    animations.insert(state.unwrap().to_string(), a);
                }

                Some(AnimationComponent::new(animations))
            }
        };

        let id = world.add_entity(position, physics, graphics, animations, None);
        if state.is_some() {
            world.add_entity_state(id, state.unwrap().to_string());
        }

        if player {
            world.player_id = Some(id);
        }
    }

    Ok(world)
}

