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

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn random(&self) -> Option<&BlockPattern> {
        self.0.choose(&mut rand::thread_rng())
    }
}

impl Default for BlockPatterns {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct PatternTest {
        pattern: fn() -> BlockPattern,
        expected_color: Color,
        expected_positions: Vec<(i32, i32)>,
    }

    #[test]
    fn test_block_patterns() {
        let tests = vec![
            PatternTest {
                pattern: BlockPattern::i,
                expected_color: Color::rgb_u8(110, 237, 240),
                expected_positions: vec![(0, 0), (0, -1), (0, 1), (0, 2)],
            },
            PatternTest {
                pattern: BlockPattern::l,
                expected_color: Color::rgb_u8(230, 163, 58),
                expected_positions: vec![(0, 0), (0, -1), (0, 1), (-1, 1)],
            },
            PatternTest {
                pattern: BlockPattern::j,
                expected_color: Color::rgb_u8(0, 0, 232),
                expected_positions: vec![(0, 0), (0, -1), (0, 1), (1, 1)],
            },
            PatternTest {
                pattern: BlockPattern::z,
                expected_color: Color::rgb_u8(220, 48, 33),
                expected_positions: vec![(0, 0), (0, -1), (1, 0), (1, 1)],
            },
            PatternTest {
                pattern: BlockPattern::s,
                expected_color: Color::rgb_u8(110, 237, 72),
                expected_positions: vec![(0, 0), (1, 0), (0, 1), (1, -1)],
            },
            PatternTest {
                pattern: BlockPattern::o,
                expected_color: Color::rgb_u8(243, 241, 80),
                expected_positions: vec![(0, 0), (0, 1), (1, 0), (1, 1)],
            },
            PatternTest {
                pattern: BlockPattern::t,
                expected_color: Color::rgb_u8(148, 29, 230),
                expected_positions: vec![(0, 0), (-1, 0), (1, 0), (0, 1)],
            },
        ];

        for test in tests {
            let pattern = (test.pattern)();
            assert_eq!(pattern.color, test.expected_color);
            assert_eq!(pattern.positions, test.expected_positions);
        }
    }

    #[test]
    fn test_is_empty() {
        let block_patterns = BlockPatterns::default();
        assert!(!block_patterns.is_empty());
    }

    #[test]
    fn test_block_patterns_length() {
        let block_patterns = BlockPatterns::default();
        assert_eq!(block_patterns.len(), 7);
    }
}
