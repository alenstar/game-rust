extern crate sdl2;

use std::ops::{Deref, DerefMut};
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use std::path::Path;

use sdl2::rect::Rect;
use sdl2::render::Renderer;
use sdl2::render::Texture;
use sdl2::image::LoadTexture;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use display::Displayable;
use node::Node;

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
    //
    // Generic.
    // children: Vec<Rc<RefCell<Displayable>>>,
    children: HashMap<String, Rc<RefCell<Displayable>>>, // background: Node,
}

// TODO: refactor this code since it's all copy pasta...but scrolling now works!
impl Scene {
    pub fn new(renderer: &Renderer) -> Scene {
        // let flappy = Rc::new(RefCell::new(Bird::new(renderer)));
        // let pipes = Rc::new(RefCell::new(Pipes::new(renderer)));

        // let mut children: Vec<Rc<RefCell<Displayable>>> = Vec::new();
        // children.push(flappy.clone());
        // children.push(pipes.clone());

        Scene {
            // flappy: flappy.clone(),
            // pipes: pipes.clone(),
            paused: false,
            game_over: false,
            // children: Vec::new(),
            children: HashMap::new(), // background: Node::new(renderer, &[path]),
        }
    }

    pub fn add_child(&mut self, name: &str, child: Rc<RefCell<Displayable>>) {
        // self.children.push(child);
        self.children.insert(name.to_string(), child);
    }

    pub fn get_child(&self, name: &str) -> Result<&Rc<RefCell<Displayable>>, &str> {
        // if (idx as usize) < self.children.len() {
        //     Ok(&self.children[idx as usize])
        // } else {
        //     Err("array out of bounds")
        // }
        match self.children.get(&name.to_string()) {
            Some(dp) => Ok(dp),
            None => Err("the element not found"),
        }
    }

    // pub fn paint_child(&self, renderer: &mut Renderer) {
    //     for child in &self.children {
    //         child.borrow_mut().paint(renderer);
    //     }
    // }

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

        if self.paused {
            return;
        }

        // TODO: allow cancel propagating events based on logic in parent.
        for (_, child) in &self.children {
            child.borrow_mut().on_key_down(event);
        }
    }

    fn update(&mut self) {
        if self.paused {
            return;
        }
        // self.background.update();
        // TODO: allow cancel propagating events based on logic in parent.
        for (_, child) in &self.children {
            child.borrow_mut().update();
        }

        // Nothing to do for the background at this point sucka.
        // TODO
    }
    fn paint(&self, renderer: &mut Renderer) {
        // self.background.paint(renderer);

        for (_, child) in &self.children {
            child.borrow_mut().paint(renderer);
        }
    }
}


// impl Deref for Scene {
//     type Target = Node;

//     fn deref<'a>(&'a self) -> &'a Node {
//         &self.background
//     }
// }

// impl DerefMut for Scene {
//     fn deref_mut<'a>(&'a mut self) -> &'a mut Node {
//         &mut self.background
//     }
// }


pub struct SceneManager {
    scene: Scene,
}

impl SceneManager {
    fn replace(&mut self, scene: Scene) {}
}

impl Displayable for SceneManager {
    fn on_key_down(&mut self, event: &Event) {
        match event {
            &Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                // TODO
            }
            _ => {}
        }
        self.on_key_down(event);
    }

    fn update(&mut self) {
        self.scene.update();

        // Nothing to do for the background at this point sucka.
        // TODO
    }
    fn paint(&self, renderer: &mut Renderer) {
        self.scene.paint(renderer);
    }
}
