use bevy::prelude::*;

use crate::{constants::UNIT_LENGTH, game_timer::GameTimer, utils, validator};

struct PosX(i32);

impl PosX {
    fn new(x: i32) -> Result<Self, String> {
        validator::validate_numeric_range(x, 0, (UNIT_LENGTH.0 - 1) as i32)?;

        Ok(Self(x))
    }

    pub fn value(&self) -> i32 {
        self.0
    }
}

struct PosY(i32);

impl PosY {
    fn new(y: i32) -> Result<Self, String> {
        validator::validate_numeric_range(y, 0, (UNIT_LENGTH.1 - 1) as i32)?;

        Ok(Self(y))
    }

    pub fn value(&self) -> i32 {
        self.0
    }
}

#[derive(Component)]
pub struct Position(PosX, PosY);

impl Position {
    pub fn new(x: i32, y: i32) -> Result<Self, String> {
        let pos_x = PosX::new(x)?;
        let pos_y = PosY::new(y)?;

        Ok(Self(pos_x, pos_y))
    }

    pub fn vec2(&self) -> Vec2 {
        Vec2::new(self.x() as f32, self.y() as f32)
    }

    pub fn translation(&self, origin: Vec2) -> Vec3 {
        let position_vec2 = self.vec2();
        let translation_vec2 = position_vec2 * utils::unit_size() + origin;

        Vec3::new(translation_vec2.x, translation_vec2.y, 0.0)
    }

    pub fn block_fall(game_timer: ResMut<GameTimer>, mut position_query: Query<&mut Position>) {
        if !game_timer.timer().finished() {
            return;
        }

        for mut position in position_query.iter_mut() {
            *position = Position::new(position.x(), position.y() - 1).unwrap();
        }
    }

    pub fn x(&self) -> i32 {
        self.0.value()
    }

    pub fn y(&self) -> i32 {
        self.1.value()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::unit_size;

    use super::*;

    #[test]
    fn test_position_new() {
        let position = Position::new(5, 1).unwrap();
        assert_eq!(position.x(), 5);
        assert_eq!(position.y(), 1);
    }

    #[test]
    fn test_position_vec2() {
        let position: Position = Position::new(4, 9).unwrap();
        let vec2 = position.vec2();
        assert_eq!(vec2, Vec2::new(4.0, 9.0));
    }

    #[test]
    fn test_position_translation() {
        let position = Position::new(3, 8).unwrap();
        let origin = Vec2::new(-100.0, 5.0);
        let translation = position.translation(origin);

        let expected_x = unit_size().x * position.x() as f32 + origin.x;
        let expected_y = unit_size().y * position.y() as f32 + origin.y;

        let expected_translation = Vec3::new(expected_x, expected_y, 0.0);
        assert_eq!(translation, expected_translation);
    }
}
