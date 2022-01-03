use std::collections::HashMap;
use std::collections::HashSet;
use sdl2::render::Canvas;
use sdl2::render::TextureQuery;
use sdl2::ttf::Font;
use sdl2::video::{Window, WindowContext};
use sdl2::image::LoadTexture;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use crate::dialog::Dialog;
use crate::geometry::PositionComponent;
use crate::physics::PhysicsComponent;
use crate::world::World;
use crate::geometry::Rect;

/// Component for rendering a single entity
#[derive(Debug)]
pub struct GraphicsComponent {
    /// Index of the texture to render
    pub texture_id: usize,
    /// Source coordinates for the texture
    pub srcbox: Option<sdl2::rect::Rect>,
    /// Coordinates to render inside the game world
    pub renderbox: Rect,
    /// Whether to flip the texture
    pub flipped: bool
}

impl GraphicsComponent {
    /// Create a new GraphicsComponent
    pub fn new(tex_id: usize, renderbox: Rect, srcbox: Option<sdl2::rect::Rect>) -> GraphicsComponent {
        GraphicsComponent {
            texture_id: tex_id,
            flipped: false,
            renderbox,
            srcbox
        }
    }
}

/// Camera to view the game world through
pub struct Camera {
    /// x coordinate of the camera in the game world
    x: f32,
    /// y coordinate of the camera in the game world
    y: f32,
    /// Width of the camera in actual screen pixels
    w: u32,
    /// Height of the camera in actual screen pixels
    h: u32,
    /// Pixel scaling factor, ie conversion factor between world units and screen pixels
    zoom: u32
}

impl Camera {
    /// Find the new rectangle with respect to the view of the camera
    fn view(&self, rect: Rect, (width, height): (u32, u32)) -> Rect {
        let screen_x = (width - self.w) / 2;
        let screen_y = (height - self.h) / 2;

        Rect::new(
            (rect.x-self.x) * self.zoom as f32 + screen_x as f32,
            (rect.y-self.y) * self.zoom as f32 + screen_y as f32,
            rect.w * self.zoom,
            rect.h * self.zoom
        )
    }

    /// Cover the world outside the camera's view with black bars
    fn render(&self, canvas: &mut Canvas<Window>) {
        let (width, height) = canvas.window().size();
        let left_offset = (width - self.w) / 2;
        let top_offset = (height - self.h) / 2;
        let right_offset = width - left_offset;
        let bottom_offset = height - top_offset;

        let old_color = canvas.draw_color();
        canvas.set_draw_color((0, 0, 0));
        canvas.fill_rect(sdl2::rect::Rect::new(0, 0, width, top_offset as u32)).unwrap();
        canvas.fill_rect(sdl2::rect::Rect::new(0, 0, left_offset as u32, height)).unwrap();
        canvas.fill_rect(sdl2::rect::Rect::new(0, bottom_offset as i32, width, top_offset as u32)).unwrap();
        canvas.fill_rect(sdl2::rect::Rect::new(right_offset as i32, 0, left_offset as u32, height)).unwrap();
        canvas.set_draw_color(old_color);
    }
}

/// Manages loading and keeping track of textures
pub struct TextureManager<'a> {
    /// Index to give a newly created texture
    next_texture_id: usize,
    /// Hashmap of texture indices to actual textures
    textures: HashMap<usize, Texture<'a>>,
    /// Sdl texture creation struct
    texture_creator: &'a TextureCreator<WindowContext>
}

impl<'a> TextureManager<'a> {
    /// Create a new texture manager
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> TextureManager<'a> {
        TextureManager {
            next_texture_id: 0,
            textures: HashMap::new(),
            texture_creator
        }
    }

    /// Read a texture from disk into memory and returns its index to reference later
    pub fn load_texture(&mut self, path: &str) -> usize {
        let id = self.next_texture_id;
        self.next_texture_id += 1;

        let tex = self.texture_creator.load_texture(path).unwrap();
        self.textures.insert(id, tex);

        id
    }

    /// Get a texture from its index
    pub fn get_texture(&mut self, id: usize) -> Option<&Texture<'a>> {
        self.textures.get(&id)
    }
}


/// The actual rendering system, uses GraphicsState
pub struct GraphicsSystem<'a> {
    /// Collection and management of textures
    pub texture_manager: TextureManager<'a>,
    /// Rendering surface, does all drawing
    canvas: &'a mut Canvas<Window>,
    /// Camera to view the world through
    camera: Camera,
    /// Display debug information such as hitboxes
    pub debug: bool,
    /// Currently rendered dialog box
    pub dialog: Option<(usize, sdl2::rect::Rect, sdl2::rect::Rect, Font<'a, 'a>)>,
}

