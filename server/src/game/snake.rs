use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::constants::Rgb;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Snake
{
    pub id: Uuid,
    pub name: String,
    pub position: (f32, f32),
    pub direction: f32,
    pub speed: f32,
    pub body: Vec<(f32, f32)>,
    pub length: u32,
    pub is_dead: bool,
    pub score: u32,
    pub is_bot: bool,
    pub color: Rgb,
}

impl Snake
{
    pub fn new(id: Uuid, name: String, color: Rgb, is_bot: bool, pos: (f32, f32)) -> Self
    {
        Self {
            id,
            name,
            position: pos,
            direction: 0.0,
            speed: crate::constants::PLAYER_DEFAULT_SPEED,
            body: vec![],
            length: 10,
            is_dead: false,
            score: 0,
            is_bot,
            color,
        }
    }

    // Methods to move the snake, rotate, etc.
}
