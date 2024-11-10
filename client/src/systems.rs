use std::collections::HashSet;

use bevy::core::FrameCount;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bot::components::Bot;
use components::{DeadSnake, GameWorld, Segment, Snake, SnakeSegment};
use orb::components::Orb;
use player::components::Player;

use crate::*;

pub fn spawn_game_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
)
{
    commands.spawn((
        GameWorld,
        Name::new("Map Boundary"),
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(MAP_RADIUS)).into(),
            material: materials.add(Color::srgb(0.1, 0.1, 0.1)),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, Z_BACKGROUND),
                ..default()
            },
            ..default()
        },
    ));
}

pub fn spawn_camera(mut commands: Commands)
{
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true, // HDR is required for the bloom effect
                ..default()
            },
            ..default()
        },
        BloomSettings::NATURAL,
    ));
}

/// We use this to avoid the white window that shows up before the GPU is ready to render the app.
/// This happens so fast the the user will not see it.
pub fn make_window_visible(mut window: Query<&mut Window>, frames: Res<FrameCount>)
{
    if frames.0 == 3 {
        // At this point the gpu is ready to show the app so we can make the window visible.
        window.single_mut().visible = true;
    }
}

pub fn check_snake_collisions(
    mut commands: Commands,
    snake_query: Query<(Entity, &Snake, &Transform)>,
    segment_query: Query<(Entity, &SnakeSegment, &Transform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
)
{
    let mut processed_deaths: HashSet<Entity> = HashSet::new();

    for (segment_entity, segment, segment_transform) in segment_query.iter() {
        if processed_deaths.contains(&segment.owner) {
            continue;
        }

        for (other_segment_entity, other_segment, other_transform) in segment_query.iter() {
            if segment_entity == other_segment_entity || segment.owner == other_segment.owner {
                continue;
            }

            if processed_deaths.contains(&other_segment.owner) {
                continue;
            }

            let collision = collide(
                segment_transform.translation,
                Vec2::new(SEGMENT_SIZE, SEGMENT_SIZE),
                other_transform.translation,
                Vec2::new(SEGMENT_SIZE, SEGMENT_SIZE),
            );

            if collision.is_some() {
                if let Ok((snake_entity, snake, snake_transform)) = snake_query.get(segment.owner) {
                    // Store the death position before despawning
                    let death_position = snake_transform.translation;

                    // First, despawn all segments
                    for &segment_entity in &snake.segments {
                        commands.entity(segment_entity).despawn_recursive();
                    }

                    // Then despawn the snake entity itself
                    commands
                        .entity(snake_entity)
                        .insert(DeadSnake {
                            killer: other_segment.owner,
                        })
                        .remove::<Snake>(); // Remove the Snake component to stop movement systems

                    // Spawn death orbs at the death position
                    spawn_death_orbs(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        snake_entity,
                        snake.length,
                        other_segment.owner,
                        false,
                        snake.color,
                        death_position,
                    );

                    processed_deaths.insert(snake_entity);
                }
            }
        }
    }
}

fn spawn_death_orbs(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    dead_entity: Entity,
    snake_length: u32,
    killer_entity: Entity,
    was_player: bool,
    color: Color,
    death_position: Vec3, // Add death_position parameter
)
{
    let num_orbs = snake_length / 2;
    let orb_value = 1;
    let spawn_radius = 50.0;

    for i in 0..num_orbs {
        let angle = (i as f32 / num_orbs as f32) * std::f32::consts::TAU;
        let offset = Vec3::new(angle.cos() * spawn_radius, angle.sin() * spawn_radius, Z_ORB_LAYER);

        commands.spawn((
            Orb {
                radius: ORB_RADIUS,
                value: orb_value,
            },
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle::new(ORB_RADIUS)).into(),
                material: materials.add(ColorMaterial::from(color)),
                transform: Transform::from_translation(death_position + offset),
                ..default()
            },
        ));
    }
}

