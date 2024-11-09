use bevy::color::Color;

/// Player movement and growth constants
pub const PLAYER_SPEED: f32 = 100.; // Pixels per second
pub const PLAYER_DEFAULT_RADIUS: f32 = 12.5;
pub const PLAYER_DEFAULT_LENGTH: u32 = 1; // Number of segments the player starts with

/// Segment movement and spacing constants
pub const SEGMENT_SPACING: f32 = 25.0; // Pixels between each segment
pub const POSITIONS_PER_SEGMENT: u32 = 5; // Number of positions per segment
pub const MAX_SEGMENT_HISTORY: usize = 100_000; // The max size a plyaer can be in the game

/// Orb constants
pub const ORB_RADIUS: f32 = 5.0;
pub const BOOST_ORB_RADIUS: f32 = 4.0;
pub const MAX_ORB_SPAWN_COUNT: usize = 5000;

/// Orb spawning and scoring constants
pub const ORB_SPAWN_PER_PLAYER: usize = 100;
pub const ORB_SPAWN_INTERVAL: f32 = 0.6; // Orb spawn interval during boosting
pub const ORB_SPAWN_DISTANCE_MARGIN: f32 = 1.0;
pub const SCORE_PER_ORB: u32 = 1;
pub const SCORE_NEEDED_FOR_BOOSTING: u32 = 5;

/// Bot constants
pub const BOT_SPAWN_INTERVAL: f32 = 0.5;
pub const BOT_DEFAULT_SPAWN_AMOUNT: usize = 3;
pub const MAX_BOT_SPAWN_COUNT: usize = 25;
pub const BOT_SPEED: f32 = 100.0;
pub const BOT_DIRECTION_CHANGE_CHANCE: f32 = 0.02; // Adjust as needed

/// Radius growth constants
pub const RADIUS_GROWTH_PER_STAGE: f32 = 2.0; // Amount to increase radius per stage
pub const SCORE_PER_RADIUS_STAGE: u32 = 250; // Score required to increase radius
pub const MIN_PLAYER_RADIUS: f32 = 12.5; // Minimum player radius

/// Color constants
pub const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

/// Screen and map constants
pub const SCREEN_WIDTH: f32 = 1000.;
pub const SCREEN_HEIGHT: f32 = 700.;

pub const MAP_RADIUS: f32 = 500.;

/// Camera constants
pub const CAM_LERP_FACTOR: f32 = 5.;

/// Window settings constants
pub const WINDOW_TITLE: &str = "Slither Wars Client";
pub const WINDOW_NAME: &str = "slither-wars.app";

// Z-Ordering Constants
pub const Z_BACKGROUND: f32 = -10.0;
pub const Z_ORBS: f32 = 0.0;
pub const Z_BOT_SEGMENTS: f32 = 1.0;
pub const Z_PLAYER_SEGMENTS: f32 = 2.0;
