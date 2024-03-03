// use bevy::prelude::*;
// use rand::seq::SliceRandom;

// #[derive(Resource)]
// pub struct ColorResources(Vec<Color>);

// impl ColorResources {
//     pub fn random(&self) -> Option<Color> {
//         self.0.choose(&mut rand::thread_rng()).cloned()
//     }

//     pub fn generate_example() -> Self {
//         let colors = vec![
//             Color::rgb_u8(64, 230, 100),
//             Color::rgb_u8(220, 64, 90),
//             Color::rgb_u8(70, 150, 210),
//             Color::rgb_u8(220, 230, 70),
//             Color::rgb_u8(35, 220, 241),
//             Color::rgb_u8(240, 140, 70),
//         ];

//         Self(colors)
//     }
// }
