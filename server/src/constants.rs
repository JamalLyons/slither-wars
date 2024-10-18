use rand::seq::IteratorRandom;
use serde::{Deserialize, Serialize};

pub const MAX_PLAYERS: usize = 100;
pub const PLAYER_DEFAULT_SPEED: f32 = 1.0;
pub const WORLD_WIDTH: f32 = 5000.0;
pub const WORLD_HEIGHT: f32 = 5000.0;
pub const COLLISION_THRESHOLD: f32 = 10.0;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Rgb(pub u8, pub u8, pub u8);

impl Rgb
{
    pub fn random() -> Rgb
    {
        COMMON_COLORS.iter().choose(&mut rand::thread_rng()).unwrap().clone()
    }

    pub fn random_food() -> Rgb
    {
        FOOD_COLORS.iter().choose(&mut rand::thread_rng()).unwrap().clone()
    }

    pub fn to_tuple(&self) -> (u8, u8, u8)
    {
        (self.0, self.1, self.2)
    }
}

// Common colors
pub const RED: Rgb = Rgb(255, 0, 0);
pub const GREEN: Rgb = Rgb(0, 255, 0);
pub const BLUE: Rgb = Rgb(0, 0, 255);
pub const YELLOW: Rgb = Rgb(255, 255, 0);
pub const CYAN: Rgb = Rgb(0, 255, 255);
pub const MAGENTA: Rgb = Rgb(255, 0, 255);
pub const ORANGE: Rgb = Rgb(255, 165, 0);
pub const PURPLE: Rgb = Rgb(128, 0, 128);
pub const PINK: Rgb = Rgb(255, 192, 203);
pub const BROWN: Rgb = Rgb(165, 42, 42);
pub const BLACK: Rgb = Rgb(0, 0, 0);
pub const WHITE: Rgb = Rgb(255, 255, 255);
pub const GRAY: Rgb = Rgb(128, 128, 128);

pub const COMMON_COLORS: [Rgb; 13] = [
    RED, GREEN, BLUE, YELLOW, CYAN, MAGENTA, ORANGE, PURPLE, PINK, BROWN, BLACK, WHITE, GRAY,
];

pub const FOOD_COLORS: [Rgb; 5] = [RED, GREEN, BLUE, YELLOW, PINK];
