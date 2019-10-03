mod asset;
mod background;
mod compat;
mod core;
mod enemy;
mod player;
mod scene;

use quicksilver::lifecycle::{run, Settings};

use crate::scene::{System, SCREEN_SIZE};

fn main() {
    stdweb_logger::init();
    log::set_max_level(log::LevelFilter::Info);

    run::<System>("FlappyFerris", SCREEN_SIZE.into(), Settings::default());
}
