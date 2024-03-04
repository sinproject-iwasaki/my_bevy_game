use bevy::prelude::*;

use crate::{config, utils::unit_size};

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

pub fn calculate_origin(window: &Window) -> Vec2 {
    let half_unit_size = unit_size() / 2.0;
    let half_window_size = Vec2::new(window.width(), window.height()) / 2.0;

    half_unit_size - half_window_size
}
