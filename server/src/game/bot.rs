use uuid::Uuid;

use super::snake::Snake;
use crate::constants::Rgb;

pub struct Bot
{
    pub snake: Snake,
}

impl Bot
{
    pub fn new(id: Uuid, name: String, color: Rgb, is_bot: bool, pos: (f32, f32)) -> Self
    {
        Self {
            snake: Snake::new(id, name, color, is_bot, pos),
        }
    }

    pub fn update(&mut self)
    {
        // Logic for bot movement and behavior
    }
}
