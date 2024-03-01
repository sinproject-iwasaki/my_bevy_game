use bevy::prelude::*;

use crate::{constants, utils};

pub fn spawn_sprite_at(commands: &mut Commands, windows: Query<&mut Window>, position: Vec2) {
    let unit_size = utils::vec2_from_tuple(constants::UNIT_SIZE);
    let sprite = Sprite {
        custom_size: Some(unit_size),
        ..default()
    };

    let window = windows.single();
    let transform = utils::calculate_transform(window, position);

    let sprite_bundle = SpriteBundle {
        sprite,
        transform,
        ..default()
    };

    commands.spawn(sprite_bundle);
}
