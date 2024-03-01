use crate::text_resources::TextResources;
use bevy::prelude::*;

fn spawn_camera(commands: &mut Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn init_and_display_text(commands: &mut Commands, asset_server: Res<AssetServer>) {
    let text_resources = TextResources::new(&asset_server);
    let initial_text =
        "初めてテキストを日本語で表示できたよ！\n２行目もかけた\n「A」押したら文字が出るよ\n";
    text_resources.spawn_text_entity(commands, initial_text);
}

fn init_sprite(commands: &mut Commands) {
    let sprite = Sprite {
        custom_size: Some(Vec2::new(20.0, 20.0)),
        ..default()
    };

    let sprite_bundle = SpriteBundle {
        sprite,
        ..default()
    };

    commands.spawn(sprite_bundle);
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_camera(&mut commands);
    init_and_display_text(&mut commands, asset_server);
    init_sprite(&mut commands);
}
