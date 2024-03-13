use bevy::prelude::*;

use crate::constants::UNIT_SIZE;

pub fn vec2_from_tuple<T: Into<f32> + Copy>(tuple: (T, T)) -> Vec2 {
    Vec2::new(tuple.0.into(), tuple.1.into())
}

pub fn unit_size() -> Vec2 {
    vec2_from_tuple((UNIT_SIZE.0 as u16, UNIT_SIZE.1 as u16))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::tuple((100, 50), Vec2::new(100.0, 50.0))]
    #[case::tuple((200, 100), Vec2::new(200.0, 100.0))]
    fn test_vec2_from_tuple(#[case] tuple: (u8, u8), #[case] expected: Vec2) {
        let actual = vec2_from_tuple(tuple);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_unit_size() {
        let actual = unit_size();
        assert_eq!(
            actual,
            vec2_from_tuple((UNIT_SIZE.0 as u16, UNIT_SIZE.1 as u16))
        );
    }
}
