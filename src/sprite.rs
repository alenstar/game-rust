extern crate sdl2;

// use std::borrow::Borrow;
use std::convert::AsRef;
use std::ops::{Deref, DerefMut};
use std::time::{Duration, SystemTime};
use std::path::Path;
use std::vec::Vec;
use std::rc::Rc;
use std::cell::RefCell;

use sdl2::rect::Rect;
use sdl2::render::{Texture, Renderer, BlendMode};
use sdl2::image::LoadTexture;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use node::Node;
use display::{Displayable, Float, FloatToDuration};
use atlas::TexElement;
use animation::Animation;

// #[derive(Debug)]
pub struct Sprite {
    x: i32,
    y: i32,
    interval: f32,
    lasttime: SystemTime,
    // visible: bool,
    running: bool,
    tex: Rc<RefCell<TexElement>>,
}

impl Sprite {
    pub fn new_from_tex(tex: Rc<RefCell<TexElement>>) -> Sprite {
        Sprite {
            x: 0,
            y: 0,
            interval: 0.0,
            lasttime: SystemTime::now(),
            // visible: true,
            running: false,
            tex: tex,
        }
    }
    pub fn new(renderer: &Renderer, path: &str) -> Sprite {
        Sprite {
            x: 0,
            y: 0,
            interval: 0.0,
            lasttime: SystemTime::now(),
            // visible: true,
            running: false,
            tex: Rc::new(RefCell::new(TexElement::new(renderer, path))),
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

    pub fn set_interval(&mut self, secs: f32) {
        self.interval = secs;
    }

    pub fn get_interval(&self) -> f32 {
        self.interval
    }

    // pub fn hide<'a>(&'a mut self) -> &'a mut Sprite {
    //     self.visible = false;
    //     self
    // }

    // pub fn show<'a>(&'a mut self) -> &'a mut Sprite {
    //     self.visible = true;
    //     self
    // }

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
}

impl Displayable for Sprite {
    fn update(&mut self) {}

    fn paint(&self, renderer: &mut Renderer) {
        if self.get_visible() {
            let rect = Rect::new(self.x, self.y, self.width(), self.height());
            self.paint_ex(renderer, rect);
            // renderer.copy_ex(&self.texture, None, Some(rect), 0.0, None, false, false)
            //         .expect("Single star particle should have rendered.");
        }
    }
    fn on_key_down(&mut self, event: &Event) {
        match event {
            &Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                // self.reset();
                // TODO
            }
            _ => {}
        }
    }
}

impl Deref for Sprite {
    type Target = TexElement;

    fn deref<'a>(&'a self) -> &'a TexElement {
        &(*self.tex)
    }
}

impl DerefMut for Sprite {
    fn deref_mut<'a>(&'a mut self) -> &'a mut TexElement {
        &mut (*self.tex)
    }
}


// 自动移动屏幕
pub struct AutoPan {
    max_w: u32,
    max_h: u32,
    sprite: Sprite,
}

impl AutoPan {
    pub fn new(sp: Sprite, w: u32, h: u32) -> AutoPan {
        AutoPan {
            max_w: w,
            max_h: h,
            sprite: sp,
        }
    }
}

impl Displayable for AutoPan {
    fn update(&mut self) {}

    fn paint(&self, renderer: &mut Renderer) {
        if self.get_visible() {
            let sz = self.sprite.get_visible_size();
            let rect = Rect::new(self.x, self.y, sz.0, sz.1);
            // self.paint_ex(renderer, rect);
            let texture = self.sprite.get_texture();
            renderer.copy_ex(texture, None, Some(rect), 0.0, None, false, false)
                    .expect("Single star particle should have rendered.");
        }
    }
    fn on_key_down(&mut self, event: &Event) {
        match event {
            &Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                // self.reset();
                // TODO
            }
            _ => {}
        }
    }
}

impl Deref for AutoPan {
    type Target = Sprite;

    fn deref<'a>(&'a self) -> &'a Sprite {
        &self.sprite
    }
}

impl DerefMut for AutoPan {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Sprite {
        &mut self.sprite
    }
}
