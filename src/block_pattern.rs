use bevy::prelude::*;
use rand::seq::SliceRandom;

pub struct BlockPattern {
    pub color: Color,
    pub positions: Vec<(i32, i32)>,
}

impl BlockPattern {
    fn i() -> Self {
        Self {
            color: Color::rgb_u8(110, 237, 240),
            positions: vec![(0, 0), (0, -1), (0, 1), (0, 2)],
        }
    }

    fn l() -> Self {
        Self {
            color: Color::rgb_u8(230, 163, 58),
            positions: vec![(0, 0), (0, -1), (0, 1), (-1, 1)],
        }
    }

    fn j() -> Self {
        Self {
            color: Color::rgb_u8(0, 0, 232),
            positions: vec![(0, 0), (0, -1), (0, 1), (1, 1)],
        }
    }

    fn z() -> Self {
        Self {
            color: Color::rgb_u8(220, 48, 33),
            positions: vec![(0, 0), (0, -1), (1, 0), (1, 1)],
        }
    }

    fn s() -> Self {
        Self {
            color: Color::rgb_u8(110, 237, 72),
            positions: vec![(0, 0), (1, 0), (0, 1), (1, -1)],
        }
    }

    fn o() -> Self {
        Self {
            color: Color::rgb_u8(243, 241, 80),
            positions: vec![(0, 0), (0, 1), (1, 0), (1, 1)],
        }
    }

    fn t() -> Self {
        Self {
            color: Color::rgb_u8(148, 29, 230),
            positions: vec![(0, 0), (-1, 0), (1, 0), (0, 1)],
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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::i(BlockPattern::i(), Color::rgb_u8(110, 237, 240), vec![(0, 0), (0, -1), (0, 1), (0, 2)])]
    #[case::l(BlockPattern::l(), Color::rgb_u8(230, 163, 58), vec![(0, 0), (0, -1), (0, 1), (-1, 1)])]
    #[case::j(BlockPattern::j(), Color::rgb_u8(0, 0, 232), vec![(0, 0), (0, -1), (0, 1), (1, 1)])]
    #[case::z(BlockPattern::z(), Color::rgb_u8(220, 48, 33), vec![(0, 0), (0, -1), (1, 0), (1, 1)])]
    #[case::s(BlockPattern::s(), Color::rgb_u8(110, 237, 72), vec![(0, 0), (1, 0), (0, 1), (1, -1)])]
    #[case::o(BlockPattern::o(), Color::rgb_u8(243, 241, 80), vec![(0, 0), (0, 1), (1, 0), (1, 1)])]
    #[case::t(BlockPattern::t(), Color::rgb_u8(148, 29, 230), vec![(0, 0), (-1, 0), (1, 0), (0, 1)])]
    #[should_panic]
    #[case::color_mismatch(BlockPattern::l(), Color::rgb_u8(110, 237, 240), vec![(0, 0), (0, -1), (0, 1), (-1, 1)])]
    #[should_panic]
    #[case::positions_mismatch(BlockPattern::l(), Color::rgb_u8(230, 163, 58), vec![(0, 0), (0, -1), (1, 1), (-1, 1)])]
    fn test_block_patterns(
        #[case] pattern: BlockPattern,
        #[case] expected_color: Color,
        #[case] expected_positions: Vec<(i32, i32)>,
    ) {
        assert_eq!(expected_color, pattern.color);
        assert_eq!(expected_positions, pattern.positions)
    }
}
