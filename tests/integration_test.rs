use bevy::prelude::*;
use my_bevy_game::*;

fn init_game_app() -> App {
    let mut app = App::new();

    app.add_plugins(MinimalPlugins);
    my_app::setup(&mut app);
    // app.update();

    app
}

#[test]
fn test_app_creation() {
    let app = init_game_app();

    let block_patterns = app
        .world
        .get_resource::<block_pattern::BlockPatterns>()
        .unwrap();

    assert_eq!(block_patterns.len(), 7);
}
