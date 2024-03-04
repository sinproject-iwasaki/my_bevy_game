mod app;
mod block_pattern;
mod camera;
mod color_resources;
mod config;
mod constants;
mod input;
mod setup;
mod sprite;
mod text;
mod text_resources;
mod utils;
mod window;

fn main() {
    app::create_app().run();
}
