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
    cfg: Option<Config>,
    bg: Background,
    player: Player,
}

impl Game {
    fn create(cfg: Config) -> Result<Self> {
        let mut game = Game::new()?;
        game.cfg = Some(cfg);
        Ok(game)
    }
}

trait GameObject {
    fn update(&mut self, _: &mut Window) -> Result<()> { Ok(()) }
    fn draw(&mut self, _: &mut Window) -> Result<()> { Ok(()) }
}

struct Background {
    left: Vector,
    right: Vector,
    size: Vector,
    img: Asset<Image>,
}

impl Background {
    fn new(size: Vector, img: Asset<Image>) -> Self {
        Background {
            left: Vector::ZERO,
            right: size.x_comp() + Vector::new(0.5, 0).x_comp(),
            size,
            img,
        }
    }
}

struct Player {
    pos: Vector,
    size: Vector,
    img: Asset<Image>,
}

impl GameObject for Background {
    fn update(&mut self, _: &mut Window) -> Result<()> {
        self.left = self.scroll(&self.left, -0.5);
        self.right = self.scroll(&self.right, -0.5);
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        let size = window.screen_size();
        let left = Rectangle::new(self.left, size);
        let right = Rectangle::new(self.right, size);
        self.img.execute(|img| {
            window.draw(&left, Img(&img));
            window.draw(&right, Img(&img));
            Ok(())
        })
    }
}

impl Background {
    fn scroll(&self, xy: &Vector, dx: f32) -> Vector {
        let mut xy = xy.clone();
        xy.x += dx;

        if xy.x < -self.size.x {
            xy.x += self.size.x;
        }

        xy
    }
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
                size: Vector::new(17, 12) * 3,
                img: Asset::new(Image::load("sprite.png").map(|img| {
                    img.subimage(Rectangle::new(Vector::new(264, 64), Vector::new(17, 12)))
                })),
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
    let screen_size = Vector::new(432, 768);
    let cfg = Config { screen_size };
    run_with("FlappyFerris", screen_size, Settings::default(), || {
        Game::create(cfg)
    });
}
