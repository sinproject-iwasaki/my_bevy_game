use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            // FrameTimeDiagnosticsPlugin,
            // LogDiagnosticsPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, keyboard_input_system)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/NotoSerifJP-Medium.otf");

    commands.spawn(Camera2dBundle::default());

    commands.spawn(
        TextBundle::from_section(
            "初めてテキストを日本語で表示できたよ！\n２行目もかけた\n「A」押したら文字が出るよ\n",
            TextStyle {
                font: font.clone(),
                font_size: 52.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            margin: UiRect::all(Val::Px(50.0)),
            // position_type: PositionType::Absolute,
            // top: Val::Px(20.0),
            // left: Val::Px(20.0),
            ..default()
        }),
    );
}

fn keyboard_input_system(mut text: Query<&mut Text>, key_code: Res<ButtonInput<KeyCode>>) {
    if key_code.just_pressed(KeyCode::KeyA) {
        info!("'A' currently pressed");
        let mut text = text.single_mut();
        let text = &mut text.sections[0].value;

        text.push_str("あ");
    }
}
