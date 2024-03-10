// use bevy::prelude::*;
use my_bevy_game::*;

use my_bevy_game::my_app::MyApp;

#[test]
fn test_app_creation() {
    let my_app = MyApp::new_for_debug();

    let block_patterns = my_app
        .app()
        .world
        .get_resource::<block_pattern::BlockPatterns>()
        .unwrap();

    assert_eq!(block_patterns.len(), 7);
}
