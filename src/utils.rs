use bevy::prelude::*;

use crate::constants;

pub fn vec2_from_tuple(tuple: (u32, u32)) -> Vec2 {
    Vec2::new(tuple.0 as f32, tuple.1 as f32)
}

fn unit_size() -> Vec2 {
    vec2_from_tuple(constants::UNIT_SIZE)
}

fn calculate_origin(window: &Window) -> Vec2 {
    let half_unit_size = unit_size() / 2.0;
    let half_window_size = Vec2::new(window.width(), window.height()) / 2.0;

    half_unit_size - half_window_size
}

pub fn calculate_transform(window: &Window, position: Vec2) -> Transform {
    let origin = calculate_origin(window);
    let transform_position = position * unit_size() + origin;

    Transform::from_xyz(transform_position.x, transform_position.y, 0.0)
}
