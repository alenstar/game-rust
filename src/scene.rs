extern crate sdl2;

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
use spirits::Spirits;

pub struct Scene {
    // Internal state.
    paused: bool,
    game_over: bool,

    // Objects.
    // https://www.reddit.com/r/rust/comments/4ij34q/how_to_use_rcrefcellt_properly/
    // I did this because I want to share assets to control individually within the scene.
    // And additionally control generically as Displayable objects.
    // flappy: Rc<RefCell<Bird>>,
    // pipes: Rc<RefCell<Pipes>>,

    // Generic.
    children: Vec<Rc<RefCell<Displayable>>>,
    spirits: Spirits,
}

// TODO: refactor this code since it's all copy pasta...but scrolling now works!
impl Scene {
    pub fn new(renderer: &Renderer, path: &str) -> Scene {
        //let flappy = Rc::new(RefCell::new(Bird::new(renderer)));
        //let pipes = Rc::new(RefCell::new(Pipes::new(renderer)));

        //let mut children: Vec<Rc<RefCell<Displayable>>> = Vec::new();
        //children.push(flappy.clone());
        //children.push(pipes.clone());

        Scene {
            // flappy: flappy.clone(),
            // pipes: pipes.clone(),
            paused: false,
            game_over: false,
            children: Vec::new(), // children,
            spirits: Spirits::new(renderer, path, 0, 0), 
        }
    }

    pub fn add_child(&mut self, child: Rc<RefCell<Displayable>>) {
        self.children.push(child);
    }

    pub fn restart(&mut self) {
        // Reset all assets.
        // self.flappy.borrow_mut().restart();
        // self.pipes.borrow_mut().restart();


        // Finally reset the state of the scene.
        self.game_over = false;
    }

    pub fn game_over(&self) -> bool {
        self.game_over
    }
}

impl Displayable for Scene {
    fn on_key_down(&mut self, event: &Event) {
        match event {
            &Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                self.paused = !self.paused;
            }
            _ => {}
        }

        if self.paused{
            return;
        }

        //TODO: allow cancel propagating events based on logic in parent.
        for child in &self.children {
            child.borrow_mut().on_key_down(event);
        }
    }

    fn update(&mut self) {
        if self.paused {
            return;
        }

        //TODO: allow cancel propagating events based on logic in parent.
        for child in &self.children {
            child.borrow_mut().update();
        }

        // Introduce a scope to puposefully limit the scope of the borrows.
        /* {
            let ref mut bird = *self.flappy.borrow_mut();
            self.pipes.borrow().touch(bird);

            if bird.is_dead() {
                // TODO:
                self.game_over = true;
            }
        } */

        // Nothing to do for the background at this point sucka.
        // TODO
    }

    fn paint(&self, renderer: &mut Renderer) {
        self.spirits.paint(renderer);

        for child in &self.children {
            child.borrow_mut().paint(renderer);
        }
    }
}
