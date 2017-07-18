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

use sprite::Sprite;
use display::Displayable;

#[derive(Debug, Copy, Clone)]
pub enum RollMode {
    None,
    Horizontal,
    HorizontalEx,
    Vertical,
    VerticalEx,
}
pub struct Layer {
    // visible: bool,
    scroll: RollMode,
    scroll_step: i32,
    scroll_x1: i32,
    scroll_x2: i32,
    scroll_w1: u32,
    scroll_w2: u32,
    w: u32,
    h: u32,
    children: Vec<Rc<RefCell<Displayable>>>,
    node: Sprite,
}

impl Layer {
    pub fn new(renderer: &Renderer, w: u32, h: u32, path: &str) -> Layer {
        Layer {
            scroll: RollMode::None,
            scroll_step: 1,
            scroll_x1: 0,
            scroll_x2: w as i32,
            scroll_w1: 0,
            scroll_w2: 0,
            w: w,
            h: h,
            children: Vec::new(),
            node: Sprite::new(renderer, path),
        }
    }

    pub fn set_scroll(&mut self, enable: RollMode) {
        self.scroll = enable;
    }

    pub fn get_scroll(&self) -> RollMode {
        self.scroll
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
        // if self.scroll {

        // }
        match self.scroll {
            RollMode::None => {}
            RollMode::Horizontal => {
                self.scroll_x1 -= self.scroll_step;
                self.scroll_x2 -= self.scroll_step;

                if self.scroll_x1 < -1 * (self.w as i32 - self.scroll_step) {
                    self.scroll_x1 = self.w as i32;
                }
                if self.scroll_x2 < -1 * (self.w as i32 - self.scroll_step) {
                    self.scroll_x2 = self.scroll_x1 + self.w as i32;
                }

                // for child in &self.children {
                //     child.borrow_mut().update();
                // }
            }
            RollMode::HorizontalEx => {
                let sz = self.get_size();
                self.scroll_x1 += self.scroll_step as i32;
                if self.scroll_x1 > sz.0 as i32 {
                    self.scroll_x1 = 0;
                }

                if self.scroll_x1 > (sz.0 - self.w) as i32 {
                    self.scroll_w1 = sz.0 - self.scroll_x1 as u32;
                } else {
                    self.scroll_w1 = self.w;
                }

                self.scroll_x2 += self.scroll_step;
                if (self.scroll_x2 - self.w as i32) > sz.0 as i32 {
                    self.scroll_x2 = self.w as i32;
                }

                self.scroll_w2 = self.w - self.scroll_w1;
            }
            RollMode::Vertical => {}
            RollMode::VerticalEx => {}
        }
    }

    fn paint(&self, renderer: &mut Renderer) {
        if self.get_visible() {

            match self.scroll {
                RollMode::None => {
                    // self.node.paint(renderer);
                    let mut current_texture = self.get_texture();
                    renderer.copy(&mut current_texture,
                                  None,
                                  Some(Rect::new(0, 0, self.w, self.h)))
                            .expect("layer should have rendered.");
                }
                RollMode::Horizontal => {
                    let mut current_texture = self.get_texture();
                    renderer.copy(&mut current_texture,
                                  None,
                                  Some(Rect::new(self.scroll_x1, 0, self.w, self.h)))
                            .expect("layer should have rendered.");


                    renderer.copy(&mut current_texture,
                                  None,
                                  Some(Rect::new(self.scroll_x2, 0, self.w, self.h)))
                            .expect("layer should have rendered.");
                }
                RollMode::HorizontalEx => {
                    let mut current_texture = self.get_texture();
                    renderer.copy(&mut current_texture,
                                  Some(Rect::new(self.scroll_x1 as i32,
                                                 0,
                                                 self.scroll_w1,
                                                 self.h)),
                                  Some(Rect::new(0, 0, self.scroll_w1, self.h)))
                            .expect("background should have rendered.");

                    if self.scroll_w2 > 0 {
                        renderer.copy(&mut current_texture,
                                      Some(Rect::new(0, 0, self.scroll_w2, self.h)),
                                      Some(Rect::new((self.w - self.scroll_w2) as i32,
                                                     0,
                                                     self.scroll_w2,
                                                     self.h)))
                                .expect("background should have rendered.");
                    }
                }
                RollMode::Vertical => {}
                RollMode::VerticalEx => {}
            }
            // for child in &self.children {
            //     child.borrow_mut().paint(renderer);
            // }

        }
    }
}


impl Deref for Layer {
    type Target = Sprite;

    fn deref<'a>(&'a self) -> &'a Sprite {
        &self.node
    }
}

impl DerefMut for Layer {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Sprite {
        &mut self.node
    }
}
