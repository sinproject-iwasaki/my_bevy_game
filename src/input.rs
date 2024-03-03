use bevy::{app::AppExit, prelude::*};

pub fn keyboard_input_system(
    mut text: Query<&mut Text>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyA) {
        info!("'A' currently pressed");
        let mut text = text.single_mut();
        let text = &mut text.sections[0].value;

        text.push('あ');
    }
}

pub fn check_esc_to_exit(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit);
    }
}
