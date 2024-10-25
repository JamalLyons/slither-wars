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
#[derive(Component)]
pub struct Player {
    pub name: String,
    pub score: u32,
    pub length: u32,
    pub segments: Vec<Segment>,
}

impl Player {
    pub fn new(name: String) -> Self {
        // Add default segments based on player length
        for _ in 0..PLAYER_DEFAULT_LENGTH {
            
        }

        Self {
            name,
            score: 0,
            length: PLAYER_DEFAULT_LENGTH,
            segments: Vec::new(),
        }
    }
}

/// A segment of the player snake body
#[derive(Component)]
pub struct Segment;

/// An orb that the player can collect to increase their length
#[derive(Component)]
pub struct Orb;

/// Despawn's all entities with the given component
pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands)
{
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
