#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

pub mod block_pattern;
mod camera;
mod color_resources;
mod config;
mod constants;
mod events;
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

#[cfg_attr(coverage_nightly, coverage(off))]
pub fn run() {
    my_app::create_app().run();
}
