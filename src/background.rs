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

pub struct Background {
    pos: Vector,
    size: Vector,
    screen_size: Vector,
}

impl Background {
    pub fn new(size: Vector) -> Self {
        Background {
            pos: Vector::ZERO,
            size,
            screen_size: Vector::ZERO,
        }
    }

    pub fn set_screen_size(&mut self, size: Vector) {
        self.screen_size = size
    }

    pub fn fit(&self, xy: &Vector) -> Vector {
        let mut xy = xy.clone();
        if xy.x < -self.screen_size.x {
            xy.x = self.screen_size.x;
        }

        xy
    }

    pub fn left(&self) -> Rectangle {
        Rectangle::new(self.pos, self.screen_size + Vector::new(0.5, 0.0))
    }

    pub fn right(&self) -> Rectangle {
        let right = if self.pos.x < 0.0 {
            self.pos + self.screen_size.x_comp()
        } else {
            self.pos - self.screen_size.x_comp()
        };
        Rectangle::new(self.fit(&right), self.screen_size + Vector::new(0.5, 0.0))
    }

    pub fn scroll(&mut self, dx: f32, screen_width: f32) {
        self.pos.x += dx;
        self.pos = self.fit(&self.pos);

        /*
        debug!(
            "scrolling background dx={}, width={}, left={}, right={}",
            dx,
            screen_width,
            self.pos,
            self.right().pos,
        );
        */
    }
}

impl GameObject for Background {
    fn resource(&self) -> &'static str {
        "sprite.png"
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        self.set_screen_size(window.screen_size());
        self.scroll(-0.5, window.screen_size().x);
        Ok(())
    }

    fn draw(&mut self, window: &mut Window, img: Option<Rc<Image>>) -> Result<()> {
        let left = self.left();
        let right = self.right();
        if let Some(img) = img {
            let bg = img
                .deref()
                .subimage(Rectangle::new(Vector::ZERO, Vector::new(144, 256)));
            let ground = img
                .deref()
                .subimage(Rectangle::new(Vector::new(146, 0), Vector::new(154, 56)));
            window.draw(&left, Img(&bg));
            window.draw(&right, Img(&bg));
            window.draw(&Rectangle::new(Vector::new(0, 400), Vector::new(308, 112)), Img(&ground));
        }
        Ok(())
    }

    fn area(&self) -> Rectangle {
        Rectangle::new(Vector::ZERO, Vector::ZERO)
    }
}