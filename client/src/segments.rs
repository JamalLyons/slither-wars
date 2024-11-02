// for some reason rust does not let me name this file 'segment' lol
// not sure if its a cargo thing...

use std::collections::VecDeque;

use bevy::{math::Vec3, prelude::Component};

use crate::MAX_SEGMENT_HISTORY;

/// A segment of the player snake body
#[derive(Component)]
pub struct Segment
{
    pub radius: f32,
    pub index: u32,
}

/// The history of the player's position
/// This is needed to know how to move the player segments in the game
#[derive(Component, Clone, Debug)]
pub struct PositionHistory
{
    pub positions: VecDeque<Vec3>,
}

impl Default for PositionHistory
{
    fn default() -> Self
    {
        Self {
            positions: VecDeque::with_capacity(MAX_SEGMENT_HISTORY as usize),
        }
    }
}