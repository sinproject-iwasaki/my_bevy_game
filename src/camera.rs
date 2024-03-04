use bevy::prelude::*;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn_camera() {
        let mut app = App::new();
        app.add_systems(Startup, spawn_camera);

        app.update();

        let entity_count = app.world.query::<&Camera2d>().iter(&app.world).count();
        assert_eq!(entity_count, 1);
    }
}
