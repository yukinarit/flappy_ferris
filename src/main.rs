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
    fn draw(&mut self, window: &mut Window) -> Result<()>;
}

struct Background {
    pos: Vector,
    size: Vector,
    img: Asset<Image>,
}

struct Player {
    pos: Vector,
    size: Vector,
    img: Asset<Image>,
}

impl GameObject for Background {
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        self.img.execute(|img| {
            window.draw(&window_rect(&window), Img(&img));
            Ok(())
        })
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

impl State for Game {
    fn new() -> Result<Game> {
        Ok(Game {
            cfg: None,
            bg: Background {
                pos: Vector::ZERO,
                size: Vector::new(144, 256),
                img: Asset::new(
                    Image::load("sprite.png").map(|img| {
                        img.subimage(Rectangle::new(Vector::ZERO, Vector::new(144, 256)))
                    }),
                ),
            },
            player: Player {
                pos: Vector::ZERO,
                size: Vector::new(17, 12) * 3,
                img: Asset::new(Image::load("sprite.png").map(|img| {
                    img.subimage(Rectangle::new(Vector::new(264, 64), Vector::new(17, 12)))
                })),
            },
        })
    }

    fn update(&mut self, _window: &mut Window) -> Result<()> {
        // Gravity.
        self.player.pos.y += 1.0;

        Ok(())
    }

    fn draw(&mut self, mut window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;
        self.bg.draw(&mut window)?;
        self.player.draw(&mut window)?;

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
