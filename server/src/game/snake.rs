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
            body: vec![pos],
            length: 1,
            is_dead: false,
            score: 0,
            is_bot,
            color,
        }
    }

    pub fn move_forward(&mut self) {
        // Update position based on direction and speed
        let delta_x = self.speed * self.direction.to_radians().cos();
        let delta_y = self.speed * self.direction.to_radians().sin();
        self.position.0 += delta_x;
        self.position.1 += delta_y;

        // Add new position to the front of the body
        self.body.insert(0, self.position);

        // Remove last segment if body is longer than length
        if self.body.len() > self.length as usize {
            self.body.pop();
        }
    }

    pub fn grow(&mut self, amount: u32) {
        self.length += amount;
    }
}
