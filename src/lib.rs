extern crate sdl2;

pub mod atlas;
pub mod node;
use std::collections::HashMap;


// Conditionally compile the module `test` only when the test-suite is run.
#[cfg(test)]
mod test {

    use atlas::AtlasLoader;

    #[test]
    fn testAtlasLoader() {
        let hm = AtlasLoader("res/atlas.txt");
        for (k, v) in &hm {
            println!("{} {}", k, v);
        }

    }
}

// usage: cargo test -- --nocapture
