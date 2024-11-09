use bevy::core::FrameCount;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use components::{DeadSnake, GameWorld, Snake, SnakeSegment};
use orb::components::Orb;
use std::collections::HashSet;

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
) {
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
                    commands.entity(snake_entity)
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
) {
    let num_orbs = snake_length / 2;
    let orb_value = 1;
    let spawn_radius = 50.0;

    for i in 0..num_orbs {
        let angle = (i as f32 / num_orbs as f32) * std::f32::consts::TAU;
        let offset = Vec3::new(
            angle.cos() * spawn_radius,
            angle.sin() * spawn_radius,
            Z_ORB_LAYER,
        );

        commands.spawn((
            Orb { radius: ORB_RADIUS },
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle::new(ORB_RADIUS)).into(),
                material: materials.add(ColorMaterial::from(color)),
                transform: Transform::from_translation(death_position + offset),
                ..default()
            },
        ));
    }
}

fn collide(position1: Vec3, size1: Vec2, position2: Vec3, size2: Vec2) -> Option<()> {
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

pub fn cleanup_dead_snakes(
    mut commands: Commands,
    dead_snakes: Query<Entity, With<DeadSnake>>,
) {
    for dead_snake in dead_snakes.iter() {
        commands.entity(dead_snake).despawn_recursive();
    }
}