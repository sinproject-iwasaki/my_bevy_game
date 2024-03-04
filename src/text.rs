use crate::text_resources::TextResources;
use bevy::prelude::*;

pub fn spawn_initial_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_resources = TextResources::new(&asset_server);
    let initial_text = "テトリスみたいなものを作ってみてる\n「A」押したら文字が出る\n";
    text_resources.spawn_text_entity(&mut commands, initial_text);
}
