use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::bot::Bot;
use crate::constants::*;
use crate::player::Player;
use crate::segments::spawn_segment;
use crate::settings::GameSettings;
use crate::utils::{generate_random_color, generate_random_position_within_radius};

/// An orb that the player can collect to increase their length
#[derive(Component, Clone, Debug)]
pub struct Orb
{
    pub radius: f32,
}

pub fn orb_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_settings: Res<GameSettings>,
    existing_orbs: Query<Entity, With<Orb>>,
)
{
    let desired_orb_count = calculate_desired_orb_count(&game_settings);

    let current_orb_count = existing_orbs.iter().count();

    if current_orb_count < desired_orb_count {
        let orbs_to_spawn = desired_orb_count - current_orb_count;

        for _ in 0..orbs_to_spawn {
            let position = generate_random_position_within_radius(game_settings.map_radius);
            let color = generate_random_color();

            spawn_orb(&mut commands, &mut meshes, &mut materials, color, position, ORB_RADIUS);
        }
    }
}

pub fn orb_collection_system(
    mut commands: Commands,
    mut player_query: Query<(&mut Transform, &mut Player), (Without<Orb>, Without<Bot>)>,
    mut bot_query: Query<(&mut Transform, &mut Bot), (Without<Orb>, Without<Player>)>,
    orb_query: Query<(Entity, &Transform, &Orb), (Without<Player>, Without<Bot>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
)
{
    for (player_transform, mut player) in player_query.iter_mut() {
        for (orb_entity, orb_transform, orb) in orb_query.iter() {
            let distance = player_transform
                .translation
                .truncate()
                .distance(orb_transform.translation.truncate());

            if distance < player.radius + orb.radius {
                // Collect the orb
                commands.entity(orb_entity).despawn();
                player.score += SCORE_PER_ORB;

                // Add a new segment
                let segment_entity = spawn_segment(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    player_transform.translation,
                    player.color,
                    player.radius,
                    player.segment_count,
                    false
                );

                // Store the segment entity
                player.segments.push_back(segment_entity);

                // Increment the segment count
                player.segment_count += 1;
            }
        }
    }

    for (bot_transform, mut bot) in bot_query.iter_mut() {
        for (orb_entity, orb_transform, orb) in orb_query.iter() {
            let distance = bot_transform
                .translation
                .truncate()
                .distance(orb_transform.translation.truncate());

            if distance < bot.radius + orb.radius {
                // Collect the orb
                commands.entity(orb_entity).despawn();
                bot.score += SCORE_PER_ORB;

                // Add a new segment
                let segment_entity = spawn_segment(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    bot_transform.translation,
                    bot.color,
                    bot.radius,
                    bot.segment_count,
                    true
                );

                // Store the segment entity
                bot.segments.push_back(segment_entity);

                // Increment the segment count
                bot.segment_count += 1;
            }
        }
    }
}

/// Util function to spawn an orb
pub fn spawn_orb(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    color: Color,
    position: Vec2,
    radius: f32,
) -> Entity
{
    commands
        .spawn((
            Orb { radius },
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle::new(5.0)).into(), // Orbs have a radius of 5.0
                material: materials.add(ColorMaterial::from(color)),
                transform: Transform {
                    translation: position.extend(Z_ORBS),
                    ..default()
                },
                ..default()
            },
        ))
        .id()
}

fn calculate_desired_orb_count(game_settings: &GameSettings) -> usize
{
    // For example, set 20 orbs per player, adjust as needed
    let base_orbs = ORB_SPAWN_PER_PLAYER * game_settings.player_count;

    // Cap or adjust total orbs based on map size or other factors if needed
    base_orbs.min(game_settings.total_orbs)
}
