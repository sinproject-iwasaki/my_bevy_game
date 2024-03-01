use bevy::prelude::*;

pub fn keyboard_input_system(mut text: Query<&mut Text>, key_code: Res<ButtonInput<KeyCode>>) {
    if key_code.just_pressed(KeyCode::KeyA) {
        info!("'A' currently pressed");
        let mut text = text.single_mut();
        let text = &mut text.sections[0].value;

        text.push_str("あ");
    }
}
