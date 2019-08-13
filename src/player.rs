use std::rc::Rc;
use std::ops::Deref;

use quicksilver::prelude::*;
use quicksilver::{
    graphics::Image,
    geom::{Rectangle, Vector},
    lifecycle::Window,
    Result,
};

use crate::core::GameObject;

pub struct Player {
    pub pos: Vector,
    pub size: Vector,
}

impl Player {
    pub fn new(pos: Vector, size: Vector) -> Self {
        Player { pos, size }
    }
}

impl GameObject for Player {
    fn resource(&self) -> &'static str {
        "ferris.png"
    }

    fn draw(&mut self, window: &mut Window, img: Option<Rc<Image>>) -> Result<()> {
        let rect = Rectangle::new(self.pos, self.size);
        if let Some(img) = img {
            window.draw(&rect, Img(&img.deref()));
        }
        Ok(())
    }

    fn area(&self) -> Rectangle {
        Rectangle::new(self.pos, self.size)
    }

    fn on_collided(&mut self, other: &GameObject) {
        self.pos.x = 9999.0;
        self.pos.y = 9999.0;
    }
}
