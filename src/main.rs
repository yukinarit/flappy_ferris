mod asset;
mod compat;
mod core;
mod background;
mod player;
mod enemy;

use std::ops::Deref;

// use log::*;
use quicksilver::prelude::*;
use quicksilver::{
    geom::{Rectangle, Vector},
    lifecycle::{run_with, Settings, State, Window},
    Result,
};
use serde::Deserialize;

use crate::{asset::AssetLoader, core::GameObject, background::Background, player::Player, enemy::Pipe};

#[derive(Debug, Deserialize)]
struct Config {
    screen_size: Vector,
}

struct Game {
    cfg: Option<Config>,
    bg: Background,
    player: Player,
    enemies: Vec<Pipe>,
    asset_loader: AssetLoader,
    last_spawned: u64,
    frame_count: u64,
}

impl Game {
    fn create(cfg: Config) -> Result<Self> {
        let mut game = Game::new()?;
        game.cfg = Some(cfg);
        Ok(game)
    }

    /// Spawn an enemy.
    fn spawn(&mut self, window: &mut Window) {
        self.enemies
            .push(Pipe::new(Vector::new(window.screen_size().x, 0), compat::randrng(0.0, 0.5)));
    }

    fn check_collision(&mut self) {
        for enemy in &mut self.enemies {
            if enemy.area().overlaps(&self.player.area()) {
                enemy.on_collided(&self.player);
                self.player.on_collided(enemy.deref());
            }
        }
    }
}

impl State for Game {
    fn new() -> Result<Game> {
        let mut asset_loader = AssetLoader::new();
        asset_loader.load("sprite.png".into());
        asset_loader.load("ferris.png".into());

        Ok(Game {
            cfg: None,
            bg: Background::new(Vector::new(144, 256)),
            player: Player::new(Vector::new(40, 20), Vector::new(60, 40)),
            enemies: vec![],
            asset_loader,
            last_spawned: 0,
            frame_count: 0,
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        self.asset_loader.update();
        self.bg.update(window)?;

        for enemy in &mut self.enemies {
            enemy.update(window)?;
        }

        // Gravity.
        self.player.pos.y += 1.0;

        // Spawn an enemy.
        self.frame_count += 1;
        if (self.frame_count - self.last_spawned) as f64 / window.average_fps() >= 3.0 {
            self.spawn(window);
            self.last_spawned = self.frame_count;
        }

        self.check_collision();

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        let bg = &mut self.bg;
        let player = &mut self.player;
        let loader = &self.asset_loader;

        bg.draw(window, loader.get(bg.resource()))?;
        player.draw(window, loader.get(player.resource()))?;
        for enemy in &mut self.enemies {
            enemy.draw(window, loader.get(enemy.resource()))?;
        }

        Ok(())
    }

    fn event(&mut self, event: &Event, _: &mut Window) -> Result<()> {
        match event {
            Event::MouseButton(_, ButtonState::Released) => {
                self.player.pos.y -= 24.0;
            }
            _ => {}
        }

        Ok(())
    }
}

fn main() {
    compat::init_logger();
    let screen_size = Vector::new(277, 512);
    let cfg = Config { screen_size };
    run_with("FlappyFerris", screen_size, Settings::default(), || Game::create(cfg));
}
