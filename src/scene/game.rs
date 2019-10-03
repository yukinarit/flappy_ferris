use std::ops::Deref;

use log::*;
use quicksilver::prelude::*;
use quicksilver::{
    geom::Vector,
    lifecycle::{State, Window},
    Result,
};

use crate::{
    asset::AssetLoader, background::Background, compat, core::GameObject, enemy::Pipe,
    player::Player,
};

pub struct Game {
    bg: Background,
    player: Player,
    enemies: Vec<Pipe>,
    asset_loader: AssetLoader,
    last_spawned: u64,
    frame_count: u64,
}

impl Game {
    /// Spawn an enemy.
    fn spawn(&mut self, window: &mut Window) {
        self.enemies.push(Pipe::new(
            Vector::new(window.screen_size().x, 0),
            compat::randrng(0.0, 0.5),
        ));
    }

    fn check_collision(&mut self) {
        for enemy in &mut self.enemies {
            if enemy.area().overlaps(&self.player.area()) {
                enemy.on_collided(&self.player);
                self.player.on_collided(enemy.deref());
            }
        }

        if self.bg.area().overlaps(&self.player.area()) {
            self.bg.on_collided(&self.player);
            self.player.on_collided(&self.bg);
        }
    }
}

impl State for Game {
    fn new() -> Result<Game> {
        let mut asset_loader = AssetLoader::new();
        asset_loader.load("sprite.png".into());
        asset_loader.load("ferris.png".into());

        Ok(Game {
            bg: Background::new(),
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
        self.player.pos.y += 2.0;

        // Spawn an enemy.
        self.frame_count += 1;
        if (self.frame_count - self.last_spawned) as f64 / window.average_fps() >= 3.0 {
            self.spawn(window);
            self.last_spawned = self.frame_count;
        }

        self.check_collision();

        debug!("{}", self.player.pos);

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
        if let Event::MouseButton(_, ButtonState::Released) = event {
            self.player.pos.y -= 64.0;
        }

        Ok(())
    }
}
