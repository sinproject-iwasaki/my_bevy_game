use crate::constants::{UNIT_LENGTH, UNIT_SIZE, WINDOW_TITLE};
use crate::validator;

pub struct Title(String);

impl Title {
    pub fn new(title: &str) -> Result<Self, String> {
        validator::validate_string_length(title, 1, 50)?;
        Ok(Self(title.to_string()))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

pub struct Width(u32);

impl Width {
    pub fn new(width: u32) -> Result<Self, String> {
        validator::validate_numeric_range(width, 1, 1000)?;
        Ok(Self(width))
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}

pub struct Height(u32);

impl Height {
    pub fn new(height: u32) -> Result<Self, String> {
        validator::validate_numeric_range(height, 1, 1000)?;
        Ok(Self(height))
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}

pub struct WindowConfig {
    pub title: Title,
    pub width: Width,
    pub height: Height,
}

impl WindowConfig {
    pub fn new(title: &str, width: u32, height: u32) -> Result<WindowConfig, String> {
        Ok(Self {
            title: Title::new(title)?,
            width: Width::new(width)?,
            height: Height::new(height)?,
        })
    }

    fn calculate_window_size() -> (u32, u32) {
        (UNIT_SIZE.0 * UNIT_LENGTH.0, UNIT_SIZE.1 * UNIT_LENGTH.1)
    }

    fn create_with_default_size(title: &str) -> Result<WindowConfig, String> {
        let (width, height) = Self::calculate_window_size();
        Self::new(title, width, height)
    }
}

pub fn create_window_config() -> Result<WindowConfig, String> {
    WindowConfig::create_with_default_size(WINDOW_TITLE)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::min_1("T")]
    #[case::max_50("12345678901234567890123456789012345678901234567890")]
    #[case::normal("Test Window")]
    #[should_panic]
    #[case::min_0("")]
    #[should_panic]
    #[case::max_51("123456789012345678901234567890123456789012345678901")]
    fn test_title_new(#[case] title: &str) {
        assert!(Title::new(title).is_ok());
    }

    #[rstest]
    #[case::min_1(1)]
    #[case::max_1000(1000)]
    #[case::normal(50)]
    #[should_panic]
    #[case::min_0(0)]
    #[should_panic]
    #[case::max_1001(1001)]
    fn test_width_new(#[case] width: u32) {
        assert!(Width::new(width).is_ok());
    }

    #[rstest]
    #[case::min_1(1)]
    #[case::max_1000(1000)]
    #[case::normal(50)]
    #[should_panic]
    #[case::min_0(0)]
    #[should_panic]
    #[case::max_1001(1001)]
    fn test_height_new(#[case] height: u32) {
        assert!(Height::new(height).is_ok());
    }

    #[rstest]
    #[case::normal("Test Window", 800, 600)]
    #[case::min_1("T", 1, 1)]
    #[case::max_1000("12345678901234567890123456789012345678901234567890", 1000, 1000)]
    fn test_window_config_creation(#[case] title: &str, #[case] width: u32, #[case] height: u32) {
        let window_config = WindowConfig::new(title, width, height).unwrap();

        assert_eq!(window_config.title.value(), title);
        assert_eq!(window_config.width.value(), width);
        assert_eq!(window_config.height.value(), height);
    }

    #[test]
    fn test_default_window_size() {
        let window_config = WindowConfig::create_with_default_size("Default Size Window").unwrap();
        let expected_size = WindowConfig::calculate_window_size();

        assert_eq!(
            (window_config.width.value(), window_config.height.value()),
            expected_size
        );
    }
}
