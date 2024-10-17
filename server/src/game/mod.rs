use crate::constants;

pub mod bot;
pub mod food;
pub mod leaderboard;
pub mod snake;
pub mod world;

pub fn create_random_position() -> (f32, f32)
{
    (
        rand::random::<f32>() * constants::WORLD_WIDTH,
        rand::random::<f32>() * constants::WORLD_HEIGHT,
    )
}

pub fn create_player_id() -> uuid::Uuid
{
    uuid::Uuid::new_v4()
}
