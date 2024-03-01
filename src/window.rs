use bevy::prelude::*;

use crate::config;

fn create_window(title: String, width: u32, height: u32) -> Window {
    Window {
        title,
        resolution: (width as f32, height as f32).into(),
        ..default()
    }
}

pub fn init_window() -> Window {
    let window_config = config::create_window_config();

    create_window(
        window_config.title,
        window_config.width,
        window_config.height,
    )
}
