use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "I am a window!".into(),
                resolution: (500., 300.).into(),
                ..default()
            }),
            ..default()
        }))
        .run();
}
