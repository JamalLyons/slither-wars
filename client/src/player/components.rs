use std::collections::VecDeque;

use bevy::prelude::*;

use crate::constants::*;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct Player
{
    pub name: String,
    pub score: u32,
    pub radius: f32,
    pub length: u32,
    pub color: Color,
    pub boost_timer: f32,
    pub orb_spawn_timer: f32,
    pub segment_count: u32,
    pub segments: VecDeque<Entity>,
}

impl Player
{
    pub fn new(name: String, color: Color) -> Self
    {
        Player {
            name,
            score: 0,
            radius: PLAYER_DEFAULT_RADIUS,
            length: PLAYER_DEFAULT_LENGTH,
            color,
            boost_timer: 0.0,
            orb_spawn_timer: 0.0,
            segment_count: 0,
            segments: VecDeque::new(),
        }
    }
}

#[derive(Component)]
pub struct Segment
{
    pub index: u32,
    pub radius: f32,
}

#[derive(Component, Clone, Debug)]
pub struct SegmentPositionHistory
{
    pub positions: VecDeque<Vec3>,
}

impl Default for SegmentPositionHistory
{
    fn default() -> Self
    {
        Self {
            positions: VecDeque::new(),
        }
    }
}
