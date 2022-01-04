use std::collections::HashMap;
use std::process::exit;
use std::time::Duration;
use std::fs::File;
use std::io::{BufReader, Read};

use yaml_rust::{YamlLoader, Yaml};

use sdl2::event::Event;
use sdl2::image::InitFlag;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::GameControllerSubsystem;
use sdl2::ttf::Sdl2TtfContext;

use game::dialog::Dialog;
use game::state::{ActionComponent, Sequence, StateSystem};
use game::map::WorldMap;
use game::world::World;
use game::input::InputSystem;
use game::geometry::{PositionComponent, Rect};
use game::physics::{PhysicsSystem, PhysicsComponent};
use game::graphics::{GraphicsSystem, GraphicsComponent, TextureManager, Camera};
use game::animation::{AnimationSystem, AnimationComponent, Animation};
use game::effect::EffectSystem;
use game::actions::{ShowDialog, AddState, RemoveState, Action};


fn main() {
    // Create context and relevant subsystems
    let sdl2_context = sdl2::init().unwrap();
    let video_subsystem = sdl2_context.video().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();
    let mut ttf_context = sdl2::ttf::init().unwrap();
    let controller_subsystem = sdl2_context.game_controller().unwrap();

    // Create graphics objects such as window, canvas, and texture manager
    let window = video_subsystem.window("title", 800, 600)
        .vulkan()
        .maximized()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().accelerated().build().unwrap();
    let mut event_pump = sdl2_context.event_pump().unwrap();

    canvas.set_draw_color((255, 255, 255));

    let texture_creator = canvas.texture_creator();
    let texture_manager = TextureManager::new(&texture_creator);

    let (
        mut world,
        mut input_system,
        mut physics_system,
        mut effects_system,
        mut animation_system,
        mut graphics_system,
        mut state_system
    ) = load_yaml_string(include_str!("../game.yml"), texture_manager, &mut canvas, controller_subsystem, &mut ttf_context);

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
        state_system.run(&mut world);
        animation_system.run(&mut world);
        graphics_system.run(&mut world);

        // Sleep
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn load_yaml_file<'a>(path: &str, mut texture_manager: TextureManager<'a>, canvas: &'a mut Canvas<Window>, gs: GameControllerSubsystem, ttf_context: &'a mut Sdl2TtfContext) -> (World, InputSystem, PhysicsSystem, EffectSystem, AnimationSystem, GraphicsSystem<'a>, StateSystem) {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).unwrap();

    load_yaml_string(&contents, texture_manager, canvas, gs, ttf_context)
}

    fn load_yaml_string<'a>(contents: &str, mut texture_manager: TextureManager<'a>, canvas: &'a mut Canvas<Window>, gs: GameControllerSubsystem, ttf_context: &'a mut Sdl2TtfContext) -> (World, InputSystem, PhysicsSystem, EffectSystem, AnimationSystem, GraphicsSystem<'a>, StateSystem) {

    let docs = YamlLoader::load_from_str(contents).unwrap();
    let doc = &docs[0];

    let world = world_from_yaml(doc, &mut texture_manager);
    let input_system = InputSystem::new(gs);
    let physics_system = PhysicsSystem::new();
    let effect_system = EffectSystem::new();
    let animation_system = AnimationSystem::new();
    let graphics_system = graphics_system_from_yaml(doc, texture_manager, canvas, ttf_context);
    let state_system = StateSystem::new();

    (world, input_system, physics_system, effect_system, animation_system, graphics_system, state_system)
}

