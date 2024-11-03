// for some reason rust does not let me name this file 'segment' lol
// not sure if its a cargo thing...

use std::collections::VecDeque;

use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::bot::Bot;
use crate::player::Player;
use crate::MAX_SEGMENT_HISTORY;
use crate::constants::*;

/// A segment of the player snake body
#[derive(Component)]
pub struct Segment
{
    pub radius: f32,
    pub index: u32,
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
            positions: VecDeque::with_capacity(MAX_SEGMENT_HISTORY as usize),
        }
    }
}

pub fn spawn_segment(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec3,
    color: Color,
    radius: f32,
    index: u32,
    is_bot: bool,
) -> Entity
{
    let z_index = if is_bot { Z_BOT_SEGMENTS } else { Z_PLAYER_SEGMENTS };

    commands
        .spawn((
            Segment { radius, index },
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle::new(1.0)).into(),
                material: materials.add(ColorMaterial::from(color)),
                transform: Transform {
                    translation: position,
                    scale: Vec3::new(radius, radius, z_index),
                    ..default()
                },
                ..default()
            },
        ))
        .id()
}

pub fn remove_player_segments(commands: &mut Commands, player: &mut Player, segments_to_remove: u32)
{
    let segments_to_remove = segments_to_remove.min(player.segment_count);

    for _ in 0..segments_to_remove {
        let segment_entity = player.segments.pop_back();
        if let Some(entity) = segment_entity {
            commands.entity(entity).despawn();
            player.segment_count -= 1;
        }
    }
}

pub fn remove__bot_segments(commands: &mut Commands, bot: &mut Bot, segments_to_remove: u32)
{
    let segments_to_remove = segments_to_remove.min(bot.segment_count);

    for _ in 0..segments_to_remove {
        let segment_entity = bot.segments.pop_back();
        if let Some(entity) = segment_entity {
            commands.entity(entity).despawn();
            bot.segment_count -= 1;
        }
    }
}
