use quicksilver::prelude::*;
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::Color,
    lifecycle::{run_with, Settings, State, Window},
    Result,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    screen_size: Vector,
}

struct Game {
    pub cfg: Option<Config>,
    pub bg: Background,
    pub player: Player,
}

impl Game {
    fn create(cfg: Config) -> Result<Self> {
        let mut game = Game::new()?;
        game.cfg = Some(cfg);
        Ok(game)
    }
}

trait GameObject {
    fn update(&mut self, _: &mut Window) -> Result<()> {
        Ok(())
    }
    fn draw(&mut self, _: &mut Window) -> Result<()> {
        Ok(())
    }
}

struct Background {
    pos: Vector,
    size: Vector,
    screen_size: Vector,
    img: Asset<Image>,
}

impl Background {
    fn new(size: Vector, img: Asset<Image>) -> Self {
        Background {
            pos: Vector::ZERO,
            size,
            screen_size: Vector::ZERO,
            img,
        }
    }
}

impl GameObject for Background {
    fn update(&mut self, window: &mut Window) -> Result<()> {
        self.set_screen_size(window.screen_size());
        self.scroll(-0.5, window.screen_size().x);
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        let left = self.left();
        let right = self.right();
        self.img.execute(|img| {
            window.draw(&left, Img(&img));
            window.draw(&right, Img(&img));
            Ok(())
        })
    }
}

impl Background {
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

        /*
        println!(
            "scrolling background dx={}, width={}, left={}, right={}",
            dx,
            screen_width,
            self.pos,
            self.right().pos,
        );
        */
    }
}

struct Player {
    pos: Vector,
    size: Vector,
    img: Asset<Image>,
}

impl GameObject for Player {
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        let rect = Rectangle::new(self.pos, self.size);
        self.img.execute(|img| {
            window.draw(&rect, Img(&img));
            Ok(())
        })
    }
}

/// Helper to get window rect.
fn window_rect(window: &Window) -> Rectangle {
    Rectangle::new(Vector::ZERO, window.screen_size())
}

fn rect(x1: f32, y1: f32, x2: f32, y2: f32) -> Rectangle {
    Rectangle::new(Vector::new(x1, y1), Vector::new(x2, y2))
}

impl State for Game {
    fn new() -> Result<Game> {
        Ok(Game {
            cfg: None,
            bg: Background::new(
                Vector::new(144, 256),
                Asset::new(
                    Image::load("sprite.png").map(|img| {
                        img.subimage(Rectangle::new(Vector::ZERO, Vector::new(144, 256)))
                    }),
                ),
            ),
            player: Player {
                pos: Vector::new(70, 20),
                size: Vector::new(60, 40),
                img: Asset::new(Image::load("ferris.png")),
            },
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        // Gravity.
        self.player.pos.y += 1.0;
        self.bg.update(window)?;

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;
        self.bg.draw(window)?;
        self.player.draw(window)?;

        Ok(())
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        match event {
            Event::Key(_, _) => {
                self.player.pos.y -= 24.0;
            }
            _ => {}
        }

        Ok(())
    }
}

fn main() {
    let screen_size = Vector::new(277, 512);
    let cfg = Config { screen_size };
    run_with("FlappyFerris", screen_size, Settings::default(), || {
        Game::create(cfg)
    });
}
