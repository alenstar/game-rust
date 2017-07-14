extern crate sdl2;

pub mod atlas;
pub mod node;
pub mod display;
use std::collections::HashMap;


// Conditionally compile the module `test` only when the test-suite is run.
#[cfg(test)]
mod test {
    #[test]
    fn testAtlasLoader() {
        // let hm = atlas::TexLoader("res/atlas.txt", "res/atlas.png");
        // for (k, v) in &hm {
        //     println!("{} {}", k, v);
        // }

    }
}

// usage: cargo test -- --nocapture