fn collide(position1: Vec3, size1: Vec2, position2: Vec3, size2: Vec2) -> Option<()>
{
    // Convert Vec3 to Vec2 by dropping the z component
    let pos1 = position1.truncate();
    let pos2 = position2.truncate();

    // Calculate the half-sizes
    let half_size1 = size1 * 0.5;
    let half_size2 = size2 * 0.5;

    // Calculate the bounds for each rectangle
    let min1 = pos1 - half_size1;
    let max1 = pos1 + half_size1;
    let min2 = pos2 - half_size2;
    let max2 = pos2 + half_size2;

    // Check for overlap on both axes
    if max1.x > min2.x && min1.x < max2.x && max1.y > min2.y && min1.y < max2.y {
        Some(()) // Collision detected
    } else {
        None // No collision
    }
}

pub fn cleanup_dead_snakes(mut commands: Commands, dead_snakes: Query<Entity, With<DeadSnake>>)
{
    for dead_snake in dead_snakes.iter() {
        commands.entity(dead_snake).despawn_recursive();
    }
}

pub fn orb_collection(
    mut commands: Commands,
    mut snake_query: Query<(Entity, &Transform, &mut Snake, Option<&mut Player>, Option<&mut Bot>)>,
    orb_query: Query<(Entity, &Transform, &Orb)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
)
{
    for (snake_entity, snake_transform, mut snake, mut player, mut bot) in snake_query.iter_mut() {
        for (orb_entity, orb_transform, orb) in orb_query.iter() {
            let collision = collide(
                snake_transform.translation,
                Vec2::new(SEGMENT_SIZE * 2.0, SEGMENT_SIZE * 2.0),
                orb_transform.translation,
                Vec2::new(ORB_RADIUS * 2.0, ORB_RADIUS * 2.0),
            );

            if collision.is_some() {
                commands.entity(orb_entity).despawn_recursive();

                // Update score and calculate new radius
                let new_radius = if let Some(player) = &mut player {
                    player.score += orb.value;
                    calculate_radius(player.score)
                } else if let Some(ref mut bot) = bot {
                    bot.score += orb.value;
                    calculate_radius(bot.score)
                } else {
                    PLAYER_DEFAULT_RADIUS
                };

                // Add new segment with the calculated radius
                let segment_entity = commands
                    .spawn((
                        Segment {
                            index: snake.length,
                            radius: new_radius,
                        },
                        SnakeSegment { owner: snake_entity },
                        MaterialMesh2dBundle {
                            mesh: meshes.add(Circle::new(1.0)).into(),
                            material: materials.add(ColorMaterial::from(snake.color)),
                            transform: Transform {
                                translation: snake_transform.translation,
                                scale: Vec3::new(new_radius, new_radius, Z_PLAYER_SEGMENTS),
                                ..default()
                            },
                            ..default()
                        },
                    ))
                    .id();

                snake.segments.push_back(segment_entity);
                snake.length += orb.value;
            }
        }
    }
}

// Add this helper function to calculate radius based on score
fn calculate_radius(score: u32) -> f32
{
    let stages = score / SCORE_PER_RADIUS_STAGE;
    MIN_PLAYER_RADIUS + (stages as f32 * RADIUS_GROWTH_PER_STAGE)
}

// Add a system to update existing segment sizes
pub fn update_segment_sizes(
    snake_query: Query<(Entity, Option<&Player>, Option<&Bot>)>,
    mut segment_query: Query<(&mut Transform, &SnakeSegment)>,
)
{
    for (snake_entity, player, bot) in snake_query.iter() {
        let radius = if let Some(player) = player {
            calculate_radius(player.score)
        } else if let Some(bot) = bot {
            calculate_radius(bot.score)
        } else {
            continue;
        };

        // Update all segments belonging to this snake
        for (mut transform, segment) in segment_query.iter_mut() {
            if segment.owner == snake_entity {
                transform.scale = Vec3::new(radius, radius, transform.scale.z);
            }
        }
    }
}
