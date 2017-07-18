extern crate sdl2;

use std::time::{Duration, SystemTime};

use sdl2::render::Renderer;
use sdl2::event::Event;

// Displayable is any type that be updated and rendered to the screen.
pub trait Displayable {
    // In the future, could add set_alpha, set_rotation, set_visible, set_blend, etc.

    // update handles only updating the internal state of a Displayable object.
    fn update(&mut self);

    // paint handles the actual painting of the Displayable object against a Renderer.
    fn paint(&self, renderer: &mut Renderer);

    // on_key_down handles a key down event with a default implmentation of noop.
    // fn on_key_down(&mut self, _event: &Event) {}

    // on_key_up handles a key up event with a default implmentation of noop.
    // fn on_key_up(&mut self, _event: &Event) {}
    // }
    //
    // pub trait Action {
    // on_key_down handles a key down event with a default implmentation of noop.
    fn on_key_down(&mut self, _event: &Event) {}

    // on_key_up handles a key up event with a default implmentation of noop.
    fn on_key_up(&mut self, _event: &Event) {}

    fn on_mouse_down(&mut self, _event: &Event) {}
    fn on_mouse_up(&mut self, _event: &Event) {}
    fn on_mouse_move(&mut self, _event: &Event) {}

    fn on_touch_down(&mut self, _event: &Event) {}
    fn on_touch_up(&mut self, _event: &Event) {}
}

pub trait Float {
    fn to_f64(&self) -> f64;
    fn to_f32(&self) -> f32;
}

impl Float for Duration {
    fn to_f64(&self) -> f64 {
        self.as_secs() as f64 + self.subsec_nanos() as f64 * 0.000000001
    }
    fn to_f32(&self) -> f32 {
        self.as_secs() as f32 + self.subsec_nanos() as f32 * 0.000000001
    }
}

pub trait FloatToDuration {
    fn to_duration(self) -> Duration;
}

impl FloatToDuration for f32 {
    fn to_duration(self) -> Duration {
        Duration::from_millis((self.trunc() * 1000.0) as u64 + (self.fract() * 1000.0) as u64)
    }
}

impl FloatToDuration for f64 {
    fn to_duration(self) -> Duration {
        Duration::from_millis((self.trunc() * 1000.0) as u64 + (self.fract() * 1000.0) as u64)
    }
}
