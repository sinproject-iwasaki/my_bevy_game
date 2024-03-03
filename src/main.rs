use bevy::{app::PluginGroupBuilder, prelude::*};
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
    let plugin = init_plugin();
    // let color_resources = ColorResources::generate_example();
    let block_patterns = block_pattern::BlockPatterns::new();

    App::new()
        .add_plugins(plugin)
        .insert_resource(block_patterns)
        // .insert_resource(color_resources)
        .add_systems(Startup, setup::setup)
        .add_systems(Update, input::keyboard_input_system)
        .add_systems(Update, input::check_esc_to_exit)
        .add_systems(Update, sprite::position_transform)
        // .add_systems(Update, sprite::change_color)
        .add_systems(Update, sprite::spawn_block)
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
