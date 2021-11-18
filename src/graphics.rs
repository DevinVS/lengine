use std::collections::HashMap;
use std::sync::Arc;

use sdl2::{EventPump, VideoSubsystem};
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::{Window, WindowContext};
use sdl2::image::{InitFlag, LoadTexture, Sdl2ImageContext};
use sdl2::Sdl;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use crate::entity::Entity;
use crate::world::World;

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
    canvas: &'a mut Canvas<Window>
}

impl<'a> GraphicsSystem<'a> {
    pub fn new(texture_manager: TextureManager<'a>, canvas: &'a mut Canvas<Window>) -> GraphicsSystem<'a> {
        GraphicsSystem {
            texture_manager,
            canvas
        }
    }

    // Draw an entity based on its position and texture
    pub fn draw_entity(&mut self, entity: &Entity) {
        let texture = self.texture_manager.get_texture(entity.texture_id().unwrap()).unwrap();
        self.canvas.copy(texture, None, None).unwrap();
    }

    // Run the system
    pub fn run(&mut self, world: &mut World) {
        self.canvas.clear();

        world.drawables().for_each(|e| {
            self.draw_entity(e);
        });

        self.canvas.present();
    }
}