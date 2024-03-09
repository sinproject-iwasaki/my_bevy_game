use bevy::prelude::*;

use crate::block_pattern;
use crate::camera;
use crate::events;
use crate::input;
use crate::sprite;
use crate::text;
use crate::window;

fn init_window_plugin() -> WindowPlugin {
    let window = window::init_window();

    WindowPlugin {
        primary_window: Some(window),
        ..default()
    }
}

pub fn setup(app: &mut App) {
    // let color_resources = ColorResources::generate_example();
    let block_patterns = block_pattern::BlockPatterns::new();

    app.insert_resource(block_patterns)
        .add_event::<events::NewBlockEvent>()
        .init_resource::<events::EventTriggerState>()
        .add_systems(Startup, camera::spawn_camera)
        .add_systems(Startup, text::spawn_initial_text)
        // .insert_resource(color_resources)
        // .add_systems(Startup, setup::setup)
        .add_systems(Update, (events::event_trigger, events::event_listener))
        .add_systems(Update, input::keyboard_input_system)
        .add_systems(Update, input::check_esc_to_exit)
        .add_systems(Update, sprite::position_transform);
    // .add_systems(Update, sprite::change_color)
    // .add_systems(Update, sprite::spawn_block);
}

#[cfg_attr(coverage_nightly, coverage(off))]
pub fn create_app() -> App {
    let mut app = App::new();
    let plugin = DefaultPlugins.set(init_window_plugin());

    app.add_plugins(plugin);
    setup(&mut app);

    app
}

#[cfg(test)]
mod tests {
    // use bevy::winit::WinitPlugin;

    use super::*;

    #[test]
    fn test_init_window_plugin() {
        init_window_plugin();
    }

    #[test]
    fn test_create_app() {
        let mut app = App::new();
        // let plugin = MinimalPlugins.set(init_window_plugin());
        // plugin.build().disable::<WinitPlugin>()

        app.add_plugins(MinimalPlugins);
        setup(&mut app);

        assert!(app
            .world
            .contains_resource::<block_pattern::BlockPatterns>());
    }
}
