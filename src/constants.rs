pub const UNIT_SIZE: (u32, u32) = (40, 40);
pub const UNIT_LENGTH: (u32, u32) = (10, 18);

pub const WINDOW_TITLE: &str = "I can be a Tetris?";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_size() {
        assert_eq!(UNIT_SIZE, (40, 40));
    }

    #[test]
    fn test_unit_length() {
        assert_eq!(UNIT_LENGTH, (10, 18));
    }

    #[test]
    fn test_window_title() {
        assert_eq!(WINDOW_TITLE, "I can be a Tetris?");
    }
}
