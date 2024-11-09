use bevy::prelude::*;
use crate::{constants::BOT_SPAWN_INTERVAL, utils::generate_random_color};

#[derive(Component, Clone, Debug)]
pub struct Bot {
    pub color: Color,
    pub target_position: Option<Vec2>,
    pub decision_timer: Timer,
}

impl Default for Bot {
    fn default() -> Self {
        Self {
            color: generate_random_color(),
            target_position: None,
            decision_timer: Timer::from_seconds(BOT_SPAWN_INTERVAL, TimerMode::Repeating),
        }
    }
}
