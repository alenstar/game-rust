// extern crate sdl2;

// use std::ops::{Deref, DerefMut, Fn};

// use std::path::Path;
// use std::vec::Vec;
// use std::rc::Rc;
// use std::cell::RefCell;

// use sdl2::rect::Rect;
// use sdl2::render::Renderer;
// use sdl2::render::Texture;
// use sdl2::render::BlendMode;
// use sdl2::image::LoadTexture;
// use sdl2::event::Event;
// use sdl2::keyboard::Keycode;

// use std::sync::{Arc, Mutex, Once, ONCE_INIT};
// use std::time::{SystemTime, Duration};
// use std::{mem, thread};

// #[derive(Clone)]
// pub struct Scheduler {
//     // Since we will be used in many threads, we need to protect
//     // concurrent access
//     schedules: Vec<Rc<RefCell<Job<'static>>>>,
// }

// impl Scheduler {
//     // add code here
//     pub fn get_scheduler() -> Scheduler {
//         // Initialize it to a null value
//         static mut SINGLETON: *const Scheduler = 0 as *const Scheduler;
//         static ONCE: Once = ONCE_INIT;

//         unsafe {
//             ONCE.call_once(|| {
//                 // Make it
//                 let singleton = Scheduler { schedules: Vec::new() };

//                 // Put it in the heap so it can outlive this call
//                 SINGLETON = mem::transmute(Box::new(singleton));
//             });

//             // Now we give out a copy of the data that is safe to use concurrently.
//             (*SINGLETON).clone()
//         }
//     }

//     pub fn unschedule(&mut self) {}
//     pub fn schedule<F>(&mut self, cb: F, tg: i32, interval: f32) {
//         self.schedules.push(Rc::new(RefCell::new(Job::new(cb, tg, interval))));
//     }
// }
// trait FnBox {
//     fn call_box(self: Box<Self>);
// }

// impl<F: Fn()> FnBox for F {
//     fn call_box(self: Box<F>) {
//         (*self)()
//     }
// }

// type Caller<'a> = Box<FnBox + 'a>;

// struct Job<'a> {
//     running: bool,
//     once: bool,
//     callback: Caller<'a>,
//     interval: f32,
//     target: i32,
// }

// impl<'a> Job<'a> {
//     pub fn new<F>(cb: F, tg: i32, interval: f32) -> Job<'a> {
//         Job {
//             once: false,
//             running: false,
//             callback: Box::new(cb()),
//             interval: interval,
//             target: tg,
//         }
//     }
// }
