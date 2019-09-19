use quicksilver::{geom::Rectangle, graphics::Image, lifecycle::Window, Result};
use std::rc::Rc;

pub trait GameObject {
    fn resource(&self) -> &'static str;
    fn update(&mut self, _: &mut Window) -> Result<()> {
        Ok(())
    }
    fn draw(&mut self, _: &mut Window, _: Option<Rc<Image>>) -> Result<()> {
        Ok(())
    }
    fn area(&self) -> Rectangle;
    fn on_collided(&mut self, other: &GameObject) {}
}
