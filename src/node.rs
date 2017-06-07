extern crate sdl2;

use std::ops::{Deref, DerefMut};

use std::path::Path;
use std::vec::Vec;
use std::rc::Rc;
use std::cell::RefCell;

use sdl2::rect::Rect;
use sdl2::render::Renderer;
use sdl2::render::Texture;
use sdl2::render::BlendMode;
use sdl2::image::LoadTexture;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct Node {
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    visible: bool,
    texture: Rc<Texture>,
}

impl Node {
    pub fn new (renderer: &Renderer, path: &str) -> Node {
        let mut texture = renderer.load_texture(
            Path::new(path)
            ).unwrap();

        let tquery = texture.query();
        let rc_texture = Rc::new(texture);
        Node {
            x: 0,
            y: 0,
            w: tquery.width,
            h: tquery.height,
            visible: true,
            texture: rc_texture,
        }
    }

    pub fn reset(&mut self, start_x: i32, start_y: i32) {
        self.x = start_x;
        self.y = start_y;
    }
    
    pub fn reseize(&mut self, w: u32, h: u32) {
        self.w = w;
        self.h = h;
    }

    pub fn paint(&self, renderer: &mut Renderer) {
        let rect = Rect::new(self.x, self.y, self.w, self.h);
        renderer
            .copy_ex(&self.texture, 
                     None, Some(rect), 0.0, None, false, false)
            .expect("Single star particle should have rendered.");
    }

    pub fn blend_mode_none(&mut self) {
        Rc::get_mut(&mut self.texture).unwrap()
            .set_blend_mode(BlendMode::None); 
    }
    pub fn blend_mode_add(&mut self) {
        Rc::get_mut(&mut self.texture).unwrap()
            .set_blend_mode(BlendMode::Add); 
    }
    pub fn blend_mode_mod(&mut self) {
        Rc::get_mut(&mut self.texture).unwrap()
            .set_blend_mode(BlendMode::Mod); 
    }
    pub fn blend_mode_blend(&mut self) {
        Rc::get_mut(&mut self.texture).unwrap()
            .set_blend_mode(BlendMode::Blend); 
    }
}
