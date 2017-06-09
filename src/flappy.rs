extern crate sdl2;

use std::ops::{Deref, DerefMut};

use std::path::Path;

use sdl2::rect::Rect;
use sdl2::render::Renderer;
use sdl2::render::Texture;
use sdl2::image::LoadTexture;
use sdl2::event::Event;
use std::cell::RefCell;
use std::rc::Rc;
use sdl2::keyboard::Keycode;

use display::Displayable;
use sprite::Sprite;
use scene::Scene;
use layer::Layer;

pub struct Bird {
    speed: f32,
    xaccelerate: f32,
    yaccelerate: f32,
    sprite: Sprite,
}

impl Bird {
    // add code here
    pub fn new(renderer: &Renderer) -> Bird {
        Bird {
            speed: 0.0,
            xaccelerate: 0.0,
            yaccelerate: 0.0,
            sprite: Sprite::new(renderer,
                                &["res/imgs/bird_frame_1.png",
                                  "res/imgs/bird_frame_2.png",
                                  "res/imgs/bird_frame_3.png",
                                  "res/imgs/bird_frame_4.png"]),
        }
    }
}

impl Displayable for Bird {
    // add code here
    fn update(&mut self) {
        self.sprite.update();
    }

    fn paint(&self, renderer: &mut Renderer) {
        self.sprite.paint(renderer);
    }
}

impl Deref for Bird {
    type Target = Sprite;

    fn deref<'a>(&'a self) -> &'a Sprite {
        &self.sprite
    }
}

impl DerefMut for Bird {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Sprite {
        &mut self.sprite
    }
}

pub struct FlappyScene {
    scroll_step: u32,
    scroll_x1: u32,
    scroll_x2: u32,
    scroll_w1: u32,
    scroll_w2: u32,
    width: u32,
    height: u32,
    // layer: Layer,
    scene: Scene,
}

impl FlappyScene {
    // add code here
    pub fn new(renderer: &Renderer, w: u32, h: u32) -> FlappyScene {
        let mut bird = Rc::new(RefCell::new(Bird::new(renderer)));
        bird.borrow_mut().set_interval(0.3);
        let sz = bird.borrow_mut().get_size();
        bird.borrow_mut().set_position(w as i32 / 2 - sz.0 as i32, h as i32 / 2 - sz.1 as i32);
        bird.borrow_mut().start();

        let mut scene = Scene::new(renderer, "res/imgs/background.png");

        {
            for i in 3..6 {
                let mut layer = Rc::new(RefCell::new(Layer::new(renderer,
                                                                &format!("res/imgs/layer_0{}_1920 \
                                                                         x 1080.png",
                                                                        i)
                                                                    [..],
                                                                w,
                                                                h)));
                // if i == 3 {
                //     layer.borrow_mut().set_scroll(false);
                // }
                scene.add_child(layer);
            }
        }
        // let mut layer = scene.get_child(0).unwrap().borrow_mut() as &mut Layer;

        scene.add_child(bird);
        scene.set_interval(0.5);

        FlappyScene {
            scroll_step: 1,
            scroll_x1: 0,
            scroll_x2: w,
            scroll_w1: 0,
            scroll_w2: 0,
            width: w,
            height: h,
            // layer: Layer::new(renderer, "res/imgs/layer_04_1920 x 1080.png", w, h),
            scene: scene,
        }
    }

    pub fn start(&mut self) {
        // TODO
    }

    pub fn stop(&mut self) {
        // TODO
    }
}


impl Deref for FlappyScene {
    type Target = Scene;

    fn deref<'a>(&'a self) -> &'a Scene {
        &self.scene
    }
}

impl DerefMut for FlappyScene {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Scene {
        &mut self.scene
    }
}

impl Displayable for FlappyScene {
    // add code here
    fn update(&mut self) {

        if self.get_elapsed() >= self.get_interval() {
            self.cursor_incr();
            self.update_time();
        }
        self.scene.update();

        let sz = self.get_texture_size(0).unwrap();
        self.scroll_x1 += self.scroll_step;
        if self.scroll_x1 > sz.0 {
            self.scroll_x1 = 0;
        }

        if self.scroll_x1 > (sz.0 - self.width) {
            self.scroll_w1 = sz.0 - self.scroll_x1;
        } else {
            self.scroll_w1 = self.width;
        }

        self.scroll_x2 += self.scroll_step;
        if (self.scroll_x2 - self.width) > sz.0 {
            self.scroll_x2 = self.width;
        }

        self.scroll_w2 = self.width - self.scroll_w1;
    }

    fn paint(&self, renderer: &mut Renderer) {

        let mut current_texture = self.get_texture(0).unwrap();
        renderer.copy(&mut current_texture,
                      Some(Rect::new(self.scroll_x1 as i32, 0, self.scroll_w1, self.height)),
                      Some(Rect::new(0, 0, self.scroll_w1, self.height)))
                .expect("background should have rendered.");

        if self.scroll_w2 > 0 {
            renderer.copy(&mut current_texture,
                          Some(Rect::new(0, 0, self.scroll_w2, self.height)),
                          Some(Rect::new((self.width - self.scroll_w2) as i32,
                                         0,
                                         self.scroll_w2,
                                         self.height)))
                    .expect("background should have rendered.");
        }

        // self.layer.paint(renderer);

        self.scene.paint_child(renderer);
    }
}
