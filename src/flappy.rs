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

pub struct Bird {
    sprite: Sprite,
}

impl Bird {
    // add code here
    pub fn new(renderer: &Renderer) -> Bird {
        Bird {
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
    scene: Scene,
}

impl FlappyScene {
    // add code here
    pub fn new(renderer: &Renderer) -> FlappyScene {
        let mut bird = Rc::new(RefCell::new(Bird::new(renderer)));
        bird.borrow_mut().set_interval(0.3);
        bird.borrow_mut().start();

        let mut scene = Scene::new(renderer, "res/imgs/background.png");
        scene.add_child(bird);

        FlappyScene { scene: scene }
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

// impl Displayable for FlappyScene {
// add code here
// fn update(&mut self) {
// self.scene.update();
// }
//
// fn paint(&self, renderer: &mut Renderer) {
// self.scene.paint(renderer);
// }
// }
//
