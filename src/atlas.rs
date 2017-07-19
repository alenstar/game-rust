extern crate sdl2;

use sdl2::rect::{Rect, Point};
use sdl2::render::{Renderer, Texture, BlendMode};
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

pub trait Element {
    fn hide(&mut self);

    fn show(&mut self);

    fn get_size(&self) -> (u32, u32);

    fn width(&self) -> u32 {
        self.get_size().0
    }

    fn height(&self) -> u32 {
        self.get_size().1
    }

    fn set_visible(&mut self, enable: bool) {
        if enable {
            self.show()
        } else {
            self.hide()
        }
    }

    fn get_visible(&self) -> bool;
}

// #[derive(Debug)]
pub struct TexElement {
    flip_h: bool,
    flip_v: bool,
    angle: f64,
    visible_w: u32,
    visible_h: u32,
    visible: bool,
    center: Point,
    rect: Rect,
    texture: Rc<Texture>,
}

impl fmt::Display for TexElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "TexElement {} {} {} {} {:?} {:?}",
               self.flip_h,
               self.flip_v,
               self.angle,
               self.visible,
               self.center,
               self.rect)
    }
}

impl TexElement {
    pub fn load_texture(renderer: &Renderer, path: &str) -> Result<Texture, String> {
        renderer.load_texture(Path::new(path))
    }

    pub fn new_from_texture(texture: Rc<Texture>, rect: Rect) -> TexElement {
        TexElement {
            flip_v: false,
            flip_h: false,
            angle: 0.0,
            visible_w: rect.w as u32,
            visible_h: rect.h as u32,
            visible: true,
            center: rect.center(),
            rect: rect,
            texture: texture,
        }
    }

    pub fn new(renderer: &Renderer, path: &str) -> TexElement {

        let mut texture = renderer.load_texture(Path::new(path))
                                  .unwrap();
        let tquery = texture.query();
        let rect = Rect::new(0, 0, tquery.width, tquery.height);
        TexElement {
            flip_v: false,
            flip_h: false,
            angle: 0.0,
            visible_w: rect.w as u32,
            visible_h: rect.h as u32,
            visible: true,
            center: rect.center(),
            rect: rect,
            texture: Rc::new(texture),
        }
    }

    pub fn get_size(&self) -> (u32, u32) {
        (self.rect.w as u32, self.rect.h as u32)
    }

    pub fn width(&self) -> u32 {
        self.rect.w as u32
    }

    pub fn height(&self) -> u32 {
        self.rect.h as u32
    }

    pub fn hide<'a>(&'a mut self) -> &'a mut TexElement {
        self.visible = false;
        self
    }

    pub fn show<'a>(&'a mut self) -> &'a mut TexElement {
        self.visible = true;
        self
    }

    pub fn set_visible(&mut self, enable: bool) {
        self.visible = enable;
    }

    pub fn get_visible(&self) -> bool {
        self.visible
    }

    pub fn get_visible_size(&self) -> (u32, u32) {
        (self.visible_w, self.visible_h)
    }

    pub fn set_visible_size(&mut self, w: u32, h: u32) {
        self.visible_w = w;
        self.visible_h = h;
    }

    pub fn set_angle<'a>(&'a mut self, angle: f64) -> &'a mut TexElement {
        self.angle = angle;
        self
    }

    pub fn get_angle(&self, angle: f64) -> f64 {
        self.angle
    }

    pub fn set_flip<'a>(&'a mut self, horizontal: bool, vertical: bool) -> &'a mut TexElement {
        self.flip_h = horizontal;
        self.flip_v = vertical;
        self
    }

    pub fn get_flip(&self) -> (bool, bool) {
        (self.flip_h, self.flip_v)
    }

    pub fn set_center<'a>(&'a mut self, x: i32, y: i32) -> &'a mut TexElement {
        self.center = Point::new(x, y);
        self
    }

    pub fn get_center(&self) -> (i32, i32) {
        (self.center.x(), self.center.y())
    }

    pub fn blend_mode_none(&mut self) {
        Rc::get_mut(&mut self.texture)
            .unwrap()
            .set_blend_mode(BlendMode::None);
    }
    pub fn blend_mode_add(&mut self) {
        Rc::get_mut(&mut self.texture)
            .unwrap()
            .set_blend_mode(BlendMode::Add);
    }
    pub fn blend_mode_mod(&mut self) {
        Rc::get_mut(&mut self.texture)
            .unwrap()
            .set_blend_mode(BlendMode::Mod);
    }
    pub fn blend_mode_blend(&mut self) {
        Rc::get_mut(&mut self.texture)
            .unwrap()
            .set_blend_mode(BlendMode::Blend);
    }

    pub fn get_texture(&self) -> &Texture {
        &self.texture
    }

    pub fn paint_ex(&self, renderer: &mut Renderer, rect: Rect) {
        renderer.copy_ex(&self.texture,
                          Some(self.rect),
                          Some(rect), self.angle, Some(self.center), self.flip_h, self.flip_v)
                          // Some(Rect::new(self.x, self.y, self.w, self.h)))
                    .expect("layer should have rendered.");

    }
}

impl Displayable for TexElement {
    fn update(&mut self) {}
    fn paint(&self, renderer: &mut Renderer) {
        if self.visible {
            let size = self.get_size();
            self.paint_ex(renderer, Rect::new(0, 0, size.0, size.1));
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
            flip_v: false,
            flip_h: false,
            angle: 0.0,
            visible_w: rect.w as u32,
            visible_h: rect.h as u32,
            visible: true,
            center: rect.center(),
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
