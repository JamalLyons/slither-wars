use bevy::prelude::Component;

pub mod camera;

#[derive(Component)]
pub struct Player;

/// Camera lerp factor.
pub const CAM_LERP_FACTOR: f32 = 2.;

/// Player movement speed factor.
pub const PLAYER_SPEED: f32 = 100.;
