use bevy::prelude::*;

#[derive(Resource)]
pub struct GameTimer {
    timer: Timer,
}

impl GameTimer {
    pub fn new() -> Self {
        Self {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }

    pub fn game_timer(time: Res<Time>, mut game_timer: ResMut<Self>) {
        if game_timer.timer.tick(time.delta()).finished() {
            println!("Game timer ticked!");
        }
    }

    pub fn timer(&self) -> &Timer {
        &self.timer
    }
}

impl Default for GameTimer {
    fn default() -> Self {
        Self::new()
    }
}
