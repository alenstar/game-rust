extern crate sdl2;

use std::ops::{Deref, DerefMut};

use std::path::Path;
use rand::{thread_rng, Rng};

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
use atlas::Atlas;

pub struct Bird {
    speed: f32,
    xaccelerate: f32,
    yaccelerate: f32,
    sprite: Sprite,
    died: bool,
}

impl Bird {
    // add code here
    pub fn new(renderer: &Renderer) -> Bird {
        Bird {
            speed: 0.0,
            xaccelerate: 0.0,
            yaccelerate: 0.2,
            sprite: Sprite::new(renderer,
                                &["res/imgs/bird_frame_1.png",
                                  "res/imgs/bird_frame_2.png",
                                  "res/imgs/bird_frame_3.png",
                                  "res/imgs/bird_frame_4.png"]),
            died: false,
        }
    }

    pub fn jump(&mut self) {
        self.speed = -8.0;
    }

    pub fn is_died(&self) -> bool {
        self.died
    }
}

impl Displayable for Bird {
    // add code here

    fn on_key_down(&mut self, event: &Event) {
        match event {
            &Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                self.jump();
                // self.particles.reset(self.x, self.y);
            }
            _ => {}
        }
    }
    fn update(&mut self) {
        let pos = self.sprite.get_position();
        self.sprite.set_position(pos.0, pos.1 + self.speed as i32);
        self.speed += self.yaccelerate;
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


#[derive(Debug)]
enum GameStatus {
    STOPED,
    PAUSED,
    RUNNING,
}

pub struct FlappyScene {
    scroll: bool,
    scroll_step: u32,
    scroll_x1: u32,
    scroll_x2: u32,
    scroll_w1: u32,
    scroll_w2: u32,
    width: u32,
    height: u32,

    state: GameStatus,
    atlas: Atlas,
    bird: Bird,
    scene: Scene,
}

impl FlappyScene {
    // add code here
    pub fn new(renderer: &Renderer, w: u32, h: u32) -> FlappyScene {
        let mut scene = Scene::new(renderer, "res/imgs/background.png");

        {
            for i in 1..6 {
                let mut layer = Rc::new(RefCell::new(Layer::new(renderer,
                                                                &format!("res/imgs/layer_0{}_1920x1080.png",
                                                                        i)
                                                                    [..],
                                                                w,
                                                                h)));
                if i < 4 {
                    layer.borrow_mut().set_scroll(false);
                }
                scene.add_child(layer);
            }
        }
        // let mut layer = scene.get_child(0).unwrap().borrow_mut() as &mut Layer;

        // scene.add_child(bird);
        scene.set_interval(0.5);

        FlappyScene {
            scroll: false,
            scroll_step: 1,
            scroll_x1: 0,
            scroll_x2: w,
            scroll_w1: 0,
            scroll_w2: 0,
            width: w,
            height: h,
            state: GameStatus::STOPED,
            atlas: Atlas::new(renderer, "res/atlas.png", "res/atlas.txt"),
            bird: Bird::new(renderer),
            scene: scene,
        }
    }

    pub fn start(&mut self) {
        // self.atlas.hide();

        // self.atlas
        //     .select_rect(&("button_play".to_string()))
        //     .set_position(&("button_play".to_string()), 0, 0);

        // self.atlas
        //     .select_rect(&("text_ready".to_string()))
        //     .set_position(&("text_ready".to_string()), 0, 100);

        self.atlas
            .select_rect(&("tutorial".to_string()))
            .set_position(&("tutorial".to_string()), 0, 200);


        self.bird.set_interval(0.3);
        let sz = self.bird.get_size();
        self.bird.set_position(self.width as i32 / 2 - sz.0 as i32,
                               self.height as i32 / 2 - sz.1 as i32);
        self.bird.start();
        self.bird.show();
    }

    pub fn stop(&mut self) {
        // TODO
        self.bird.hide();
        self.atlas
            .select_rect(&("text_game_over".to_string()))
            .set_position(&("text_game_over".to_string()), 0, 0);
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

    fn on_key_down(&mut self, event: &Event) {
        match event {
            &Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                // self.paused = !self.paused;
            }
            _ => {}
        }

        // TODO: allow cancel propagating events based on logic in parent.
        self.scene.on_key_down(event);
    }

    fn update(&mut self) {

        if self.get_elapsed() >= self.get_interval() {
            self.cursor_incr();
            self.update_time();
        }
        self.scene.update();

        if self.scroll {
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
        self.atlas.paint(renderer);
    }
}



pub struct StartScene {
    scroll: bool,
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

impl StartScene {
    // add code here
    pub fn new(renderer: &Renderer, w: u32, h: u32) -> StartScene {
        let mut bird = Rc::new(RefCell::new(Bird::new(renderer)));
        bird.borrow_mut().set_interval(0.3);
        let sz = bird.borrow_mut().get_size();
        bird.borrow_mut().set_position(w as i32 / 2 - sz.0 as i32, h as i32 / 2 - sz.1 as i32);
        bird.borrow_mut().start();

        let mut scene = Scene::new(renderer, "res/imgs/background.png");

        scene.add_child(bird);
        scene.set_interval(0.5);

        StartScene {
            scroll: false,
            scroll_step: 1,
            scroll_x1: 0,
            scroll_x2: w,
            scroll_w1: 0,
            scroll_w2: 0,
            width: w,
            height: h,
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


impl Deref for StartScene {
    type Target = Scene;

    fn deref<'a>(&'a self) -> &'a Scene {
        &self.scene
    }
}

impl DerefMut for StartScene {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Scene {
        &mut self.scene
    }
}

impl Displayable for StartScene {
    // add code here

    fn on_key_down(&mut self, event: &Event) {
        match event {
            &Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                // self.paused = !self.paused;
            }
            _ => {}
        }

        // TODO: allow cancel propagating events based on logic in parent.
        self.scene.on_key_down(event);
    }

    fn update(&mut self) {

        if self.get_elapsed() >= self.get_interval() {
            self.cursor_incr();
            self.update_time();
        }
        self.scene.update();

        if self.scroll {
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


pub struct Pipes {
    speed: f32,
    xaccelerate: f32,
    yaccelerate: f32,
    pipes: Vec<Pipe>,
    sprite: Sprite,
}

impl Pipes {
    // add code here
    pub fn new(renderer: &Renderer) -> Pipes {
        Pipes {
            speed: 0.0,
            xaccelerate: 0.0,
            yaccelerate: 0.2,
            pipes: vec![Pipe::new()],
            sprite: Sprite::new(renderer, &["res/imgs/pipe.png"]),
        }
    }

    pub fn jump(&mut self) {
        self.speed = -8.0;
    }
}

impl Displayable for Pipes {
    // add code here

    fn on_key_down(&mut self, event: &Event) {
        match event {
            &Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                self.jump();
                // self.particles.reset(self.x, self.y);
            }
            _ => {}
        }
    }
    fn update(&mut self) {
        let pos = self.sprite.get_position();
        self.sprite.set_position(pos.0, pos.1 + self.speed as i32);
        self.speed += self.yaccelerate;
        self.sprite.update();
    }

    fn paint(&self, renderer: &mut Renderer) {
        self.sprite.paint(renderer);
    }
}

impl Deref for Pipes {
    type Target = Sprite;

    fn deref<'a>(&'a self) -> &'a Sprite {
        &self.sprite
    }
}

impl DerefMut for Pipes {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Sprite {
        &mut self.sprite
    }
}



#[derive(Clone, Copy)]
pub struct Pipe {
    pub x: i32,
    pub h: i32,
    pub w: i32,
    pub inverted: bool,
}


impl Pipe {
    pub fn new() -> Pipe {
        let mut inverted = false;

        // Add some variation.
        if thread_rng().gen_range(0, 10) > 5 {
            inverted = true;
        }

        Pipe {
            x: 800,
            h: 100 + thread_rng().gen_range(0, 300) as i32,
            w: 50,
            inverted: inverted,
        }
    }

    pub fn paint(&self, renderer: &mut Renderer, texture: &Texture) {
        let mut rect = Rect::new(self.x, 600 - self.h, self.w as u32, self.h as u32);

        let mut flip = false;
        if self.inverted {
            rect.y = 0;
            flip = true;
        }

        renderer.copy_ex(texture, None, Some(rect), 0.0, None, false, flip)
                .expect("Single pipe should have rendered.");
    }

    // TODO // collision detection
}
