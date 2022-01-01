use std::collections::HashMap;
use std::collections::HashSet;
use sdl2::render::Canvas;
use sdl2::video::{Window, WindowContext};
use sdl2::image::LoadTexture;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use crate::animation::AnimationComponent;
use crate::geometry::PositionComponent;
use crate::physics::PhysicsComponent;
use crate::world::World;
use crate::geometry::Rect;

#[derive(Debug)]
pub struct GraphicsComponent {
    pub texture_id: usize,
    pub srcbox: Option<sdl2::rect::Rect>,
    pub renderbox: Rect,
    pub flipped: bool,
}

impl GraphicsComponent {
    pub fn new(tex_id: usize, renderbox: Rect, srcbox: Option<sdl2::rect::Rect>) -> GraphicsComponent {
        GraphicsComponent {
            texture_id: tex_id,
            flipped: false,
            renderbox,
            srcbox
        }
    }
}

pub struct Camera {
    x: i32,
    y: i32,
    zoom: u32   // Pixels per in game units
}

// Manages loading and keeping track of textures
pub struct TextureManager<'a> {
    next_texture_id: usize,
    textures: HashMap<usize, Texture<'a>>,
    texture_creator: &'a TextureCreator<WindowContext>
}

impl<'a> TextureManager<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> TextureManager<'a> {
        TextureManager {
            next_texture_id: 0,
            textures: HashMap::new(),
            texture_creator
        }
    }

    pub fn load_texture(&mut self, path: &str) -> usize {
        let id = self.next_texture_id;
        self.next_texture_id += 1;

        let tex = self.texture_creator.load_texture(path).unwrap();
        self.textures.insert(id, tex);

        id
    }

    pub fn get_texture(&mut self, id: usize) -> Option<&Texture<'a>> {
        self.textures.get(&id)
    }
}


// The actual rendering system, uses GraphicsState
pub struct GraphicsSystem<'a> {
    texture_manager: TextureManager<'a>,
    canvas: &'a mut Canvas<Window>,
    camera: Camera
}

impl<'a> GraphicsSystem<'a> {
    pub fn new(texture_manager: TextureManager<'a>, canvas: &'a mut Canvas<Window>) -> GraphicsSystem<'a> {
        GraphicsSystem {
            texture_manager,
            canvas,
            camera: Camera {x: -50, y: -50, zoom: 5}
        }
    }

    // Make the Camera follow the entity
    fn follow(&mut self, entity: (&PositionComponent, &GraphicsComponent)) {
    }

    // Draw an entity based on its position and texture
    pub fn draw_entity(&mut self, entity: (&HashSet<String>, &PositionComponent, &GraphicsComponent), physics: Option<&PhysicsComponent>) {
        let tex_id = entity.2.texture_id;
        let flipped = entity.2.flipped;
        let texture = self.texture_manager.get_texture(tex_id).unwrap();

        let mut entity_rect = entity.2.renderbox.after_position(entity.1);

        entity_rect.x *= self.camera.zoom as f32;
        entity_rect.y *= self.camera.zoom as f32;
        entity_rect.w *= self.camera.zoom;
        entity_rect.h *= self.camera.zoom;

        self.canvas.copy_ex(texture, entity.2.srcbox, entity_rect.sdl2(), 0.0, None, flipped, false).unwrap();

        // Draw hitbox
        if physics.is_some() {
            let mut hitbox = physics.unwrap().hitbox.after_position(entity.1);

            hitbox.x *= self.camera.zoom as f32;
            hitbox.y *= self.camera.zoom as f32;
            hitbox.w *= self.camera.zoom;
            hitbox.h *= self.camera.zoom;

            self.canvas.set_draw_color((255, 0, 0));
            self.canvas.draw_rect(hitbox.sdl2()).unwrap();
            self.canvas.set_draw_color((0, 0, 0));
        }
    }

    // Run the system
    pub fn run(&mut self, world: &mut World) {
        self.canvas.clear();

        // if let Some(player) = world.get_player() {
        //     self.follow(&player);
        // }

        let mut drawables: Vec<(usize, (_, &PositionComponent, &GraphicsComponent))> = world.graphics().collect();


        drawables.sort_by_key(|e| {
            let r = e.1.2.renderbox.after_position(e.1.1);
            r.y as i32+r.h as i32
        });


        drawables.iter().for_each(|e| {
            let physics = world.get_entity_physics(e.0);
            self.draw_entity(e.1, physics.1);
        });

        self.canvas.present();
    }
}
