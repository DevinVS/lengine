use std::time::Duration;

use game::{entity::Entity, graphics::{GraphicsSystem, TextureManager}, input::InputSystem, map::WorldMap, world::World};
use sdl2::{image::InitFlag};

fn main() {
    // Create context and relevant subsystems
    let sdl2_context = sdl2::init().unwrap();
    let video_subsystem = sdl2_context.video().unwrap();
    let audio_subsystem = sdl2_context.audio().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();

    // Create graphics objects such as window, canvas, and texture manager
    let window = video_subsystem.window("title", 600, 800)
        .position_centered()
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
    let tex_id = texture_manager.load_texture("./assets/ginny.png");
    let entity = Entity::new_drawable(0, 100, 100, 100, 100, tex_id);
    world.add_entity(0, entity);

    // Create Game Systems
    let mut graphics_system = GraphicsSystem::new(texture_manager, &mut canvas);
    let mut input_system = InputSystem::new(&mut event_pump);

    // Run Game Loop
    loop {
        input_system.run(&mut world);
        graphics_system.run(&mut world);
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

    }
}