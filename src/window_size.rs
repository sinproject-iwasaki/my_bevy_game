use crate::{
    constants::{UNIT_LENGTH, UNIT_SIZE},
    validator,
};

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

pub struct WindowSize(Width, Height);

impl WindowSize {
    pub fn new_by_unit_size() -> Self {
        let width = Width::new(UNIT_SIZE.0 * UNIT_LENGTH.0).unwrap();
        let height = Height::new(UNIT_SIZE.1 * UNIT_LENGTH.1).unwrap();

        Self(width, height)
    }

    pub fn width(&self) -> &Width {
        &self.0
    }

    pub fn height(&self) -> &Height {
        &self.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

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

    #[test]
    fn test_new_by_unit_size() {
        let window_size = WindowSize::new_by_unit_size();

        assert_eq!(window_size.width().value(), UNIT_SIZE.0 * UNIT_LENGTH.0);
        assert_eq!(window_size.height().value(), UNIT_SIZE.1 * UNIT_LENGTH.1);
    }
}
