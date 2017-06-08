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

    pub fn set_interval(&mut self, secs: f32) {
        self.interval = secs;
    }

    pub fn reset(&mut self, start_x: i32, start_y: i32) {
        self.x = start_x;
        self.y = start_y;
    }

    pub fn reseize(&mut self, w: u32, h: u32) {
        self.w = w;
        self.h = h;
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
