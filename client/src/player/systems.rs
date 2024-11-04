use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use super::components::*;
use crate::constants::*;
use crate::orb::components::Orb;
use crate::orb::systems::spawn_singlular_orb;
use crate::resources::GlobalGameState;
use crate::utils::*;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut global_game_state: ResMut<GlobalGameState>,
)
{
    let player_name = "Player".to_string();
    let player_color = generate_random_color();
    let player_spawn_localtion = generate_random_position_within_radius(MAP_RADIUS);
    let player_size = Vec3::new(PLAYER_DEFAULT_RADIUS, PLAYER_DEFAULT_RADIUS, Z_PLAYER_SEGMENTS);

    commands.spawn((
        Player::new(player_name.clone(), player_color),
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(1.0)).into(),
            material: materials.add(ColorMaterial::from(player_color)),
            transform: Transform {
                scale: player_size, // Scale to initial radius
                translation: player_spawn_localtion.extend(Z_PLAYER_SEGMENTS),
                ..default()
            },
            ..default()
        },
        SegmentPositionHistory::default(),
    ));

    for i in 0..PLAYER_DEFAULT_LENGTH {
        commands.spawn((
            Segment {
                index: i,
                radius: PLAYER_DEFAULT_RADIUS,
            },
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle::new(1.0)).into(),
                material: materials.add(player_color),
                transform: Transform {
                    translation: Vec3::new(-(i as f32) * SEGMENT_SPACING, 0.0, Z_PLAYER_SEGMENTS),
                    scale: player_size,
                    ..default()
                },
                ..default()
            },
        ));
    }

    global_game_state.total_snakes += 1;
}

pub fn move_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut SegmentPositionHistory, &mut Player), With<Player>>,
    mut segment_query: Query<(&mut Transform, &Segment), Without<Player>>,
)
{
    for (mut player_transform, mut segment_history, mut player) in player_query.iter_mut() {
        let mut direction = Vec3::ZERO;
        let mut speed = PLAYER_SPEED;
        let delta_seconds = time.delta_seconds();

        if keyboard_input.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }

        let mut is_boosting = false;

        if keyboard_input.pressed(KeyCode::Space) && player.score >= SCORE_NEEDED_FOR_BOOSTING {
            is_boosting = true;
        }

        if is_boosting {
            speed *= 2.0;

            // Accumulate time for score deduction
            player.boost_timer += delta_seconds;

            // Deduct score every half second of boosting
            if player.boost_timer >= 0.5 {
                let score_deduction = player.boost_timer.floor() as u32;
                player.score = player.score.saturating_sub(score_deduction);
                player.boost_timer -= score_deduction as f32;

                // Remove segments based on the score deduction
                remove_segment(&mut commands, &mut player, score_deduction);
            }

            // Accumulate time for orb spawning
            player.orb_spawn_timer += delta_seconds;

            // Spawn orbs at intervals during boosting
            if player.orb_spawn_timer >= ORB_SPAWN_INTERVAL {
                if direction != Vec3::ZERO {
                    direction = direction.normalize();
                } else {
                    // Use previous movement direction
                    direction = segment_history
                        .positions
                        .get(1)
                        .map_or(Vec3::ZERO, |prev_pos| (player_transform.translation - *prev_pos).normalize());
                }

                // Calculate the minimum safe distance to avoid immediate collection
                let collection_threshold = player.radius + BOOST_ORB_RADIUS;
                let orb_position =
                    player_transform.translation - direction * (collection_threshold + ORB_SPAWN_DISTANCE_MARGIN);

                spawn_singlular_orb(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    player.color,
                    orb_position.truncate(),
                    BOOST_ORB_RADIUS,
                );

                player.orb_spawn_timer -= ORB_SPAWN_INTERVAL;
            }
        } else {
            // Reset timers when not boosting
            player.boost_timer = 0.0;
            player.orb_spawn_timer = 0.0;
        }

        // Movement and world boundary checks
        if direction != Vec3::ZERO {
            direction = direction.normalize();
            let new_translation = player_transform.translation + direction * speed * delta_seconds;

            // Boundary check
            let distance_from_center = new_translation.truncate().length();
            if distance_from_center + player.radius <= MAP_RADIUS {
                player_transform.translation = new_translation;
            } else {
                // Clamp position
                let clamped_position = new_translation.truncate().normalize() * (MAP_RADIUS - player.radius);
                player_transform.translation = clamped_position.extend(player_transform.translation.z);
            }

            // Record position
            segment_history.positions.push_front(player_transform.translation);

            // If player has exceeded the segment history limit, remove the oldest position
            if segment_history.positions.len() > MAX_SEGMENT_HISTORY {
                segment_history.positions.pop_back();
            }
        }

        // Record player's position
        segment_history.positions.push_front(player_transform.translation);
        if segment_history.positions.len() > MAX_SEGMENT_HISTORY as usize {
            segment_history.positions.pop_back();
        }

        let new_radius = calculate_player_radius(player.score);

        // Update the player's radius and scale if it has changed
        if (new_radius - player.radius).abs() > f32::EPSILON {
            player.radius = new_radius;
            player_transform.scale = Vec3::new(player.radius, player.radius, Z_PLAYER_SEGMENTS);
        }

        // Update the player's segments
        for (mut segment_transform, segment) in segment_query.iter_mut() {
            let index = (segment.index + 1) * POSITIONS_PER_SEGMENT;
            if index < segment_history.positions.len().try_into().unwrap() {
                segment_transform.translation = segment_history.positions[index as usize];
                // Ensure the segment's scale matches the player's radius
                segment_transform.scale = Vec3::new(player.radius, player.radius, Z_PLAYER_SEGMENTS);
            } else {
                segment_transform.translation = player_transform.translation;
            }
        }
    }
}

