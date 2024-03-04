use crate::constants::{UNIT_LENGTH, UNIT_SIZE, WINDOW_TITLE};

pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

impl WindowConfig {
    pub fn new(title: String, width: u32, height: u32) -> WindowConfig {
        Self {
            title,
            width,
            height,
        }
    }

    fn calculate_window_size() -> (u32, u32) {
        (UNIT_SIZE.0 * UNIT_LENGTH.0, UNIT_SIZE.1 * UNIT_LENGTH.1)
    }

    fn create_with_default_size(title: &str) -> WindowConfig {
        let (width, height) = Self::calculate_window_size();
        Self::new(title.to_string(), width, height)
    }
}

pub fn create_window_config() -> WindowConfig {
    WindowConfig::create_with_default_size(WINDOW_TITLE)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_config_creation() {
        let title = "Test Window".to_string();
        let width = 800;
        let height = 600;
        let window_config = WindowConfig::new(title.clone(), width, height);

        assert_eq!(window_config.title, title);
        assert_eq!(window_config.width, width);
        assert_eq!(window_config.height, height);
    }

    #[test]
    fn test_default_window_size() {
        let window_config = WindowConfig::create_with_default_size("Default Size Window");
        let expected_size = WindowConfig::calculate_window_size();

        assert_eq!((window_config.width, window_config.height), expected_size);
    }
}
