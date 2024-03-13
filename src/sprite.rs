use bevy::prelude::*;

use crate::{block_pattern::BlockPatterns, constants, position::Position, utils, window};

// pub fn spawn_sprite_at(commands: &mut Commands, windows: Query<&Window>, position: Vec2) {
//     let sprite = Sprite {
//         custom_size: Some(unit_size()),
//         ..default()
//     };

//     let window = windows.single();
//     let origin = utils::calculate_origin(window);
//     let translation = calculate_translation(origin, position);
//     let transform: Transform = Transform::from_translation(translation);

//     let sprite_bundle = SpriteBundle {
//         sprite,
//         transform,
//         ..default()
//     };

//     commands.spawn(sprite_bundle);
// }

#[cfg_attr(coverage_nightly, coverage(off))]
fn spawn_block_element(commands: &mut Commands, position: Position, color: Color) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite { color, ..default() },
            ..default()
        })
        .insert(position);
}

#[cfg_attr(coverage_nightly, coverage(off))]
pub fn spawn_block(commands: &mut Commands, block_patterns: &mut Res<BlockPatterns>) {
    let block = block_patterns.random().unwrap();

    let initial_x = (constants::UNIT_LENGTH.0 / 2) as i32;
    let initial_y = (constants::UNIT_LENGTH.1 - 4) as i32;

    block.positions.iter().for_each(|(x, y)| {
        let position = Position::new(x + initial_x, y + initial_y).unwrap();

        spawn_block_element(commands, position, block.color);
    });
}

// pub fn spawn_sprite(commands: &mut Commands) {
//     commands
//         .spawn(SpriteBundle {
//             sprite: Sprite { ..default() },
//             ..default()
//         })
//         .insert(Position { x: 3, y: 4 });
// }

// pub fn change_color(mut query: Query<&mut Sprite>, color_resources: Res<ColorResources>) {
//     let color = color_resources.random().unwrap();

//     query.iter_mut().for_each(|mut sprite| {
//         sprite.color = color;
//     });
// }

#[cfg_attr(coverage_nightly, coverage(off))]
pub fn position_transform(
    windows: Query<&Window>,
    mut position_query: Query<(&Position, &mut Transform, &mut Sprite)>,
) {
    let window = windows.single();
    let origin = window::calculate_origin(window);

    position_query
        .iter_mut()
        .for_each(|(position, mut transform, mut sprite)| {
            transform.translation = position.translation(origin);
            sprite.custom_size = Some(utils::unit_size());
        });
}
