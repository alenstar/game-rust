extern crate sdl2;

use sdl2::rect::Rect;
use sdl2::render::{Renderer, Texture};
use sdl2::image::LoadTexture;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::rc::Rc;
use std::path::Path;

use display::Displayable;
use node::Node;

// type AtlasRect = Rect;
#[derive(Debug)]
pub struct AtlasRect {
    x: i32,
    y: i32,
    rect: Rect,
}
impl fmt::Display for AtlasRect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}  {:?}", self.x, self.y, self.rect)
    }
}

pub fn AtlasLoader(path: &str) -> HashMap<String, AtlasRect> {
    let mut map: HashMap<String, AtlasRect> = HashMap::new();
    let file = File::open(path).unwrap();

    for line in BufReader::new(file).lines() {
        let l = line.unwrap();
        let mut items = l.split_whitespace();
        let name = items.next().unwrap();
        let w = items.next().unwrap().parse::<u32>().unwrap();
        let h = items.next().unwrap().parse::<u32>().unwrap();
        let a = AtlasRect {
            x: 0,
            y: 0,
            rect: Rect::new((items.next().unwrap().parse::<f32>().unwrap() * 1024.0 + 0.1) as i32,
                            (items.next().unwrap().parse::<f32>().unwrap() * 1024.0 + 0.1) as i32,
                            w,
                            h),
        };

        map.insert(name.to_string(), a);
    }
    map
}

pub struct Element {
    x: i32,
    y: i32,
    visible: bool,
    rect: Rect,
    texture: Rc<Texture>,
}

impl Element {
    pub fn new(texture: Rc<Texture>, rect: Rect) -> Element {
        Element {
            x: 0,
            y: 0,
            visible: true,
            rect: rect,
            texture: texture,
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
        self.rect.w = w as i32;
        self.rect.h = h as i32;
    }

    pub fn get_size(&self) -> (u32, u32) {
        (self.rect.w as u32, self.rect.h as u32)
    }
}

impl Displayable for Element {
    fn on_key_down(&mut self, event: &Event) {
        // TODO: allow cancel propagating events based on logic in parent.
    }

    fn update(&mut self) {
        // TODO:
    }

    fn paint(&self, renderer: &mut Renderer) {
        if self.visible {
            // let pos = self.get_position(&self.name);
            // let rect = self.get_rect(&self.name);
            renderer.copy(&self.texture,
                          Some(self.rect),
                          Some(Rect::new(self.x, self.y, self.rect.w as u32, self.rect.h as u32)))
                    .expect("layer should have rendered.");
        }
    }
}


pub struct Atlas {
    name: String,
    visible: bool,
    atlas: HashMap<String, AtlasRect>,
    texture: Rc<Texture>,
}

impl Atlas {
    pub fn new(renderer: &Renderer, path: &str, path2: &str) -> Atlas {
        let mut texture = renderer.load_texture(Path::new(path))
                                  .unwrap();

        Atlas {
            name: String::new(),
            visible: true,
            atlas: AtlasLoader(path2),
            texture: Rc::new(texture),
        }
    }

    pub fn set_position(&mut self, name: &String, x: i32, y: i32) -> &mut Atlas {
        {
            let mut a = self.atlas.get_mut(name).unwrap();
            a.x = x;
            a.y = y;
        }
        self
    }

    pub fn create_element(&self, name: &String) -> Result<Element, &str> {
        match self.atlas.get(name) {
            Some(rect) => Ok(Element::new(self.texture.clone(), rect.rect)),
            None => Err("the element not found"),
        }
    }

    pub fn get_position(&self, name: &String) -> (i32, i32) {
        let a = self.atlas.get(name).unwrap();
        (a.x, a.y)
    }

    pub fn get_rect(&self, name: &String) -> Rect {
        self.atlas.get(name).unwrap().rect.clone()
    }

    pub fn select_rect(&mut self, name: &String) -> &mut Atlas {
        self.name = name.clone();
        self
    }

    pub fn hide<'a>(&'a mut self) -> &'a mut Atlas {
        self.visible = false;
        self
    }

    pub fn show<'a>(&'a mut self) -> &'a mut Atlas {
        self.visible = true;
        self
    }
}


impl Displayable for Atlas {
    fn on_key_down(&mut self, event: &Event) {
        // TODO: allow cancel propagating events based on logic in parent.
    }

    fn update(&mut self) {
        // TODO:
    }

    fn paint(&self, renderer: &mut Renderer) {
        if self.visible {
            let pos = self.get_position(&self.name);
            let rect = self.get_rect(&self.name);
            renderer.copy(&self.texture,
                          Some(rect),
                          Some(Rect::new(pos.0, pos.1, rect.w as u32, rect.h as u32)))
                    .expect("layer should have rendered.");
        }
    }
}
