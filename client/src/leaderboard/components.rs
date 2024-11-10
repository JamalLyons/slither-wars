use bevy::prelude::*;

#[derive(Component)]
pub struct Leaderboard;

#[derive(Component)]
pub struct LeaderboardEntry {
    pub name: String,
    pub score: u32,
} 