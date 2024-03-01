use bevy::prelude::*;

pub struct TextResources {
    font: Handle<Font>,
}

impl TextResources {
    pub fn new(asset_server: &Res<AssetServer>) -> Self {
        let font = asset_server.load("fonts/NotoSerifJP-Medium.otf");
        Self { font }
    }

    pub fn create_text_style(&self) -> TextStyle {
        TextStyle {
            font: self.font.clone(),
            font_size: 52.0,
            color: Color::WHITE,
            ..default()
        }
    }

    pub fn create_text_bundle(&self, text: &str) -> TextBundle {
        TextBundle::from_section(text, self.create_text_style()).with_style(Style {
            margin: UiRect::all(Val::Px(25.0)),
            ..default()
        })
    }

    pub fn spawn_text_entity(&self, commands: &mut Commands, text: &str) {
        let text_bundle = self.create_text_bundle(text);
        commands.spawn(text_bundle);
    }
}
