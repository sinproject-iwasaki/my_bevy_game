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

    #[test]
    fn test_vec2_from_tuple() {
        let tuple = (100, 50);
        let result = vec2_from_tuple(tuple);
        assert_eq!(result, Vec2::new(100.0, 50.0));
    }

    #[test]
    fn test_unit_size() {
        let result = unit_size();
        assert_eq!(result, vec2_from_tuple(constants::UNIT_SIZE));
    }
}