fn world_from_yaml(doc: &Yaml, texture_manager: &mut TextureManager) -> World {
    let world_map = WorldMap::new();
    let mut world = World::new(world_map);

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

        let actions = {
            let mut action_map = HashMap::new();
            let event_iter = entity["events"].as_vec();

            if event_iter.is_none() {
                None
            } else {
                for event in event_iter.unwrap() {
                    let mut actions: Vec<(f32, Box<dyn Action>)> = Vec::new();
                    let state = event["state"].as_str().map(|e| e.to_string());
                    let action_iter = event["actions"].as_vec();

                    if action_iter.is_none() || state.is_none() {continue;}

                    for action in action_iter.unwrap() {
                        let action_type = action["type"].as_str();
                        if action_type.is_none() { continue; }
                        let delay = action["delay"].as_f64().unwrap_or(0.0) as f32;

                        match action_type.unwrap() {
                            "dialog" => {
                                let dialog = action["dialog"].as_str().unwrap().to_string();
                                let action = ShowDialog { dialog };
                                actions.push((delay, Box::new(action)));
                            }
                            "remove_state" => {
                                let state = action["state"].as_str().unwrap().to_string();
                                let action = RemoveState { state };
                                actions.push((delay, Box::new(action)));
                            }
                            "add_state" => {
                                let state = action["state"].as_str().unwrap().to_string();
                                let action = AddState { state };
                                actions.push((delay, Box::new(action)));
                            }
                            _ => continue
                        }
                    }

                    action_map.insert(state.unwrap(), Sequence::new(actions));
                }

                Some(ActionComponent::new(action_map))
            }
        };

        let id = world.add_entity(position, physics, graphics, animations, actions);
        if state.is_some() {
            world.add_entity_state(id, state.unwrap().to_string());
        }

        if player {
            world.player_id = Some(id);
        }
    }

    world.dialogs = {
        let mut dialogs = HashMap::new();

        if let Some(dialog_iter) = doc["dialogs"].as_vec() {
            for d in dialog_iter {
                let name = d["name"].as_str().map(|e| e.to_string());
                let m_iter = d["messages"].as_vec();

                if name.is_none() || m_iter.is_none() {
                    continue;
                }

                let messages = m_iter.unwrap().iter().map(|m| m.as_str().unwrap().to_string()).collect();
                dialogs.insert(name.unwrap(), Dialog::new(messages));
            }
        }

        dialogs
    };

    world
}

fn graphics_system_from_yaml<'a>(doc: &Yaml, texture_manager: TextureManager<'a>, canvas: &'a mut Canvas<Window>, ttf_context: &'a mut Sdl2TtfContext) -> GraphicsSystem<'a> {
    let mut gs = GraphicsSystem::new(texture_manager, canvas);
    gs.debug = doc["graphics"]["debug"].as_bool().unwrap_or(false);

    let dialog_path = doc["graphics"]["dialog"]["path"].as_str();
    let dialog_x = doc["graphics"]["dialog"]["renderbox"]["x"].as_i64().map(|e| e as i32);
    let dialog_y = doc["graphics"]["dialog"]["renderbox"]["y"].as_i64().map(|e| e as i32);
    let dialog_w = doc["graphics"]["dialog"]["renderbox"]["w"].as_i64().map(|e| e as u32);
    let dialog_h = doc["graphics"]["dialog"]["renderbox"]["h"].as_i64().map(|e| e as u32);

    let text_x = doc["graphics"]["dialog"]["textbox"]["x"].as_i64().map(|e| e as i32);
    let text_y = doc["graphics"]["dialog"]["textbox"]["y"].as_i64().map(|e| e as i32);
    let text_w = doc["graphics"]["dialog"]["textbox"]["w"].as_i64().map(|e| e as u32);
    let text_h = doc["graphics"]["dialog"]["textbox"]["h"].as_i64().map(|e| e as u32);

    let font = doc["graphics"]["dialog"]["font"].as_str();
    let fontsize = doc["graphics"]["dialog"]["fontsize"].as_i64().unwrap_or(12) as u16;

    let cam_x = doc["graphics"]["camera"]["x"].as_f64().map(|e| e as f32);
    let cam_y = doc["graphics"]["camera"]["y"].as_f64().map(|e| e as f32);
    let cam_w = doc["graphics"]["camera"]["w"].as_i64().map(|e| e as u32);
    let cam_h = doc["graphics"]["camera"]["h"].as_i64().map(|e| e as u32);
    let cam_zoom = doc["graphics"]["camera"]["zoom"].as_i64().map(|e| e as u32);

    if dialog_path.is_some() && dialog_x.is_some() && dialog_y.is_some() && dialog_w.is_some() && dialog_h.is_some() && text_x.is_some() && text_y.is_some() && text_w.is_some() && text_h.is_some() && font.is_some() && cam_x.is_some() && cam_y.is_some() && cam_w.is_some() && cam_h.is_some() && cam_zoom.is_some() {
        // Handle Dialog configuration
        let tex_id = gs.texture_manager.load_texture(dialog_path.unwrap());
        let renderbox = sdl2::rect::Rect::new(dialog_x.unwrap(), dialog_y.unwrap(), dialog_w.unwrap(), dialog_h.unwrap());
        let textbox = sdl2::rect::Rect::new(text_x.unwrap(), text_y.unwrap(), text_w.unwrap(), text_h.unwrap());
        let font = ttf_context.load_font(font.unwrap(), fontsize).unwrap();
        gs.dialog = Some((tex_id, renderbox, textbox, font));

        // Camera configuration
        let cam = Camera {
            x: cam_x.unwrap(),
            y: cam_y.unwrap(),
            w: cam_w.unwrap(),
            h: cam_h.unwrap(),
            zoom: cam_zoom.unwrap()
        };
        gs.camera = cam;
    }

    gs
}

