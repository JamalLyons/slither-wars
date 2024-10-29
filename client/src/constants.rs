use bevy::color::Color;

pub const CAM_LERP_FACTOR: f32 = 5.;

pub const PLAYER_SPEED: f32 = 100.; // Pixels per second
pub const PLAYER_DEFAULT_RADIUS: f32 = 12.5;
pub const PLAYER_DEFAULT_LENGTH: u32 = 10; // Number of segments the player starts with
pub const PLAYER_BOOST_MULTIPLIER: f32 = 2.;
pub const PLAYER_ORB_SPAWN_INTERVAL: f32 = 0.2;

pub const SEGMENT_SPACING: f32 = 25.0; // Pixels between each segment
pub const MAX_SEGMENT_HISTORY: usize = 1000; // Number of positions to keep track of at once
pub const POSITIONS_PER_SEGMENT: usize = 5; // Number of positions per segment
pub const SCORE_PER_ORB: u32 = 1;
pub const SCORE_NEEDED_FOR_BOOSTING: u32 = 5;

pub const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

pub const SCREEN_WIDTH: f32 = 1000.;
pub const SCREEN_HEIGHT: f32 = 700.;

pub const MAP_RADIUS: f32 = 500.;
