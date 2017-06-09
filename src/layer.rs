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

use node::Node;
use display::Displayable;

pub struct Layer {
    visible: bool,
    scroll_step: i32,
    scroll_x1: i32,
    scroll_x2: i32,
    width: u32,
    height: u32,
    children: Vec<Rc<RefCell<Displayable>>>,
    node: Node,
}

impl Layer {
    pub fn new(renderer: &Renderer, path: &str, w: u32, h: u32) -> Layer {
        Layer {
            visible: true,
            scroll_step: 0,
            scroll_x1: 0,
            scroll_x2: w as i32,
            width: w,
            height: h,
            children: Vec::new(),
            node: Node::new(renderer, &[path]),
        }
    }
}

impl Displayable for Layer {
    fn on_key_down(&mut self, event: &Event) {
        // TODO: allow cancel propagating events based on logic in parent.
        for child in &self.children {
            child.borrow_mut().on_key_down(event);
        }
    }

    fn update(&mut self) {
        self.scroll_x1 -= self.scroll_step;
        self.scroll_x2 -= self.scroll_step;

        if self.scroll_x1 < -1 * self.width as i32 {
            self.scroll_x1 = self.width as i32;
        }
        if self.scroll_x2 < -1 * self.width as i32 {
            self.scroll_x2 = self.scroll_x1 + self.width as i32 - self.scroll_step;
        }

        for child in &self.children {
            child.borrow_mut().update();
        }
        // Nothing to do for the background at this point sucka.
        // TODO
    }

    fn paint(&self, renderer: &mut Renderer) {
        if self.visible {
            let mut current_texture = self.get_texture(0).unwrap();
            renderer.copy(&mut current_texture,
                          None,
                          Some(Rect::new(self.scroll_x1, 0, self.width, self.height)))
                    .expect("layer should have rendered.");


            renderer.copy(&mut current_texture,
                          None,
                          Some(Rect::new(self.scroll_x2, 0, self.width, self.height)))
                    .expect("layer should have rendered.");

            for child in &self.children {
                child.borrow_mut().paint(renderer);
            }
        }
    }
}


impl Deref for Layer {
    type Target = Node;

    fn deref<'a>(&'a self) -> &'a Node {
        &self.node
    }
}

impl DerefMut for Layer {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Node {
        &mut self.node
    }
}
