pub mod components;
pub mod systems;

use bevy::prelude::*;
use systems::*;

pub struct LeaderboardPlugin;

impl Plugin for LeaderboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_leaderboard)
           .add_systems(Update, update_leaderboard);
    }
} 