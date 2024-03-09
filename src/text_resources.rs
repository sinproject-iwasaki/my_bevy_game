use bevy::prelude::*;

pub struct TextResources {
    font: Handle<Font>,
}

impl TextResources {
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub fn new(asset_server: &Res<AssetServer>) -> Self {
        let font = asset_server.load("fonts/NotoSerifJP-Medium.otf");
        Self { font }
    }

    #[cfg_attr(coverage_nightly, coverage(off))]
    pub fn create_text_style(&self) -> TextStyle {
        TextStyle {
            font: self.font.clone(),
            font_size: 36.0,
            color: Color::WHITE,
        }
    }

    #[cfg_attr(coverage_nightly, coverage(off))]
    pub fn create_text_bundle(&self, text: &str) -> TextBundle {
        TextBundle::from_section(text, self.create_text_style()).with_style(Style {
            align_self: AlignSelf::Center,
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            bottom: Val::Px(50.0),
            margin: UiRect::all(Val::Px(15.0)),
            ..default()
        })
    }

    #[cfg_attr(coverage_nightly, coverage(off))]
    pub fn spawn_text_entity(&self, commands: &mut Commands, text: &str) {
        let text_bundle = self.create_text_bundle(text);
        commands.spawn(text_bundle);
    }
}
