use bevy::prelude::*;

use crate::constants::*;

#[derive(Resource)]
pub struct GlobalGameState
{
    pub total_snakes: usize,
    pub total_orbs: usize,
}

impl Default for GlobalGameState
{
    fn default() -> Self
    {
        Self {
            total_snakes: 0,
            total_orbs: MAX_ORB_SPAWN_COUNT,
        }
    }
}
