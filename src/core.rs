use quicksilver::{geom::Rectangle, graphics::Image, lifecycle::Window, Result};
use std::rc::Rc;

pub static SCREEN_SIZE: (f32, f32) = (288.0, 382.0);

pub trait GameObject {
    fn resource(&self) -> &'static str;

    fn update(&mut self, _: &mut Window) -> Result<()> {
        Ok(())
    }

    fn draw(&mut self, _: &mut Window, _: Option<Rc<Image>>) -> Result<()> {
        Ok(())
    }

    fn area(&self) -> Rectangle;

    fn on_collided(&mut self, _other: &GameObject) {}
}