impl<'a> GraphicsSystem<'a> {
    /// Create a new GraphicsSystem
    pub fn new(texture_manager: TextureManager<'a>, canvas: &'a mut Canvas<Window>) -> GraphicsSystem<'a> {
        GraphicsSystem {
            texture_manager,
            canvas,
            camera: Camera {x: 0.0, y: 0.0, w: 800, h: 600, zoom: 5},
            debug: false,
            dialog: None,
        }
    }

    /// Make the Camera follow a given rectangle
    fn follow(&mut self, rect: Rect) {
        let cam_left = self.camera.x;
        let cam_right = self.camera.x + self.camera.w as f32 / self.camera.zoom as f32;
        let cam_top = self.camera.y;
        let cam_bottom = self.camera.y + self.camera.h as f32 / self.camera.zoom as f32;

        let rect_left = rect.x;
        let rect_right = rect.x + rect.w as f32;
        let rect_top = rect.y;
        let rect_bottom = rect.y + rect.h as f32;

        if rect_left < cam_left {
            self.camera.x = rect_left;
        }

        if rect_right > cam_right {
            self.camera.x += rect_right - cam_right;
        }

        if rect_top < cam_top {
            self.camera.y = rect_top;
        }

        if rect_bottom > cam_bottom {
            self.camera.y += rect_bottom - cam_bottom;
        }
    }

    /// Draw an entity based on its position and texture
    pub fn draw_entity(&mut self, entity: (&HashSet<String>, &PositionComponent, &GraphicsComponent), physics: Option<&PhysicsComponent>) {
        let tex_id = entity.2.texture_id;
        let flipped = entity.2.flipped;
        let texture = self.texture_manager.get_texture(tex_id).unwrap();

        let entity_rect = self.camera.view(entity.2.renderbox.after_position(entity.1), self.canvas.window().size());

        self.canvas.copy_ex(texture, entity.2.srcbox, entity_rect.sdl2(), 0.0, None, flipped, false).unwrap();

        // Draw hitbox
        if self.debug && physics.is_some() {
            let hitbox = self.camera.view(physics.unwrap().hitbox.after_position(entity.1), self.canvas.window().size());

            self.canvas.set_draw_color((255, 0, 0));
            self.canvas.draw_rect(hitbox.sdl2()).unwrap();
            self.canvas.set_draw_color((255, 255, 255));
        }
    }

    /// Draw all renderable entities
    pub fn run(&mut self, world: &mut World) {
        self.canvas.clear();

        if let Some(player_id) = world.player_id {
            if let (Some(pos), Some(phys)) = world.get_entity_physics(player_id) {
                self.follow(phys.hitbox.after_position(pos));
            }
        }

        let mut drawables: Vec<(usize, (_, &PositionComponent, &GraphicsComponent))> = world.graphics().collect();


        drawables.sort_by_key(|e| {
            let r = e.1.2.renderbox.after_position(e.1.1);
            r.y as i32+r.h as i32
        });

        // Draw Entities
        drawables.iter().for_each(|e| {
            let physics = world.get_entity_physics(e.0);
            self.draw_entity(e.1, physics.1);
        });

        // Draw Dialog If Exists
        if self.dialog.is_some() {
            if let Some(dialog) = world.current_dialog() {
                self.render_dialog(dialog);
            }
        }

        // Draw Camera Borders
        self.camera.render(self.canvas);
        self.canvas.present();
    }

    /// Render a dialog window
    fn render_dialog(&mut self, dialog: &Dialog) {
        let (screen_width, screen_height) = self.canvas.window().size();
        let left_offset = ((screen_width - self.camera.w) / 2) as i32;
        let top_offset = ((screen_height - self.camera.h) / 2) as i32;

        // Draw Box
        let (tex_id, r, t, font) = self.dialog.as_ref().unwrap();
        let tex = self.texture_manager.get_texture(*tex_id).unwrap();
        self.canvas.copy(tex, None, sdl2::rect::Rect::new(left_offset+r.x, top_offset+r.y, r.width(), r.height())).unwrap();

        // Draw Text
        let msg = dialog.msg();
        let surface = font.render(&msg).blended_wrapped((46, 53, 42), t.width()).unwrap();
        let tex = self.texture_manager.texture_creator.create_texture_from_surface(&surface).unwrap();

        let TextureQuery { width, height, .. } = tex.query();

        self.canvas.copy(
            &tex,
            None,
            sdl2::rect::Rect::new(
                left_offset+r.x+t.x,
                top_offset+r.y+t.y,
                width,
                height
            )
        ).unwrap();

    }
}


