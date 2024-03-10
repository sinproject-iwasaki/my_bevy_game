use bevy::prelude::*;

use crate::{block_pattern::BlockPatterns, sprite};

// #[derive(Resource)]
// pub struct EventTriggerState {
//     event_timer: Timer,
// }

// impl Default for EventTriggerState {
//     fn default() -> Self {
//         Self {
//             event_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
//         }
//     }
// }

#[derive(Event, Default)]
pub struct NewBlockEvent;

// pub fn event_trigger(
//     time: Res<Time>,
//     mut state: ResMut<EventTriggerState>,
//     mut new_block_events: EventWriter<NewBlockEvent>,
// ) {
//     if state.event_timer.tick(time.delta()).finished() {
//         new_block_events.send_default();
//     }
// }

pub fn new_bock_event_listener(
    mut commands: Commands,
    mut new_block_events: EventReader<NewBlockEvent>,
    mut block_patterns: Res<BlockPatterns>,
) {
    for _ in new_block_events.read() {
        sprite::spawn_block(&mut commands, &mut block_patterns)
    }
}
