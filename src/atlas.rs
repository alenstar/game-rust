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

// #[derive(Debug)]
pub struct TexElement {
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    visible: bool,
    rect: Rect,
    texture: Rc<Texture>,
}

impl fmt::Display for TexElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "({}, {}, {}, {}) {} {:?}",
               self.x,
               self.y,
               self.w,
               self.h,
               self.visible,
               self.rect)
    }
}

impl TexElement {
    pub fn load_texture(renderer: &Renderer, path: &str) -> Result<Texture, String> {
        renderer.load_texture(Path::new(path))
    }

    pub fn new_from_texture(texture: Rc<Texture>, rect: Rect) -> TexElement {
        TexElement {
            x: 0,
            y: 0,
            w: rect.w as u32,
            h: rect.h as u32,
            visible: true,
            rect: rect,
            texture: texture,
        }
    }
    pub fn new(renderer: &Renderer, path: &str) -> TexElement {

        let mut texture = renderer.load_texture(Path::new(path))
                                  .unwrap();
        let tquery = texture.query();

        TexElement {
            x: 0,
            y: 0,
            w: tquery.width,
            h: tquery.height,
            // interval: 0.0,
            // lasttime: SystemTime::now(),
            // cursor: 0,
            visible: true,
            rect: Rect::new(0, 0, 0, 0),
            texture: Rc::new(texture),
        }
    }

    pub fn set_position<'a>(&'a mut self, x: i32, y: i32) -> &'a mut TexElement {
        {
            self.x = x;
            self.y = y;
        }
        self
    }

    pub fn get_position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn set_size<'a>(&'a mut self, w: u32, h: u32) -> &'a mut TexElement {
        {
            self.w = w;
            self.h = h;
        }
        self
    }

    pub fn get_size(&self) -> (u32, u32) {
        (self.rect.w as u32, self.rect.h as u32)
    }
    pub fn hide<'a>(&'a mut self) -> &'a mut TexElement {
        self.visible = false;
        self
    }

    pub fn show<'a>(&'a mut self) -> &'a mut TexElement {
        self.visible = true;
        self
    }
}

impl Displayable for TexElement {
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
                          Some(Rect::new(self.x, self.y, self.w, self.h)))
                    .expect("layer should have rendered.");
        }
    }
}


pub fn TexLoader(renderer: &Renderer,
                 atlpath: &str,
                 texpath: &str)
                 -> HashMap<String, Rc<RefCell<TexElement>>> {
    let mut map: HashMap<String, Rc<RefCell<TexElement>>> = HashMap::new();
    let file = File::open(atlpath).unwrap();
    let mut tex = renderer.load_texture(Path::new(texpath))
                          .unwrap();
    let texture = Rc::new(tex);

    for line in BufReader::new(file).lines() {
        let l = line.unwrap();
        let mut items = l.split_whitespace();
        let name = items.next().unwrap();
        let w = items.next().unwrap().parse::<u32>().unwrap();
        let h = items.next().unwrap().parse::<u32>().unwrap();
        let rect = Rect::new((items.next().unwrap().parse::<f32>().unwrap() * 1024.0 + 0.1) as i32,
                             (items.next().unwrap().parse::<f32>().unwrap() * 1024.0 + 0.1) as i32,
                             w,
                             h);
        let a = TexElement {
            x: 0,
            y: 0,
            w: rect.w as u32,
            h: rect.h as u32,
            visible: true,
            rect: rect,
            texture: texture.clone(),
        };

        map.insert(name.to_string(), Rc::new(RefCell::new(a)));
    }
    map
}

// pub struct Atlas {
//     name: String,
//     visible: bool,
//     atlas: HashMap<String, AtlasRect>,
//     texture: Rc<Texture>,
// }

// impl Atlas {
//     pub fn new(renderer: &Renderer, path: &str, path2: &str) -> Atlas {
//         let mut texture = renderer.load_texture(Path::new(path))
//                                   .unwrap();

//         Atlas {
//             name: String::new(),
//             visible: true,
//             atlas: AtlasLoader(path2),
//             texture: Rc::new(texture),
//         }
//     }

//     pub fn set_position(&mut self, name: &String, x: i32, y: i32) -> &mut Atlas {
//         {
//             let mut a = self.atlas.get_mut(name).unwrap();
//             a.x = x;
//             a.y = y;
//         }
//         self
//     }

//     pub fn create_element(&self, name: &str) -> Result<Element, &str> {
//         match self.atlas.get(name) {
//             Some(rect) => Ok(Element::new(self.texture.clone(), rect.rect)),
//             None => Err("the element not found"),
//         }
//     }

//     pub fn get_position(&self, name: &String) -> (i32, i32) {
//         let a = self.atlas.get(name).unwrap();
//         (a.x, a.y)
//     }

//     pub fn get_rect(&self, name: &String) -> Rect {
//         self.atlas.get(name).unwrap().rect.clone()
//     }

//     pub fn select_rect(&mut self, name: &String) -> &mut Atlas {
//         self.name = name.clone();
//         self
//     }

//     pub fn hide<'a>(&'a mut self) -> &'a mut Atlas {
//         self.visible = false;
//         self
//     }

//     pub fn show<'a>(&'a mut self) -> &'a mut Atlas {
//         self.visible = true;
//         self
//     }
// }


// impl Displayable for Atlas {
//     fn on_key_down(&mut self, event: &Event) {
//         // TODO: allow cancel propagating events based on logic in parent.
//     }

//     fn update(&mut self) {
//         // TODO:
//     }

//     fn paint(&self, renderer: &mut Renderer) {
//         if self.visible {
//             let pos = self.get_position(&self.name);
//             let rect = self.get_rect(&self.name);
//             renderer.copy(&self.texture,
//                           Some(rect),
//                           Some(Rect::new(pos.0, pos.1, rect.w as u32, rect.h as u32)))
//                     .expect("layer should have rendered.");
//         }
//     }
// }
