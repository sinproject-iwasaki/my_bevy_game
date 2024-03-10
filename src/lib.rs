#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

pub mod block_pattern;
mod camera;
mod color_resources;
mod config;
mod constants;
mod events;
mod game_timer;
mod input;
pub mod my_app;
mod position;
mod setup;
mod sprite;
mod text;
mod text_resources;
mod utils;
mod validator;
mod window;

use my_app::MyApp;

#[cfg_attr(coverage_nightly, coverage(off))]
pub fn run() {
    MyApp::new().run();
}
