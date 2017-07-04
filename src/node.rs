extern crate sdl2;

use std::ops::{Deref, DerefMut};
use std::time::{Duration, SystemTime};
use std::cmp::Ordering;
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
    interval: f32,
    lasttime: SystemTime,
    cursor: u32,
    visible: bool,
    textures: Vec<Rc<Texture>>,
}

impl Node {
    pub fn new(renderer: &Renderer, paths: &[&str]) -> Node {
        let mut rc_textures = Vec::new();

        for path in paths {
            let mut texture = renderer.load_texture(Path::new(path))
                                      .unwrap();

            let rc_texture = Rc::new(texture);
            rc_textures.push(rc_texture);
        }

        let tquery = rc_textures[0].query();

        Node {
            x: 0,
            y: 0,
            w: tquery.width,
            h: tquery.height,
            interval: 0.0,
            lasttime: SystemTime::now(),
            cursor: 0,
            visible: true,
            textures: rc_textures,
        }
    }

    pub fn new_from(texture: Texture) -> Node {
        let mut rc_textures: Vec<Rc<Texture>> = Vec::new();
        let rc_texture = Rc::new(texture);
        rc_textures.push(rc_texture);
        let tquery = rc_textures[0].query();
        Node {
            x: 0,
            y: 0,
            w: tquery.width,
            h: tquery.height,
            interval: 0.0,
            lasttime: SystemTime::now(),
            cursor: 0,
            visible: true,
            textures: rc_textures,
        }
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

    pub fn get_width(&self) -> u32 {
        self.w
    }

    pub fn get_height(&self) -> u32 {
        self.h
    }

    pub fn get_texture(&self, idx: u32) -> Result<&Texture, &str> {
        if (idx as usize) < self.textures.len() {
            Ok(&self.textures[idx as usize])
        } else {
            Err("array out of bounds")
        }
    }
    pub fn get_texture_size(&self, idx: u32) -> Result<(u32, u32), &str> {
        if (idx as usize) < self.textures.len() {
            let tquery = self.textures[idx as usize].query();
            Ok((tquery.width, tquery.height))
        } else {
            Err("array out of bounds")
        }
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn get_position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn set_size(&mut self, w: u32, h: u32) {
        self.w = w;
        self.h = h;
    }

    pub fn get_size(&self) -> (u32, u32) {
        (self.w, self.h)
    }

    pub fn update(&mut self) {
        if self.textures.len() > 1 && self.lasttime.elapsed().unwrap().to_f32() >= self.interval {
            self.cursor += 1;
            self.lasttime = SystemTime::now();
        }
    }

    pub fn paint(&self, renderer: &mut Renderer) {
        let rect = Rect::new(self.x, self.y, self.w, self.h);
        let idx = self.cursor as usize % self.textures.len();
        renderer.copy_ex(&self.textures[idx],
                         None,
                         Some(rect),
                         0.0,
                         None,
                         false,
                         false)
                .expect("Single star particle should have rendered.");
    }

    pub fn blend_mode_none(&mut self) {
        for child in &mut self.textures {
            // child.borrow_mut().update();
            Rc::get_mut(child)
                .unwrap()
                .set_blend_mode(BlendMode::None);
        }
    }
    pub fn blend_mode_add(&mut self) {
        for child in &mut self.textures {
            // child.borrow_mut().update();
            Rc::get_mut(child)
                .unwrap()
                .set_blend_mode(BlendMode::Add);
        }
    }
    pub fn blend_mode_mod(&mut self) {
        for child in &mut self.textures {
            // child.borrow_mut().update();
            Rc::get_mut(child)
                .unwrap()
                .set_blend_mode(BlendMode::Mod);
        }
    }
    pub fn blend_mode_blend(&mut self) {
        for child in &mut self.textures {
            // child.borrow_mut().update();
            Rc::get_mut(child)
                .unwrap()
                .set_blend_mode(BlendMode::Blend);
        }

    }
}

pub trait Float {
    fn to_f64(&self) -> f64;
    fn to_f32(&self) -> f32;
}

impl Float for Duration {
    fn to_f64(&self) -> f64 {
        self.as_secs() as f64 + self.subsec_nanos() as f64 * 0.000000001
    }
    fn to_f32(&self) -> f32 {
        self.as_secs() as f32 + self.subsec_nanos() as f32 * 0.000000001
    }
}

pub trait FloatToDuration {
    fn to_duration(self) -> Duration;
}

impl FloatToDuration for f32 {
    fn to_duration(self) -> Duration {
        Duration::from_millis((self.trunc() * 1000.0) as u64 + (self.fract() * 1000.0) as u64)
    }
}

impl FloatToDuration for f64 {
    fn to_duration(self) -> Duration {
        Duration::from_millis((self.trunc() * 1000.0) as u64 + (self.fract() * 1000.0) as u64)
    }
}
