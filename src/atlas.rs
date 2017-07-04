extern crate sdl2;

use sdl2::render::Renderer;

use std::collections::HashMap;
use std::fmt;
use std::io::{BufReader, BufRead};
use std::fs::File;
use node::Node;

#[derive(Debug)]
pub struct AtlasRect {
    width: u32,
    height: u32,
    x: i32,
    y: i32,
}

impl fmt::Display for AtlasRect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "({}, {}) ({}, {})",
               self.width,
               self.height,
               self.x,
               self.y)
    }
}

pub fn AtlasLoader(path: &str) -> HashMap<String, AtlasRect> {
    let mut map: HashMap<String, AtlasRect> = HashMap::new();
    let file = File::open(path).unwrap();

    for line in BufReader::new(file).lines() {
        let l = line.unwrap();
        let mut items = l.split_whitespace();
        let name = items.next().unwrap();
        let a = AtlasRect {
            width: items.next().unwrap().parse::<u32>().unwrap(),
            height: items.next().unwrap().parse::<u32>().unwrap(),
            x: (items.next().unwrap().parse::<f32>().unwrap() * 1024.0 + 0.1) as i32,
            y: (items.next().unwrap().parse::<f32>().unwrap() * 1024.0 + 0.1) as i32,
        };
        map.insert(name.to_string(), a);
    }
    map
}

pub struct Atlas {
    name: String,
    atlas: HashMap<String, AtlasRect>,
    node: Node,
}

impl Atlas {
    pub fn new(renderer: &Renderer, path: &str, path2: &str) -> Atlas {
        Atlas {
            name: "".to_string(),
            atlas: AtlasLoader(path2),
            node: Node::new(renderer, &[path]),
        }
    }

    // pub fn set_atlas(&mut self, name: String) -> Atlas {
    //     self.name = name;
    //     self.as_ref()
    // }
}
