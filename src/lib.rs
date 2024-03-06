pub mod block_pattern;
mod camera;
mod color_resources;
mod config;
mod constants;
mod input;
pub mod my_app;
mod setup;
mod sprite;
mod text;
mod text_resources;
mod utils;
mod validator;
mod window;

pub fn run() {
    my_app::create_app().run();
}
