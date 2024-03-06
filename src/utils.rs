use bevy::prelude::*;

use crate::constants;

pub fn vec2_from_tuple(tuple: (u32, u32)) -> Vec2 {
    Vec2::new(tuple.0 as f32, tuple.1 as f32)
}

pub fn unit_size() -> Vec2 {
    vec2_from_tuple(constants::UNIT_SIZE)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::tuple((100, 50), Vec2::new(100.0, 50.0))]
    #[case::tuple((200, 100), Vec2::new(200.0, 100.0))]
    fn test_vec2_from_tuple(#[case] tuple: (u32, u32), #[case] expected: Vec2) {
        let actual = vec2_from_tuple(tuple);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_unit_size() {
        let actual = unit_size();
        assert_eq!(actual, vec2_from_tuple(constants::UNIT_SIZE));
    }
}
