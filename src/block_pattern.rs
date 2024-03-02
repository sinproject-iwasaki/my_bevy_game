use bevy::prelude::*;
use rand::seq::SliceRandom;

pub struct BlockPattern {
    pub positions: Vec<(i32, i32)>,
    pub color: Color,
}

impl BlockPattern {
    fn i() -> Self {
        Self {
            positions: vec![(0, 0), (0, -1), (0, 1), (0, 2)],
            color: Color::rgb_u8(110, 237, 240),
        }
    }

    fn l() -> Self {
        Self {
            positions: vec![(0, 0), (0, -1), (0, 1), (-1, 1)],
            color: Color::rgb_u8(230, 163, 58),
        }
    }

    fn j() -> Self {
        Self {
            positions: vec![(0, 0), (0, -1), (0, 1), (1, 1)],
            color: Color::rgb_u8(0, 0, 232),
        }
    }

    fn z() -> Self {
        Self {
            positions: vec![(0, 0), (0, -1), (1, 0), (1, 1)],
            color: Color::rgb_u8(220, 48, 33),
        }
    }

    fn s() -> Self {
        Self {
            positions: vec![(0, 0), (1, 0), (0, 1), (1, -1)],
            color: Color::rgb_u8(110, 237, 72),
        }
    }

    fn o() -> Self {
        Self {
            positions: vec![(0, 0), (0, 1), (1, 0), (1, 1)],
            color: Color::rgb_u8(243, 241, 80),
        }
    }

    fn t() -> Self {
        Self {
            positions: vec![(0, 0), (-1, 0), (1, 0), (0, 1)],
            color: Color::rgb_u8(148, 29, 230),
        }
    }
}

#[derive(Resource)]
pub struct BlockPatterns(Vec<BlockPattern>);

impl BlockPatterns {
    pub fn new() -> BlockPatterns {
        let patterns = vec![
            BlockPattern::i(),
            BlockPattern::l(),
            BlockPattern::j(),
            BlockPattern::z(),
            BlockPattern::s(),
            BlockPattern::o(),
            BlockPattern::t(),
        ];

        Self(patterns)
    }

    pub fn random(&self) -> Option<&BlockPattern> {
        self.0.choose(&mut rand::thread_rng())
    }
}
