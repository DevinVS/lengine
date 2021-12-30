use std::collections::HashMap;
use std::collections::HashSet;
use sdl2::render::Canvas;
use sdl2::video::{Window, WindowContext};
use sdl2::image::LoadTexture;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use crate::geometry::{GeometryComponent, Rect};
use crate::world::World;

pub struct GraphicsComponent {
    pub texture_id: usize,
    pub offset: (f32, f32, i32, i32),
    pub flipped: bool,
}

impl GraphicsComponent {
    pub fn new(tex_id: usize) -> GraphicsComponent {
        GraphicsComponent {
            texture_id: tex_id,
            flipped: false,
            offset: (0.0, 0.0, 0, 0)
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
    fn follow(&mut self, entity: (&GeometryComponent, &GraphicsComponent)) {
    }

    // Draw an entity based on its position and texture
    pub fn draw_entity(&mut self, entity: (&HashSet<String>, &GeometryComponent, &GraphicsComponent)) {
        let tex_id = entity.2.texture_id;
        let flipped = entity.2.flipped;
        let texture = self.texture_manager.get_texture(tex_id).unwrap();

        let mut entity_rect = entity.1.rect().clone();
        entity_rect.x -= self.camera.x as f32 + entity.2.offset.0;
        entity_rect.y -= self.camera.y as f32 + entity.2.offset.1;

        entity_rect.w = (entity_rect.w as i32 + entity.2.offset.2) as u32;
        entity_rect.h = (entity_rect.h as i32 + entity.2.offset.3) as u32;

        entity_rect.x *= self.camera.zoom as f32;
        entity_rect.y *= self.camera.zoom as f32;
        entity_rect.w *= self.camera.zoom;
        entity_rect.h *= self.camera.zoom;

        self.canvas.copy_ex(texture, None, entity_rect.sdl2(), 0.0, None, flipped, false).unwrap();
    }

    // Run the system
    pub fn run(&mut self, world: &mut World) {
        self.canvas.clear();

        // if let Some(player) = world.get_player() {
        //     self.follow(&player);
        // }

        let mut drawables: Vec<(usize, (_, &GeometryComponent, &GraphicsComponent))> = world.graphics().collect();

        drawables.sort_by_key(|e| {
            let r = e.1.1.rect();
            r.y as i32+r.h as i32
        });


        drawables.iter().for_each(|e| {
            self.draw_entity(e.1);
        });

        self.canvas.present();
    }
}
