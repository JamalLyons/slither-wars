use std::collections::VecDeque;

use bevy::prelude::*;

use crate::PLAYER_DEFAULT_LENGTH;

#[derive(Component)]
pub struct GameWorld;

#[derive(Component)]
pub struct Snake
{
    pub length: u32,
    pub segments: VecDeque<Entity>,
    pub color: Color,
}

impl Snake
{
    pub fn new(color: Color) -> Self
    {
        Self {
            length: PLAYER_DEFAULT_LENGTH,
            segments: VecDeque::new(),
            color,
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

#[derive(Component)]
pub struct SnakeSegment
{
    pub owner: Entity, // This will store the entity ID of the snake (bot or player) that owns this segment
}

#[derive(Component)]
pub struct DeadSnake {
    pub killer: Entity,
}