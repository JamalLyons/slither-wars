use std::collections::VecDeque;

use bevy::prelude::*;

use crate::constants::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState
{
    #[default]
    Splash,
    Menu,
    Game,
}

/// The player of the game that controls the snake
#[derive(Component, Clone, Debug)]
pub struct Player
{
    pub name: String,
    pub score: u32,
    pub length: u32,
    pub segments: Vec<Entity>,
}

impl Player
{
    pub fn new(name: String) -> Self
    {
        Self {
            name,
            score: 0,
            length: PLAYER_DEFAULT_LENGTH,
            segments: Vec::new(),
        }
    }
}

/// A segment of the player snake body
#[derive(Component, Clone, Debug)]
pub struct Segment;

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
