use std::collections::HashMap;
use std::collections::HashSet;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::render::TextureQuery;
use sdl2::ttf::Font;
use sdl2::ttf::Sdl2TtfContext;
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
#[derive(Debug)]
pub struct Camera {
    /// Rect of the camera in the world
    pub rect: Rect,
    /// Box that the player must reside in and the camera will move with the player
    pub player_box: Rect,
    /// Pixel scaling factor, ie conversion factor between world units and screen pixels
    pub zoom: u32
}

impl Camera {
    /// Find the new rectangle with respect to the view of the camera
    fn view(&self, rect: Rect, (width, height): (u32, u32)) -> Rect {
        let screen_x = (width - self.rect.w) / 2;
        let screen_y = (height - self.rect.h) / 2;

        Rect::new(
            (rect.x-self.rect.x) * self.zoom as f32 + screen_x as f32,
            (rect.y-self.rect.y) * self.zoom as f32 + screen_y as f32,
            rect.w * self.zoom,
            rect.h * self.zoom
        )
    }

    /// Cover the world outside the camera's view with black bars
    fn render(&self, canvas: &mut Canvas<Window>) {
        let (width, height) = canvas.window().size();
        let left_offset = (width - self.rect.w) / 2;
        let top_offset = (height - self.rect.h) / 2;
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

/// Configuration for the graphics system,
/// created by parsing yaml file
#[derive(Debug)]
pub struct GraphicsConfig {
    pub camera: Camera,
    pub debug: bool,
    pub dialog_tex_path: Option<String>,
    pub dialog_font_path: Option<String>,
    pub dialog_font_size: Option<u16>,
    pub dialog_textbox: Option<sdl2::rect::Rect>,
    pub dialog_renderbox: Option<sdl2::rect::Rect>
}

/// Configuration for rendering the Dialog
pub struct DialogConfig<'a> {
    tex_id: usize,
    renderbox: sdl2::rect::Rect,
    textbox: sdl2::rect::Rect,
    font: Font<'a, 'a>
}

impl<'a> DialogConfig<'a> {
    /// Create a DialogConfig from a GraphicsConfig struct
    fn from_graphics_config(gc: &GraphicsConfig, texture_manager: &mut TextureManager, ttf_context: &'a Sdl2TtfContext) -> Option<DialogConfig<'a>> {
        if gc.dialog_tex_path.is_none() || gc.dialog_font_path.is_none() || gc.dialog_font_size.is_none() || gc.dialog_renderbox.is_none() || gc.dialog_textbox.is_none() {
            None
        } else {
            let tex_id = texture_manager.load_texture(gc.dialog_tex_path.as_ref().unwrap());
            let font = ttf_context.load_font(gc.dialog_font_path.as_ref().unwrap(), gc.dialog_font_size.unwrap()).unwrap();

            Some(DialogConfig {
                tex_id,
                font,
                renderbox: gc.dialog_renderbox.unwrap(),
                textbox: gc.dialog_textbox.unwrap()
            })
        }
    }
}


/// The actual rendering system, uses GraphicsState
pub struct GraphicsSystem<'a> {
    /// Collection and management of textures
    pub texture_manager: TextureManager<'a>,
    /// Rendering surface, does all drawing
    canvas: &'a mut Canvas<Window>,
    /// Camera to view the world through
    pub camera: Camera,
    /// Display debug information such as hitboxes
    pub debug: bool,
    /// Dialog Settings
    /// (texture id, renderbox, textbox, Font)
    pub dialog: Option<DialogConfig<'a>>,
}

impl<'a> GraphicsSystem<'a> {
    /// Create a new GraphicsSystem from a GraphicsConfig
    pub fn new(config: GraphicsConfig, mut texture_manager: TextureManager<'a>, ttf_context: &'a Sdl2TtfContext, canvas: &'a mut Canvas<Window>) -> GraphicsSystem<'a> {
        let dialog_config = DialogConfig::from_graphics_config(&config, &mut texture_manager, ttf_context);

        GraphicsSystem {
            texture_manager,
            canvas,
            camera: config.camera,
            debug: config.debug,
            dialog: dialog_config
        }
    }

    /// Make the Camera follow a given rectangle
    fn follow(&mut self, rect: Rect) {
        // Bounding box
        let box_x_offset = self.camera.player_box.x / self.camera.zoom as f32;
        let box_y_offset = self.camera.player_box.y / self.camera.zoom as f32;
        let box_width = self.camera.player_box.w as f32 / self.camera.zoom as f32;
        let box_height = self.camera.player_box.h as f32 / self.camera.zoom as f32;

        let box_left = self.camera.rect.x + box_x_offset;
        let box_right = box_left + box_width;
        let box_top = self.camera.rect.y + box_y_offset;
        let box_bottom = box_top + box_height;

        let rect_left = rect.x;
        let rect_right = rect.x + rect.w as f32;
        let rect_top = rect.y;
        let rect_bottom = rect.y + rect.h as f32;

        if rect_left < box_left {
            self.camera.rect.x = rect_left - box_x_offset;
        }

        if rect_right > box_right {
            self.camera.rect.x = rect_right - box_width - box_x_offset;
        }

        if rect_top < box_top {
            self.camera.rect.y = rect_top - box_y_offset;
        }

        if rect_bottom > box_bottom {
            self.camera.rect.y = rect_bottom - box_height - box_y_offset;
        }
    }

    /// Draw an entity based on its position and texture
    pub fn draw_entity(&mut self, entity: (&HashSet<String>, &PositionComponent, &GraphicsComponent), physics: Option<&PhysicsComponent>) {
        let tex_id = entity.2.texture_id;
        let flipped = entity.2.flipped;
        let texture = self.texture_manager.get_texture(tex_id).unwrap();

        let entity_rect = self.camera.view(entity.2.renderbox.after_position(entity.1), self.canvas.window().size());

        self.canvas.copy_ex(texture, entity.2.srcbox, entity_rect.sdl2(), 0.0, None, flipped, false).unwrap();
    }

    /// Draw all renderable entities
    pub fn run(&mut self, world: &mut World) {
        // Set background color
        self.canvas.set_draw_color(world.background_color);

        self.canvas.clear();

        if let Some(player_id) = world.player_id {
            if let (Some(pos), Some(phys)) = world.get_entity_physics(player_id) {
                self.follow(phys.hitbox.after_position(pos));
            }
        }

        // Draw background if exists
        if let Some(background) = world.background.as_ref() {
            let (width, height) = self.canvas.window().size();
            let left = (width - self.camera.rect.w) as f32 / 2.0 - self.camera.rect.x * self.camera.zoom as f32;
            let top = (height - self.camera.rect.h) as f32 / 2.0 - self.camera.rect.y * self.camera.zoom as f32;
            let renderbox = background.renderbox.after_position(&PositionComponent::new(left, top)).sdl2();
            let tex = self.texture_manager.get_texture(background.texture_id).unwrap();
            self.canvas.copy(tex, None, renderbox).unwrap();
        }

        let mut drawables: Vec<(usize, (_, &PositionComponent, &GraphicsComponent))> = world.graphics().collect();

        // Sort entities by the bottom of their rects
        drawables.sort_by_key(|e| {
            let r = e.1.2.renderbox.after_position(e.1.1);
            r.y as i32+r.h as i32
        });

        // Draw Entities
        drawables.iter().for_each(|e| {
            if !e.1.0.contains(&"invisible".to_string()) {
                let physics = world.get_entity_physics(e.0);
                self.draw_entity(e.1, physics.1);
            }
        });

        // Draw hitboxes if we are in debug mode
        if self.debug {
            self.canvas.set_draw_color(Color::RED);
            for i in 0..world.states.len() {
                if world.physics[i].is_some() && world.positions[i].is_some() {
                    let rect = self.camera.view(
                        world.physics[i].as_ref().unwrap().hitbox
                            .after_position(
                                world.positions[i].as_ref().unwrap()
                            ),
                        self.canvas.window().size()
                    );

                    self.canvas.draw_rect(rect.sdl2()).unwrap();
                }
            }
        }

        // Draw Dialog If Exists
        if self.dialog.is_some() {
            if let Some(dialog) = world.current_dialog() {
                self.render_dialog(dialog);
            }
        }

        // Draw effects if we are in debug mode
        if self.debug {
            self.canvas.set_draw_color(Color::MAGENTA);
            for effect in world.effects.iter() {
                let rect = self.camera.view(effect.rect, self.canvas.window().size());
                self.canvas.draw_rect(rect.sdl2()).unwrap();
            }
        }

        // Draw Camera Borders
        self.camera.render(self.canvas);
        self.canvas.present();
    }

    /// Render a dialog window
    fn render_dialog(&mut self, dialog: &Dialog) {
        let (screen_width, screen_height) = self.canvas.window().size();
        let left_offset = ((screen_width - self.camera.rect.w) / 2) as i32;
        let top_offset = ((screen_height - self.camera.rect.h) / 2) as i32;

        // Draw Box
        let d = self.dialog.as_ref().unwrap();
        let tex = self.texture_manager.get_texture(d.tex_id).unwrap();
        self.canvas.copy(
            tex,
            None,
            sdl2::rect::Rect::new(
                left_offset+d.renderbox.x,
                top_offset+d.renderbox.y,
                d.renderbox.width(),
                d.renderbox.height()
            )
        ).unwrap();

        // Draw Text
        let msg = dialog.msg();
        let surface = d.font.render(&msg).blended_wrapped((255, 255, 255), d.textbox.width()).unwrap();
        let tex = self.texture_manager.texture_creator.create_texture_from_surface(&surface).unwrap();

        let TextureQuery { width, height, .. } = tex.query();

        self.canvas.copy(
            &tex,
            None,
            sdl2::rect::Rect::new(
                left_offset+d.renderbox.x+d.textbox.x,
                top_offset+d.renderbox.y+d.textbox.y,
                width,
                height
            )
        ).unwrap();

    }
}


