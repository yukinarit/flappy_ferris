use quicksilver::prelude::*;
use quicksilver::{
    lifecycle::{State, Window},
    Result,
};

use crate::scene::Game;

pub use crate::core::SCREEN_SIZE;

pub struct System {
    pub game: Option<Game>,
}

impl System {}

impl State for System {
    fn new() -> Result<System> {
        Ok(System {
            game: Some(Game::new()?),
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        let mut game = self.game.take().unwrap();
        let res = game.update(window);
        self.game.replace(game);
        res
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        let mut game = self.game.take().unwrap();
        let res = game.draw(window);
        self.game.replace(game);
        res
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        let mut game = self.game.take().unwrap();
        let res = game.event(event, window);
        self.game.replace(game);
        res
    }
}
