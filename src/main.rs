use std::collections::HashMap;
use std::{process::exit, time::Duration};
use game::animation::Animation;

use game::{animation::AnimationSystem, effect::EffectsSystem, geometry::GeometryComponent, graphics::{GraphicsComponent, GraphicsSystem, TextureManager}, input::InputSystem, map::WorldMap, physics::{PhysicsComponent, PhysicsSystem}, vector::Vector, world::World};
use game::animation::AnimationComponent;
use sdl2::{event::Event, image::InitFlag, keyboard::Keycode};

fn main() {
    // Create context and relevant subsystems
    let sdl2_context = sdl2::init().unwrap();
    let video_subsystem = sdl2_context.video().unwrap();
    let audio_subsystem = sdl2_context.audio().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();
    let joystick_subsystem = sdl2_context.joystick().unwrap();
    let controller_subsystem = sdl2_context.game_controller().unwrap();

    // Create graphics objects such as window, canvas, and texture manager
    let window = video_subsystem.window("title", 800, 600)
        .vulkan()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().accelerated().build().unwrap();
    let mut event_pump = sdl2_context.event_pump().unwrap();

    let texture_creator = canvas.texture_creator();
    let mut texture_manager = TextureManager::new(&texture_creator);

    // Load Game Data
    let world_map = WorldMap::new();
    let mut world = World::new(world_map);

    // Example loading
    let tex0 = texture_manager.load_texture("./assets/0.png");
    let tex1 = texture_manager.load_texture("./assets/1.png");
    let tex2 = texture_manager.load_texture("./assets/3.png");
    let tex3 = texture_manager.load_texture("./assets/4.png");
    let tex4 = texture_manager.load_texture("./assets/5.png");
    let tex5 = texture_manager.load_texture("./assets/6.png");
    let tex6 = texture_manager.load_texture("./assets/walk.png");
    let tex7 = texture_manager.load_texture("./assets/collide.png");
    let box_tex = texture_manager.load_texture("./assets/box.png");

    let idle_animation = Animation::new(
        vec![tex0, tex1, tex2, tex3, tex4, tex5, tex4, tex3, tex2, tex1],
        0.05,
    );

    let walking_animation = Animation::new(
        vec![tex6],
        10.0,
    );

    let collide_animation = Animation::new(
        vec![tex7],
        10.0
    );

    let mut a_map = HashMap::new();
    a_map.insert("idle".into(), idle_animation);
    a_map.insert("walking".into(), walking_animation);
    a_map.insert("colliding".into(), collide_animation);

    let test = world.add_entity(
        Some(GeometryComponent::new(0.0, 0.0, 20, 20)),
        Some(PhysicsComponent::new(2, true)),
        Some(GraphicsComponent::new(tex1)),
        Some(AnimationComponent::new(a_map))
    );

    world.add_entity_state(test, "idle".to_string());
    world.player_id = Some(test);

    world.add_entity(
        Some(GeometryComponent::new(30.0, 30.0, 30, 30)),
        Some(PhysicsComponent::new(20, false)),
        Some(GraphicsComponent::new(box_tex)),
        None
    );

    // Create Game Systems
    let mut input_system = InputSystem::new(
        controller_subsystem,
        joystick_subsystem
    );
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

        // Sleep
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
