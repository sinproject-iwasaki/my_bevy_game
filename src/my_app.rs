use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

use crate::block_pattern;
use crate::camera;
use crate::events;
use crate::events::NewBlockEvent;
use crate::game_timer::GameTimer;
use crate::input;
use crate::position::Position;
use crate::sprite;
use crate::text;
use crate::window;

pub struct MyApp {
    app: App,
}

impl MyApp {
    fn init_window_plugin() -> WindowPlugin {
        let window = window::init_window();

        WindowPlugin {
            primary_window: Some(window),
            ..default()
        }
    }

    fn get_plugins() -> PluginGroupBuilder {
        DefaultPlugins.set(Self::init_window_plugin())
    }

    pub fn setup(mut new_block_events: EventWriter<events::NewBlockEvent>) {
        new_block_events.send_default();
    }

    fn init(app: &mut App) {
        // let color_resources = ColorResources::generate_example();
        let block_patterns = block_pattern::BlockPatterns::new();

        app.insert_resource(block_patterns)
            .add_event::<NewBlockEvent>()
            .init_resource::<GameTimer>()
            // .init_resource::<events::EventTriggerState>()
            .add_systems(Startup, camera::spawn_camera)
            .add_systems(Startup, text::spawn_initial_text)
            .add_systems(Startup, Self::setup)
            // .insert_resource(color_resources)
            // .add_systems(Startup, setup::setup)
            // .add_systems(Update, (events::event_trigger, events::event_listener))
            .add_systems(Update, GameTimer::game_timer)
            .add_systems(Update, events::new_bock_event_listener)
            .add_systems(Update, input::keyboard_input_system)
            .add_systems(Update, input::check_esc_to_exit)
            .add_systems(Update, sprite::position_transform)
            .add_systems(Update, Position::block_fall);
        // .add_systems(Update, sprite::change_color)
        // .add_systems(Update, sprite::spawn_block);
    }

    #[cfg_attr(coverage_nightly, coverage(off))]
    pub fn new() -> Self {
        let mut app = App::new();
        let plugin = Self::get_plugins();

        app.add_plugins(plugin);
        Self::init(&mut app);

        Self { app }
    }

    pub fn new_for_debug() -> Self {
        let mut app = App::new();
        let plugins = MinimalPlugins;
        // let plugins = Self::get_plugins();
        // let plugins = DefaultPlugins.build().disable::<WinitPlugin>();

        app.add_plugins(plugins);
        Self::init(&mut app);

        Self { app }
    }

    pub fn app(&self) -> &App {
        &self.app
    }

    pub fn run(&mut self) {
        self.app.run();
    }
}

#[cfg(test)]
mod tests {
    // use bevy::winit::WinitPlugin;

    use super::*;

    #[test]
    fn test_init_window_plugin() {
        MyApp::init_window_plugin();
    }

    #[test]
    fn test_create_app() {
        let my_app = MyApp::new_for_debug();

        assert!(my_app
            .app
            .world
            .contains_resource::<block_pattern::BlockPatterns>());
    }
}
