
#[derive(Debug)]
pub struct Animation {
    x: i32,
    y: i32,
    visible: bool,
    // /
    // cursor: u32,
    // nodes: Vec<Rc<Node>>,
    running: bool,
    tex: TexElement,
}

impl Displayable for Sprite {
    fn on_key_down(&mut self, event: &Event) {
        match event {
            &Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                self.reset();
            }
            _ => {}
        }
    }

    fn update(&mut self) {
        TODO
self.cursor += 1;
    }

    fn paint(&self, renderer: &mut Renderer) {
        let rect = Rect::new(self.x, self.y, self.w, self.h);
        renderer.copy_ex(&self.texture, None, Some(rect), 0.0, None, false, false)
                .expect("Single star particle should have rendered.");
    }
}
