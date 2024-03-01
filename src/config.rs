const UNIT_SIZE: (u32, u32) = (40, 40);
const UNIT_LENGTH: (u32, u32) = (10, 18);

const WINDOW_TITLE: &str = "I am a window!";

pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

impl WindowConfig {
    pub fn new(title: &str, width: u32, height: u32) -> WindowConfig {
        Self {
            title: title.to_string(),
            width,
            height,
        }
    }

    fn calculate_window_size() -> (u32, u32) {
        (UNIT_SIZE.0 * UNIT_LENGTH.0, UNIT_SIZE.1 * UNIT_LENGTH.1)
    }

    fn create_with_default_size(title: &str) -> WindowConfig {
        let (width, height) = Self::calculate_window_size();
        Self::new(title, width, height)
    }
}

pub fn create_window_config() -> WindowConfig {
    WindowConfig::create_with_default_size(WINDOW_TITLE)
}
