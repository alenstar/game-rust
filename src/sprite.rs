extern crate sdl2;

use std::ops::{Deref, DerefMut};

use std::path::Path;
use std::vec::Vec;
use std::rc::Rc;

use sdl2::rect::Rect;
use sdl2::render::Renderer;
use sdl2::render::Texture;
use sdl2::render::BlendMode;
use sdl2::image::LoadTexture;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use node::Node;
use display::Displayable;
use animation::Animation;

pub struct Sprite {
    // x: i32,
    // y: i32,
    // w: u32,
    // h: u32,
    // visible: bool,
    // texture: Rc<Texture>,
    // /
    // cursor: u32,
    // nodes: Vec<Rc<Node>>,
    running: bool,
    node: Node,
}

impl Sprite {
    pub fn new(renderer: &Renderer, paths: &[&str]) -> Sprite {
        // let mut children: Vec<Rc<Node>> = Vec::new();
        Sprite {
            running: false,
            node: Node::new(renderer, paths),
        }
    }

    pub fn start(&mut self) {
        self.running = true;
    }

    pub fn stop(&mut self) {
        self.running = false;
        // self.node.cursor = 0;
    }

    pub fn update(&mut self) {
        if self.running {
            self.node.update();
        }
    }

    // pub fn reset(&mut self, start_x: i32, start_y: i32) {
    // self.x = start_x;
    // self.y = start_y;
    // }
    //
    // pub fn reseize(&mut self, w: u32, h: u32) {
    // self.w = w;
    // self.h = h;
    // }
    //
    // pub fn blend_mode_none(&mut self) {
    // Rc::get_mut(&mut self.texture).unwrap()
    // .set_blend_mode(BlendMode::None);
    // }
    // pub fn blend_mode_add(&mut self) {
    // Rc::get_mut(&mut self.texture).unwrap()
    // .set_blend_mode(BlendMode::Add);
    // }
    // pub fn blend_mode_mod(&mut self) {
    // Rc::get_mut(&mut self.texture).unwrap()
    // .set_blend_mode(BlendMode::Mod);
    // }
    // pub fn blend_mode_blend(&mut self) {
    // Rc::get_mut(&mut self.texture).unwrap()
    // .set_blend_mode(BlendMode::Blend);
    // }
    //
}

// impl Displayable for Sprite {
// fn on_key_down(&mut self, event: &Event) {
// match event {
// &Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
// self.reset();
// }
// _ => {}
// }
// }
//
// fn update(&mut self) {
// TODO
// self.cursor += 1;
// }
//
// fn paint(&self, renderer: &mut Renderer) {
// let rect = Rect::new(self.x, self.y, self.w, self.h);
// renderer
// .copy_ex(&self.texture,
// None, Some(rect), 0.0, None, false, false)
// .expect("Single star particle should have rendered.");
// }
// }
//

impl Deref for Sprite {
    type Target = Node;

    fn deref<'a>(&'a self) -> &'a Node {
        &self.node
    }
}

impl DerefMut for Sprite {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Node {
        &mut self.node
    }
}


#[derive(Debug)]
pub struct SpriteAnimation {
    animation: Animation,
}
