use bevy::{app::PluginGroupBuilder, prelude::*};
mod camera;
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
    let plugin = init_plugin();

    App::new()
        .add_plugins(plugin)
        .add_systems(Startup, setup::setup)
        .add_systems(Update, input::keyboard_input_system)
        .add_systems(Update, input::check_esc_to_exit)
        .run();
}

fn init_plugin() -> PluginGroupBuilder {
    let window = window::init_window();

    let window_plugin = WindowPlugin {
        primary_window: Some(window),
        ..default()
    };

    DefaultPlugins.set(window_plugin)
}
