use crate::text_resources::TextResources;
use bevy::prelude::*;

pub fn spawn_initial_text(commands: &mut Commands, asset_server: Res<AssetServer>) {
    let text_resources = TextResources::new(&asset_server);
    let initial_text =
        "初めてテキストを日本語で表示できたよ！\n2行目もかけた\n「A」押したら文字が出るよ\n";
    text_resources.spawn_text_entity(commands, initial_text);
}