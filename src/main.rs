mod asset;
mod background;
mod compat;
mod core;
mod enemy;
mod player;
mod scene;

use quicksilver::{
    geom::Vector,
    lifecycle::{run_with, Settings},
};

use crate::scene::{Config, Game};

fn main() {
    stdweb_logger::init();
    log::set_max_level(log::LevelFilter::Info);
    let screen_size = Vector::new(277, 512);
    let cfg = Config { screen_size };
    run_with("FlappyFerris", screen_size, Settings::default(), || {
        Game::create(cfg)
    });
}
