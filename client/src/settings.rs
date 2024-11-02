use bevy::prelude::Resource;

use crate::{MAP_RADIUS, MAX_ORB_SPAWN_COUNT};

#[derive(Resource)]
pub struct GameSettings
{
    pub map_radius: f32,
    pub total_players: usize,
    pub total_orbs: usize,
}

impl Default for GameSettings
{
    fn default() -> Self
    {
        Self {
            map_radius: MAP_RADIUS,
            total_players: 1,
            total_orbs: MAX_ORB_SPAWN_COUNT,
        }
    }
}
