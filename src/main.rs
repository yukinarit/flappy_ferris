mod asset;
mod compat;

use std::ops::Deref;
use std::rc::Rc;

use log::*;
use quicksilver::prelude::*;
use quicksilver::{
    geom::{Rectangle, Vector},
    lifecycle::{run_with, Settings, State, Window},
    Result,
};
use serde::Deserialize;

use asset::AssetLoader;

#[derive(Debug, Deserialize)]
struct Config {
    screen_size: Vector,
}

trait GameObject {
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

struct Background {
    pos: Vector,
    size: Vector,
    screen_size: Vector,
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
            window.draw(&left, Img(&bg));
            window.draw(&right, Img(&bg));
        }
        Ok(())
    }
    fn area(&self) -> Rectangle {
        Rectangle::new(Vector::ZERO, Vector::ZERO)
    }
}

impl Background {
    fn new(size: Vector) -> Self {
        Background {
            pos: Vector::ZERO,
            size,
            screen_size: Vector::ZERO,
        }
    }

    fn set_screen_size(&mut self, size: Vector) {
        self.screen_size = size
    }

    fn fit(&self, xy: &Vector) -> Vector {
        let mut xy = xy.clone();
        if xy.x < -self.screen_size.x {
            xy.x = self.screen_size.x;
        }

        xy
    }

    fn left(&self) -> Rectangle {
        Rectangle::new(self.pos, self.screen_size + Vector::new(0.5, 0.0))
    }

    fn right(&self) -> Rectangle {
        let right = if self.pos.x < 0.0 {
            self.pos + self.screen_size.x_comp()
        } else {
            self.pos - self.screen_size.x_comp()
        };
        Rectangle::new(self.fit(&right), self.screen_size + Vector::new(0.5, 0.0))
    }

    fn scroll(&mut self, dx: f32, screen_width: f32) {
        self.pos.x += dx;
        self.pos = self.fit(&self.pos);

        debug!(
            "scrolling background dx={}, width={}, left={}, right={}",
            dx,
            screen_width,
            self.pos,
            self.right().pos,
        );
    }
}

struct Player {
    pos: Vector,
    size: Vector,
}

impl Player {
    fn new(pos: Vector, size: Vector) -> Self {
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

struct Pipe {
    pos: Vector,
    size: Vector,
    img: &'static str,
}

impl Pipe {
    fn new(pos: Vector, size: Vector) -> Self {
        Pipe {
            pos,
            size,
            img: "sprite.png",
        }
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

/// Helper to get window rect.
fn window_rect(window: &Window) -> Rectangle {
    Rectangle::new(Vector::ZERO, window.screen_size())
}

fn rect(x1: f32, y1: f32, x2: f32, y2: f32) -> Rectangle {
    Rectangle::new(Vector::new(x1, y1), Vector::new(x2, y2))
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
            .push(Pipe::new(Vector::new(window.screen_size().x, 0), Vector::new(26, 135)));
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
