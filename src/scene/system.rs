use quicksilver::prelude::*;
use quicksilver::{
    lifecycle::{State, Window},
    Result,
};

use crate::scene::{Config, Game};

pub struct System {
    pub game: Option<Game>,
}

impl System {
    pub fn create(cfg: Config) -> Result<Self> {
        let mut system = System::new()?;
        system.game = Some(Game::create(cfg).unwrap());
        Ok(system)
    }
}

impl State for System {
    fn new() -> Result<System> {
        Ok(System { game: None })
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
