use std::collections::VecDeque;
use std::time::Duration;

use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use rand::Rng;

use super::components::Bot;
use crate::components::{Segment, SegmentPositionHistory, Snake, SnakeSegment};
use crate::constants::*;
use crate::orb::components::Orb;
use crate::utils::*;

pub fn spawn_bots(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>)
{
    // Spawn initial bots
    for _ in 0..BOT_DEFAULT_SPAWN_AMOUNT {
        let random_position = generate_random_position_within_radius(MAP_RADIUS);
        let bot_size = Vec3::new(PLAYER_DEFAULT_RADIUS, PLAYER_DEFAULT_RADIUS, Z_BOT_SEGMENTS);

        let bot = Bot::default();

        let bot_entity = commands
            .spawn((
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
            ))
            .id();

        // Spawn initial segments for the bot
        let mut snake_segments = VecDeque::new();
        for i in 0..PLAYER_DEFAULT_LENGTH {
            let segment_entity = commands
                .spawn((
                    Segment {
                        index: i,
                        radius: PLAYER_DEFAULT_RADIUS,
                    },
                    SnakeSegment { owner: bot_entity },
                    MaterialMesh2dBundle {
                        mesh: meshes.add(Circle::new(1.0)).into(),
                        material: materials.add(ColorMaterial::from(bot.color)),
                        transform: Transform {
                            translation: Vec3::new(
                                random_position.x - (i as f32 * SEGMENT_SPACING),
                                random_position.y,
                                Z_BOT_SEGMENTS,
                            ),
                            scale: bot_size,
                            ..default()
                        },
                        ..default()
                    },
                ))
                .id();
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
    mut query_set: ParamSet<(
        Query<(Entity, &mut Transform, &mut Bot, &mut SegmentPositionHistory)>,
        Query<(&mut Transform, &Segment, &SnakeSegment)>,
        Query<&Transform, With<Orb>>,
    )>,
)
{
    let mut bot_movements: Vec<(Entity, Vec3, Vec<Vec3>)> = Vec::new();
    let mut rng = rand::thread_rng();

    // First, collect orb positions
    let nearby_orbs: Vec<Vec2> = query_set.p2().iter().map(|t| t.translation.truncate()).collect();

    // Then update bot positions
    {
        let mut bot_query = query_set.p0();
        for (bot_entity, mut transform, mut bot, mut segment_history) in bot_query.iter_mut() {
            bot.decision_timer.tick(time.delta());

            if bot.decision_timer.just_finished()
                || bot.target_position.map_or(true, |target| {
                    transform.translation.truncate().distance(target) < PLAYER_DEFAULT_RADIUS
                })
            {
                let current_pos = transform.translation.truncate();
                // Filter nearby orbs based on current bot position
                let nearby_orbs: Vec<Vec2> = nearby_orbs
                    .iter()
                    .filter(|pos| current_pos.distance(**pos) < MAP_RADIUS * 0.5)
                    .copied()
                    .collect();

                if !nearby_orbs.is_empty() && rng.gen_bool(0.7) {
                    let closest_orb = nearby_orbs
                        .iter()
                        .min_by(|a, b| {
                            let dist_a = current_pos.distance(**a);
                            let dist_b = current_pos.distance(**b);
                            dist_a.partial_cmp(&dist_b).unwrap()
                        })
                        .unwrap();
                    bot.target_position = Some(*closest_orb);
                } else {
                    let safe_radius = MAP_RADIUS * 0.9;
                    let random_position = generate_random_position_within_radius(safe_radius);
                    bot.target_position = Some(random_position);
                }

                bot.decision_timer
                    .set_duration(Duration::from_secs_f32(BOT_SPAWN_INTERVAL + rng.gen_range(-0.2..0.2)));
            }

            if let Some(target) = bot.target_position {
                let current_pos = transform.translation.truncate();
                let direction = (target - current_pos).normalize();

                let wobble = Vec2::new(rng.gen_range(-0.2..0.2), rng.gen_range(-0.2..0.2));
                let direction = (direction + wobble * 0.1).normalize();

                transform.translation += direction.extend(0.0) * BOT_SPEED * time.delta_seconds();

                let angle = direction.y.atan2(direction.x);
                transform.rotation = Quat::from_rotation_z(angle);

                let distance_from_center = transform.translation.truncate().length();
                if distance_from_center > MAP_RADIUS - PLAYER_DEFAULT_RADIUS {
                    let clamped_position =
                        transform.translation.truncate().normalize() * (MAP_RADIUS - PLAYER_DEFAULT_RADIUS);
                    transform.translation = clamped_position.extend(transform.translation.z);
                    bot.target_position = None;
                }

                segment_history.positions.push_front(transform.translation);
                if segment_history.positions.len() > MAX_SEGMENT_HISTORY {
                    segment_history.positions.pop_back();
                }

                bot_movements.push((bot_entity, transform.translation, segment_history.positions.clone().into()));
            }
        }
    }

    // Finally, update segment positions
    let mut segment_query = query_set.p1();
    for (bot_entity, _bot_pos, history) in bot_movements {
        for (mut segment_transform, segment, snake_segment) in segment_query.iter_mut() {
            if snake_segment.owner == bot_entity {
                let index = (segment.index + 1) * POSITIONS_PER_SEGMENT;
                if index < history.len() as u32 {
                    segment_transform.translation = history[index as usize];
                }
            }
        }
    }
}

pub fn bot_eating(
    mut commands: Commands,
    mut bot_query: Query<(Entity, &Transform, &mut Snake, &Bot), With<Bot>>,
    food_query: Query<(Entity, &Transform), With<Orb>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
)
{
    for (bot_entity, bot_transform, mut snake, bot) in bot_query.iter_mut() {
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
                    bot_entity,
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
    owner: Entity,
) -> Entity
{
    commands
        .spawn((
            Segment {
                index,
                radius: PLAYER_DEFAULT_RADIUS,
            },
            SnakeSegment { owner },
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
