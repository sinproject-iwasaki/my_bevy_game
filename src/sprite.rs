use bevy::prelude::*;

use crate::{block_pattern::BlockPatterns, constants, utils};

#[derive(Component)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn vec2(&self) -> Vec2 {
        Vec2::new(self.x as f32, self.y as f32)
    }
}

fn unit_size() -> Vec2 {
    utils::vec2_from_tuple(constants::UNIT_SIZE)
}

pub fn calculate_translation(origin: Vec2, position: Vec2) -> Vec3 {
    let translation_vec2 = position * utils::unit_size() + origin;

    Vec3::new(translation_vec2.x, translation_vec2.y, 0.0)
}

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

fn spawn_block_element(commands: &mut Commands, position: Position, color: Color) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite { color, ..default() },
            ..default()
        })
        .insert(position);
}

pub fn spawn_block(mut commands: Commands, block_patterns: Res<BlockPatterns>) {
    let block = block_patterns.random().unwrap();

    let initial_x = constants::UNIT_LENGTH.0 / 2;
    let initial_y = constants::UNIT_LENGTH.1 - 4;

    block.positions.iter().for_each(|(x, y)| {
        let position = Position {
            x: x + initial_x as i32,
            y: y + initial_y as i32,
        };

        spawn_block_element(&mut commands, position, block.color);
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

pub fn position_transform(
    windows: Query<&Window>,
    mut position_query: Query<(&Position, &mut Transform, &mut Sprite)>,
) {
    let window = windows.single();
    let origin = utils::calculate_origin(window);

    position_query
        .iter_mut()
        .for_each(|(position, mut transform, mut sprite)| {
            let position_vec2 = position.vec2();
            let translation = calculate_translation(origin, position_vec2);

            transform.translation = translation;
            sprite.custom_size = Some(unit_size());
        });
}
