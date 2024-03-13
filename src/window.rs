use bevy::prelude::*;

use crate::{config::WindowConfig, utils::unit_size};

fn create_window(title: &str, width: u32, height: u32) -> Window {
    Window {
        title: title.to_string(),
        resolution: (width as f32, height as f32).into(),
        ..default()
    }
}

pub fn init_window() -> Window {
    let window_config = WindowConfig::new();

    create_window(
        window_config.title(),
        window_config.width(),
        window_config.height(),
    )
}

pub fn calculate_origin(window: &Window) -> Vec2 {
    let half_unit_size = unit_size() / 2.0;
    let half_window_size = Vec2::new(window.width(), window.height()) / 2.0;

    half_unit_size - half_window_size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_window() {
        let title = "Test Window";
        let width = 800;
        let height = 600;
        let window = create_window(title, width, height);

        assert_eq!(window.title, title);
        assert_eq!(window.width(), width as f32);
        assert_eq!(window.height(), height as f32);
    }

    #[test]
    fn test_init_window() {
        let window = init_window();

        assert!(!window.title.is_empty());
        assert!(window.width() > 0.0);
        assert!(window.height() > 0.0);
    }

    #[test]
    fn test_calculate_origin() {
        let window = Window {
            title: "Test".to_string(),
            resolution: (800.0, 600.0).into(),
            ..default()
        };
        let origin = calculate_origin(&window);

        assert_eq!(origin.x, unit_size().x / 2.0 - 400.0);
        assert_eq!(origin.y, unit_size().y / 2.0 - 300.0);
    }
}
