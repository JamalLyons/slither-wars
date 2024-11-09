use bevy::prelude::*;

use crate::constants::*;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component, Clone, Debug)]
pub struct Player
{
    pub name: String,
    pub score: u32,
    pub radius: f32,
    pub color: Color,
    pub boost_timer: f32,
    pub orb_spawn_timer: f32,
}

impl Player
{
    pub fn new(name: String, color: Color) -> Self
    {
        Player {
            name,
            score: 0,
            radius: PLAYER_DEFAULT_RADIUS,
            color,
            boost_timer: 0.0,
            orb_spawn_timer: 0.0,
        }
    }
}
