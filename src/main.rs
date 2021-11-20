use std::time::Duration;

use game::{animation::AnimationSystem, effect::EffectsSystem, entity::{Entity, EntityBuilder}, graphics::{EntityGraphicsState, GraphicsSystem, TextureManager}, input::InputSystem, map::WorldMap, physics::{EntityPhysicsState, PhysicsSystem}, vector::Vector, world::World};
use sdl2::{image::InitFlag};

fn main() {
    // Create context and relevant subsystems
    let sdl2_context = sdl2::init().unwrap();
    let video_subsystem = sdl2_context.video().unwrap();
    let audio_subsystem = sdl2_context.audio().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();

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
    let player_tex_id = texture_manager.load_texture("./assets/ginny.png");
    let player = Entity::builder()
        .x(0)
        .y(0)
        .width(60)
        .height(60)
        .graphics_state(EntityGraphicsState{texture_id:player_tex_id})
        .physics_state(EntityPhysicsState::new(60.0))
        .build()
        .unwrap();
    
    let player_id = world.add_entity(player);
    world.player_id = Some(player_id);

    let box_tex_id = texture_manager.load_texture("./assets/box.png");
    let box_e = Entity::builder()
        .x(500)
        .y(200)
        .width(50)
        .height(50)
        .graphics_state(EntityGraphicsState { texture_id: box_tex_id })
        .physics_state(EntityPhysicsState::new(600.0))
        .build()
        .unwrap();

    world.add_entity(box_e);


    // Create Game Systems
    let mut input_system = InputSystem::new(&mut event_pump);
    let mut physics_system = PhysicsSystem::new();
    let mut effects_system = EffectsSystem::new();
    let mut animation_system = AnimationSystem::new();
    let mut graphics_system = GraphicsSystem::new(texture_manager, &mut canvas);

    // Run Game Loop
    loop {
        input_system.run(&mut world);
        physics_system.run(&mut world);
        effects_system.run(&mut world);
        animation_system.run(&mut world);
        graphics_system.run(&mut world);
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}