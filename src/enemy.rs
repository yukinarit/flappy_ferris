use std::ops::Deref;
use std::rc::Rc;

use quicksilver::prelude::*;
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::Image,
    lifecycle::Window,
    Result,
};

use crate::core::GameObject;

pub struct Pipe {
    pos: Vector,
    size: Vector,
}

impl Pipe {
    pub fn new(mut pos: Vector, len: f32) -> Self {
        let size = Vector::new(26, 135) * 1.5;
        pos.y -= size.y * len;
        Pipe { pos, size }
    }
}

impl GameObject for Pipe {
    fn resource(&self) -> &'static str {
        "sprite.png"
    }

    fn update(&mut self, _: &mut Window) -> Result<()> {
        self.pos.x -= 0.5;
        Ok(())
    }

    fn draw(&mut self, window: &mut Window, img: Option<Rc<Image>>) -> Result<()> {
        let rect = Rectangle::new(self.pos, self.size);
        if let Some(img) = img {
            let pipe = img
                .deref()
                .subimage(Rectangle::new(Vector::new(302, 0), Vector::new(26, 135)));
            window.draw(&rect, Img(&pipe));
        }
        Ok(())
    }

    fn area(&self) -> Rectangle {
        Rectangle::new(self.pos, self.size)
    }
}
