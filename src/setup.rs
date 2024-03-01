use bevy::prelude::*;

use crate::{camera, sprite, text};

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>, windows: Query<&mut Window>) {
    camera::spawn_camera(&mut commands);
    text::spawn_initial_text(&mut commands, asset_server);
    sprite::spawn_sprite_at(&mut commands, windows, Vec2::new(2.0, 5.0));
}