pub fn collect_orb(
    mut commands: Commands,
    mut player_query: Query<(&mut Transform, &mut Player), Without<Orb>>,
    orb_query: Query<(Entity, &Transform, &Orb), Without<Player>>,
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

                commands.entity(orb_entity).despawn();
                player.score += SCORE_PER_ORB;

                let segment_entity = add_segment(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    player_transform.translation,
                    player.color,
                    player.radius,
                    player.segment_count,
                    false
                );

                player.segments.push_back(segment_entity);
                player.segment_count += 1;
            }
        }
    }
}

pub fn add_segment(
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

pub fn remove_segment(commands: &mut Commands, player: &mut Player, segments_to_remove: u32)
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

/// Updates the player's camera to follow the player in the world
/// todo - make the player camera zoom start small and scale with the player's radius in the future
pub fn update_player_camera(
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
)
{
    let Ok(mut camera) = camera_query.get_single_mut() else {
        return;
    };

    let Ok(player) = player_query.get_single() else {
        return;
    };

    let target = Vec3::new(player.translation.x, player.translation.y, camera.translation.z);

    camera.translation = camera.translation.lerp(target, time.delta_seconds() * CAM_LERP_FACTOR);
}

pub fn spawn_score_text(mut commands: Commands, asset_server: Res<AssetServer>)
{
    commands.spawn((
        ScoreText,
        TextBundle::from_section(
            "Score: 0",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 30.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            bottom: Val::Px(10.0),
            ..default()
        }),
    ));
}

pub fn update_score_text(mut player_query: Query<&Player>, mut text_query: Query<&mut Text, With<ScoreText>>)
{
    if let Ok(mut player) = player_query.get_single_mut() {
        if let Ok(mut text) = text_query.get_single_mut() {
            text.sections[0].value = format!("Score: {}", player.score);
        }
    }
}

pub fn calculate_player_radius(score: u32) -> f32
{
    let stages = score / SCORE_PER_RADIUS_STAGE;
    MIN_PLAYER_RADIUS + stages as f32 * RADIUS_GROWTH_PER_STAGE
}
