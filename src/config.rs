use crate::constants::WINDOW_TITLE;
use crate::validator;
use crate::window_size::WindowSize;

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

pub struct WindowConfig {
    title: Title,
    window_size: WindowSize,
}

impl WindowConfig {
    pub fn new() -> Self {
        let title = Title::new(WINDOW_TITLE).unwrap();
        let window_size = WindowSize::new_by_unit_size();

        Self { title, window_size }
    }

    pub fn title(&self) -> &str {
        self.title.value()
    }

    pub fn width(&self) -> u32 {
        self.window_size.width().value()
    }

    pub fn height(&self) -> u32 {
        self.window_size.height().value()
    }
}

#[cfg(test)]
mod tests {
    use crate::constants::{UNIT_LENGTH, UNIT_SIZE};

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

    #[test]
    fn test_window_config_new() {
        let window_config = WindowConfig::new();

        assert_eq!(window_config.title.value(), WINDOW_TITLE);
        assert_eq!(
            (window_config.width(), window_config.height()),
            (UNIT_SIZE.0 * UNIT_LENGTH.0, UNIT_SIZE.1 * UNIT_LENGTH.1)
        );
    }
}
