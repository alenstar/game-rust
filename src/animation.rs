extern crate sdl2;

use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{Texture, Renderer, BlendMode};

use std::borrow::Borrow;
use std::ops::{Deref, DerefMut};
use std::time::{Duration, SystemTime};
use std::vec::Vec;
use std::rc::Rc;
use std::cell::RefCell;

use display::{Displayable, Float, FloatToDuration};
use atlas::TexElement;
use sprite::Sprite;

// #[derive(Debug)]
pub struct Animation {
    x: i32,
    y: i32,
    visible: bool,
    running: bool,
    interval: f32,
    lasttime: SystemTime,
    cursor: u32,
    texs: Vec<Rc<TexElement>>,
}

impl Animation {
    pub fn new(renderer: &Renderer, paths: &[&str]) -> Animation {
        let mut texs = Vec::new();
        for path in paths {
            let texture = TexElement::new(renderer, path);
            texs.push(Rc::new(texture));
        }
        Animation {
            x: 0,
            y: 0,
            visible: true,
            running: true,
            interval: 0.0,
            lasttime: SystemTime::now(),
            cursor: 0,
            texs: texs,
        }
    }

    pub fn new_from_texture(texs: Vec<Rc<TexElement>>) -> Animation {
        Animation {
            x: 0,
            y: 0,
            visible: true,
            running: true,
            interval: 0.0,
            lasttime: SystemTime::now(),
            cursor: 0,
            texs: texs,
        }
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn get_position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn start(&mut self) {
        self.running = true;
    }

    pub fn stop(&mut self) {
        self.running = false;
        // self.node.cursor = 0;
    }

    pub fn get_elapsed(&self) -> f32 {
        self.lasttime.elapsed().unwrap().to_f32()
    }

    pub fn update_time(&mut self) {
        self.lasttime = SystemTime::now();
    }

    pub fn cursor_decr(&mut self) -> u32 {
        self.cursor -= 1;
        self.cursor
    }

    pub fn cursor_incr(&mut self) -> u32 {
        self.cursor += 1;
        self.cursor
    }

    pub fn set_cursor(&mut self, cur: u32) {
        self.cursor = cur;
    }

    pub fn get_cursor(&self) -> u32 {
        self.cursor
    }

    pub fn set_interval(&mut self, secs: f32) {
        self.interval = secs;
    }

    pub fn get_interval(&self) -> f32 {
        self.interval
    }

    pub fn hide<'a>(&'a mut self) -> &'a mut Animation {
        self.visible = false;
        self
    }

    pub fn show<'a>(&'a mut self) -> &'a mut Animation {
        self.visible = true;
        self
    }
}

impl Displayable for Animation {
    fn update(&mut self) {
        if self.running && self.texs.len() > 1 &&
           self.lasttime.elapsed().unwrap().to_f32() >= self.interval {
            self.cursor += 1;
            self.lasttime = SystemTime::now();
        }
    }

    fn paint(&self, renderer: &mut Renderer) {
        if self.visible {
            let idx = self.cursor as usize % self.texs.len();
            self.texs[idx].paint_ex(renderer,
                                    Rect::new(self.x, self.y, self.width(), self.height()));
        }
    }

    fn on_key_down(&mut self, event: &Event) {
        match event {
            &Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                // self.reset();
            }
            _ => {}
        }
    }
}
impl Deref for Animation {
    type Target = TexElement;

    fn deref<'a>(&'a self) -> &'a TexElement {
        &self.texs[0].borrow()
    }
}

impl DerefMut for Animation {
    fn deref_mut<'a>(&'a mut self) -> &'a mut TexElement {
        Rc::get_mut(&mut self.texs[0]).unwrap()
    }
}
