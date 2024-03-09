use bevy::prelude::*;

use crate::utils;

#[derive(Component)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn vec2(&self) -> Vec2 {
        Vec2::new(self.x as f32, self.y as f32)
    }

    pub fn translation(&self, origin: Vec2) -> Vec3 {
        let position_vec2 = self.vec2();
        let translation_vec2 = position_vec2 * utils::unit_size() + origin;

        Vec3::new(translation_vec2.x, translation_vec2.y, 0.0)
    }
}
#[cfg(test)]
mod tests {
    use crate::utils::unit_size;

    use super::*;

    #[test]
    fn test_position_new() {
        let position = Position::new(10, 20);
        assert_eq!(position.x, 10);
        assert_eq!(position.y, 20);
    }

    #[test]
    fn test_position_vec2() {
        let position: Position = Position::new(10, 20);
        let vec2 = position.vec2();
        assert_eq!(vec2, Vec2::new(10.0, 20.0));
    }

    #[test]
    fn test_position_translation() {
        let position = Position::new(10, 20);
        let origin = Vec2::new(-100.0, 5.0);
        let translation = position.translation(origin);

        let expected_x = unit_size().x * position.x as f32 + origin.x;
        let expected_y = unit_size().y * position.y as f32 + origin.y;

        let expected_translation = Vec3::new(expected_x, expected_y, 0.0);
        assert_eq!(translation, expected_translation);
    }
}
