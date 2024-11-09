use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use crate::components::{Segment, SegmentPositionHistory, Snake};
use crate::constants::*;
use crate::orb::components::Orb;
use super::components::Bot;
use crate::utils::*;
use std::collections::VecDeque;

pub fn spawn_bots(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn initial bots
    for _ in 0..BOT_DEFAULT_SPAWN_AMOUNT {
        let random_position = generate_random_position_within_radius(MAP_RADIUS);
        let bot_size = Vec3::new(PLAYER_DEFAULT_RADIUS, PLAYER_DEFAULT_RADIUS, Z_BOT_SEGMENTS);

        let bot = Bot::default();

        let bot_entity = commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle::new(1.0)).into(),
                material: materials.add(ColorMaterial::from(bot.color.clone())),
                transform: Transform {
                    translation: random_position.extend(Z_BOT_SEGMENTS),
                    scale: bot_size,
                    ..default()
                },
                ..default()
            },
            bot.clone(),
            Snake::default(),
            SegmentPositionHistory::default(),
        )).id();

        // Spawn initial segments for the bot
        let mut snake_segments = VecDeque::new();
        for i in 0..PLAYER_DEFAULT_LENGTH {
            let segment_entity = commands.spawn((
                Segment {
                    index: i,
                    radius: PLAYER_DEFAULT_RADIUS,
                },
                MaterialMesh2dBundle {
                    mesh: meshes.add(Circle::new(1.0)).into(),
                    material: materials.add(ColorMaterial::from(bot.color)),
                    transform: Transform {
                        translation: Vec3::new(
                            random_position.x - (i as f32 * SEGMENT_SPACING),
                            random_position.y,
                            Z_BOT_SEGMENTS
                        ),
                        scale: bot_size,
                        ..default()
                    },
                    ..default()
                },
            )).id();
            snake_segments.push_back(segment_entity);
        }

        // Add segments to the Snake component
        if let Some(mut snake) = commands.get_entity(bot_entity) {
            snake.insert(Snake {
                length: PLAYER_DEFAULT_LENGTH,
                segments: snake_segments,
            });
        }
    }
}

pub fn bot_movement(
    time: Res<Time>,
    mut bot_query: Query<(&mut Transform, &mut Bot, &mut SegmentPositionHistory)>,
    mut segment_query: Query<(&mut Transform, &Segment), Without<Bot>>,
) {
    for (mut transform, mut bot, mut segment_history) in bot_query.iter_mut() {
        bot.decision_timer.tick(time.delta());

        if bot.decision_timer.just_finished() {
            let random_position = generate_random_position_within_radius(MAP_RADIUS);
            bot.target_position = Some(random_position);
        }

        if let Some(target) = bot.target_position {
            let direction = (target - transform.translation.truncate()).normalize();
            transform.translation += direction.extend(0.0) * BOT_SPEED * time.delta_seconds();
            
            // Update rotation to face movement direction
            let angle = direction.y.atan2(direction.x);
            transform.rotation = Quat::from_rotation_z(angle);

            // Update segment history
            segment_history.positions.push_front(transform.translation);
            if segment_history.positions.len() > MAX_SEGMENT_HISTORY {
                segment_history.positions.pop_back();
            }

            // Update segments
            for (mut segment_transform, segment) in segment_query.iter_mut() {
                let index = (segment.index + 1) * POSITIONS_PER_SEGMENT;
                if index < segment_history.positions.len().try_into().unwrap() {
                    segment_transform.translation = segment_history.positions[index as usize];
                } else {
                    segment_transform.translation = transform.translation;
                }
            }
        }
    }
}

pub fn bot_eating(
    mut commands: Commands,
    mut bot_query: Query<(&Transform, &mut Snake, &Bot), With<Bot>>,
    food_query: Query<(Entity, &Transform), With<Orb>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (bot_transform, mut snake, bot) in bot_query.iter_mut() {
        for (food_entity, food_transform) in food_query.iter() {
            let distance = bot_transform.translation.distance(food_transform.translation);
            
            if distance < PLAYER_DEFAULT_RADIUS + ORB_RADIUS {
                commands.entity(food_entity).despawn();

                // Add new segment
                let segment_entity = add_segment(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    bot_transform.translation,
                    snake.length,
                    bot.color,
                );

                snake.segments.push_back(segment_entity);
                snake.length += 1;
            }
        }
    }
}

fn add_segment(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec3,
    index: u32,
    color: Color,
) -> Entity {
    commands
        .spawn((
            Segment {
                index,
                radius: PLAYER_DEFAULT_RADIUS,
            },
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle::new(1.0)).into(),
                material: materials.add(ColorMaterial::from(color)),
                transform: Transform {
                    translation: position,
                    scale: Vec3::new(PLAYER_DEFAULT_RADIUS, PLAYER_DEFAULT_RADIUS, Z_BOT_SEGMENTS),
                    ..default()
                },
                ..default()
            },
        ))
        .id()
}
