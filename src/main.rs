mod asset;

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
    fn update(&mut self, _: &mut Window) -> Result<()> {
        Ok(())
    }
    fn draw(&mut self, _: &mut Window, _: Option<Rc<Image>>) -> Result<()> {
        Ok(())
    }
}

struct Background {
    pos: Vector,
    size: Vector,
    screen_size: Vector,
    pub img: &'static str,
}

impl GameObject for Background {
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
}

impl Background {
    fn new(size: Vector) -> Self {
        Background {
            pos: Vector::ZERO,
            size,
            screen_size: Vector::ZERO,
            img: "sprite.png",
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
    img: &'static str,
}

impl Player {
    fn new(pos: Vector, size: Vector) -> Self {
        Player {
            pos,
            size,
            img: "ferris.png",
        }
    }
}

impl GameObject for Player {
    fn draw(&mut self, window: &mut Window, img: Option<Rc<Image>>) -> Result<()> {
        let rect = Rectangle::new(self.pos, self.size);
        if let Some(img) = img {
            window.draw(&rect, Img(&img.deref()));
        }
        Ok(())
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
    pub cfg: Option<Config>,
    pub bg: Background,
    pub player: Player,
    pub asset_loader: AssetLoader,
}

impl Game {
    fn create(cfg: Config) -> Result<Self> {
        let mut game = Game::new()?;
        game.cfg = Some(cfg);
        Ok(game)
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
            asset_loader,
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        self.asset_loader.update();
        self.bg.update(window)?;
        // Gravity.
        self.player.pos.y += 1.0;

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        self.bg.draw(window, self.asset_loader.get(self.bg.img))?;
        self.player.draw(window, self.asset_loader.get(self.player.img))?;

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

fn init_logger() {
    //console_log::init_with_level(log::Level::Debug).expect("Couldn't setup logger.");
}

fn main() {
    init_logger();
    let screen_size = Vector::new(277, 512);
    let cfg = Config { screen_size };
    run_with("FlappyFerris", screen_size, Settings::default(), || Game::create(cfg));
}
