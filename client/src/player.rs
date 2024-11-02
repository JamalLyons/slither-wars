use std::collections::VecDeque;

use bevy::{color::Color, prelude::{Component, Entity}};

use crate::{PLAYER_DEFAULT_LENGTH, PLAYER_DEFAULT_RADIUS, MAX_GROWTH_LIMIT};

#[derive(Component, Clone, Debug)]
pub struct Player
{
    pub name: String,
    pub score: u32,
    pub length: u32,
    pub radius: f32,
    pub color: Color,
    pub boost_timer: f32,     // Accumulates time for score deduction
    pub orb_spawn_timer: f32, // Controls orb spawn intervals during boosting
    pub segment_count: u32,
    pub segments: VecDeque<Entity>,
}

impl Player
{
    pub fn new(name: String, color: Color) -> Self
    {
        Self {
            name,
            score: 0,
            length: PLAYER_DEFAULT_LENGTH,
            radius: PLAYER_DEFAULT_RADIUS,
            color,
            boost_timer: 0.0,
            orb_spawn_timer: 0.0,
            segment_count: 0,
            segments: VecDeque::with_capacity(MAX_GROWTH_LIMIT as usize),
        }
    }
}