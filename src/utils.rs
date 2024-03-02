use bevy::prelude::*;

use crate::constants;

pub fn vec2_from_tuple(tuple: (u32, u32)) -> Vec2 {
    Vec2::new(tuple.0 as f32, tuple.1 as f32)
}

pub fn unit_size() -> Vec2 {
    vec2_from_tuple(constants::UNIT_SIZE)
}

pub fn calculate_origin(window: &Window) -> Vec2 {
    let half_unit_size = unit_size() / 2.0;
    let half_window_size = Vec2::new(window.width(), window.height()) / 2.0;

    half_unit_size - half_window_size
}
