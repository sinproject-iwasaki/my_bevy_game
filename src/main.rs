use bevy::{app::PluginGroupBuilder, prelude::*};
use color_resources::ColorResources;
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
    let plugin = init_plugin();
    let color_resources = ColorResources::generate_example();

    App::new()
        .add_plugins(plugin)
        .insert_resource(color_resources)
        .add_systems(Startup, setup::setup)
        .add_systems(Update, input::keyboard_input_system)
        .add_systems(Update, input::check_esc_to_exit)
        .add_systems(Update, sprite::position_transform)
        .add_systems(Update, sprite::change_color)
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
