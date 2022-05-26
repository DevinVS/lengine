use std::process::exit;
use std::time::Duration;

use game::animation::AnimationSystem;
use game::effect::EffectSystem;
use game::input::InputSystem;
use game::physics::PhysicsSystem;
use game::state::StateSystem;
use sdl2::event::{Event, WindowEvent};
use sdl2::image::InitFlag;
use sdl2::keyboard::Keycode;

use game::graphics::{TextureManager, GraphicsSystem};
use game::parser::parse_game_file;


fn main() {
    // Create context and relevant subsystems
    let sdl2_context = sdl2::init().unwrap();
    let video_subsystem = sdl2_context.video().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();
    let controller_subsystem = sdl2_context.game_controller().unwrap();

    // Create graphics objects such as window, canvas, and texture manager
    let mut window = video_subsystem.window("title", 1000, 800)
        .resizable()
        .build()
        .unwrap();

    window.set_minimum_size(800, 600).unwrap();

    let mut canvas = window.into_canvas().accelerated().build().unwrap();
    let mut event_pump = sdl2_context.event_pump().unwrap();

    canvas.set_draw_color((255, 255, 255));

    let texture_creator = canvas.texture_creator();
    let texture_manager = TextureManager::new(&texture_creator);

    let (mut world, input_config, graphics_config) = parse_game_file("./game.yml", texture_manager);

    // Create Game Systems
    let mut input_system = InputSystem::new(input_config, controller_subsystem);
    let mut physics_system = PhysicsSystem::new();
    let mut graphics_system = GraphicsSystem::new(graphics_config, &ttf_context, &mut canvas);
    let mut animation_system = AnimationSystem::new();
    let mut effects_system = EffectSystem::new();
    let mut state_system = StateSystem::new();

    // Run Game Loop
    loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    exit(0);
                },
                Event::Window { win_event: WindowEvent::Resized(_, _), .. } => {
                    graphics_system.refresh();
                }
                _ => {input_system.handle_event(event)}
            }
        }

        // Run all subsystems
        input_system.run(&mut world);
        physics_system.run(&mut world);
        state_system.run(&mut world);
        animation_system.run(&mut world);
        graphics_system.run(&mut world);
        effects_system.run(&mut world);

        // Check if the player is being moved to another world
        let player_states = world.states[0].clone();
        for state in player_states {
            if state.starts_with("__MOVE_TO__=") {
                let s = state.replace("__MOVE_TO__=", "");
                let (file, entrance) = s.split_once("/").unwrap();
                world.deload();
                world.load(file, entrance);

                world.states[0].remove(&state);
                break;
            }
        }

        // Sleep
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
