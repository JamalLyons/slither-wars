use std::collections::VecDeque;

use bevy::prelude::*;
use rand::Rng;

use crate::constants::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState
{
    #[default]
    Splash,
    Menu,
    Game,
}

#[derive(Resource)]
pub struct GameSettings
{
    pub map_radius: f32,
    pub total_players: usize,
    pub total_orbs: usize,
}

impl Default for GameSettings
{
    fn default() -> Self
    {
        Self {
            map_radius: MAP_RADIUS,
            total_players: 1,
            total_orbs: 50,
        }
    }
}

/// The player of the game that controls the snake
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
        }
    }
}

/// A segment of the player snake body
#[derive(Component, Clone, Debug)]
pub struct Segment
{
    pub radius: f32,
}

/// The history of the player's position
/// This is needed to know how to move the player segments in the game
#[derive(Component, Clone, Debug)]
pub struct PositionHistory
{
    pub positions: VecDeque<Vec3>,
}

impl Default for PositionHistory
{
    fn default() -> Self
    {
        Self {
            positions: VecDeque::with_capacity(MAX_SEGMENT_HISTORY),
        }
    }
}

/// An orb that the player can collect to increase their length
#[derive(Component, Clone, Debug)]
pub struct Orb;

/// Despawn's all entities with the given component
pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands)
{
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn generate_random_position_within_radius(radius: f32) -> Vec2
{
    let mut rng = rand::thread_rng();
    let angle = rng.gen_range(0.0..std::f32::consts::TAU);
    let distance = rng.gen_range(0.0..radius);

    Vec2::new(distance * angle.cos(), distance * angle.sin())
}

pub fn generate_random_color() -> Color
{
    let colors = vec![
        Color::srgb(1.0, 0.0, 0.0),  // Red
        Color::srgb(0.0, 1.0, 0.0),  // Green
        Color::srgb(0.0, 0.0, 1.0),  // Blue
        Color::srgb(1.0, 1.0, 0.0),  // Yellow
        Color::srgb(1.0, 0.65, 0.0), // Orange
        Color::srgb(0.5, 0.0, 0.5),  // Purple
        Color::srgb(0.0, 1.0, 1.0),  // Cyan
        Color::srgb(1.0, 0.75, 0.8), // Pink
    ];

    let mut rng = rand::thread_rng();
    colors[rng.gen_range(0..colors.len())]
}
