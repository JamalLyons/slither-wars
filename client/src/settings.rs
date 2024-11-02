use bevy::prelude::Resource;

use crate::constants::*;

#[derive(Resource)]
pub struct GameSettings
{
    pub map_radius: f32,
    pub player_count: usize,
    pub bot_count: usize,
    pub total_orbs: usize,
    pub total_bots: usize,
}

impl Default for GameSettings
{
    fn default() -> Self
    {
        Self {
            map_radius: MAP_RADIUS,
            player_count: 1,
            bot_count: BOT_DEFAULT_SPAWN_AMOUNT,
            total_orbs: MAX_ORB_SPAWN_COUNT,
            total_bots: MAX_BOT_SPAWN_COUNT,
        }
    }
}
