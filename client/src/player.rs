use std::collections::VecDeque;

use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::constants::*;
use crate::enums::GameState;
use crate::orb::spawn_orb;
use crate::segments::{remove_segments, PositionHistory, Segment};
use crate::utils::{generate_random_color, generate_random_position_within_radius};

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
    pub segment_count: u32,
    pub segments: VecDeque<Entity>,
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
            segment_count: 0,
            segments: VecDeque::with_capacity(MAX_GROWTH_LIMIT as usize),
        }
    }
}

pub fn setup_player(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
)
{
    // Spawn the Player entity
    let player_color = generate_random_color();
    let player_spawn_localtion = generate_random_position_within_radius(MAP_RADIUS);
    let player_size = Vec3::new(PLAYER_DEFAULT_RADIUS, PLAYER_DEFAULT_RADIUS, 1.0);

    commands.spawn((
        Player::new("Player".to_string(), player_color),
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(1.0)).into(),
            material: materials.add(ColorMaterial::from(player_color)),
            transform: Transform {
                scale: player_size, // Scale to initial radius
                translation: player_spawn_localtion.extend(0.0),
                ..default()
            },
            ..default()
        },
        PositionHistory::default(),
    ));

    // Spawn segments and attach them to the player
    for i in 0..PLAYER_DEFAULT_LENGTH {
        commands.spawn((
            Segment {
                radius: PLAYER_DEFAULT_RADIUS,
                index: i,
            },
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle::new(1.0)).into(),
                material: materials.add(player_color),
                transform: Transform {
                    translation: Vec3::new(-(i as f32) * SEGMENT_SPACING, 0.0, 0.0),
                    scale: player_size,
                    ..default()
                },
                ..default()
            },
        ));
    }
}

pub fn move_player(
    mut commands: Commands,
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut player_query: Query<(&mut Transform, &mut PositionHistory, &mut Player), With<Player>>,
    mut segment_query: Query<(&mut Transform, &Segment), Without<Player>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
)
{
    // End game if escape is pressed
    if keyboard_input.just_pressed(KeyCode::Escape) {
        game_state.set(GameState::Menu);
    }

    for (mut player_transform, mut history, mut player) in player_query.iter_mut() {
        let mut direction = Vec3::ZERO;
        let mut speed = PLAYER_SPEED;

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

        let delta_seconds = time.delta_seconds();

        let mut is_boosting = false;

        if keyboard_input.pressed(KeyCode::Space) && player.score >= SCORE_NEEDED_FOR_BOOSTING {
            is_boosting = true;
        }

        // Stop boosting if score falls below threshold
        if player.score < SCORE_NEEDED_FOR_BOOSTING {
            is_boosting = false;
            speed = PLAYER_SPEED;
        }

        if is_boosting {
            speed *= 2.0;

            // Accumulate time for score deduction
            player.boost_timer += delta_seconds;

            // Deduct score every 1 second of boosting
            if player.boost_timer >= 1.0 {
                let score_deduction = player.boost_timer.floor() as u32;
                player.score = player.score.saturating_sub(score_deduction);
                player.boost_timer -= score_deduction as f32;

                // Remove segments based on the score deduction
                remove_segments(&mut commands, &mut player, score_deduction);
            }

            // Accumulate time for orb spawning
            player.orb_spawn_timer += delta_seconds;

            // Spawn orbs at intervals during boosting
            if player.orb_spawn_timer >= ORB_SPAWN_INTERVAL {
                if direction != Vec3::ZERO {
                    direction = direction.normalize();
                } else {
                    // Use previous movement direction
                    direction = history
                        .positions
                        .get(1)
                        .map_or(Vec3::ZERO, |prev_pos| (player_transform.translation - *prev_pos).normalize());
                }

                // Calculate the minimum safe distance to avoid immediate collection
                let collection_threshold = player.radius + BOOST_ORB_RADIUS;
                let orb_position =
                    player_transform.translation - direction * (collection_threshold + ORB_SPAWN_DISTANCE_MARGIN);

                spawn_orb(
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
            history.positions.push_front(player_transform.translation);
            if history.positions.len() > MAX_SEGMENT_HISTORY as usize {
                history.positions.pop_back();
            }
        }

        // Record player's position
        history.positions.push_front(player_transform.translation);
        if history.positions.len() > MAX_SEGMENT_HISTORY as usize {
            history.positions.pop_back();
        }

        let new_radius = calculate_player_radius(player.score);

        // Update the player's radius and scale if it has changed
        if (new_radius - player.radius).abs() > f32::EPSILON {
            player.radius = new_radius;
            player_transform.scale = Vec3::new(player.radius, player.radius, 1.0);
        }

        // Update segments
        for (mut segment_transform, segment) in segment_query.iter_mut() {
            let index = (segment.index + 1) * POSITIONS_PER_SEGMENT;
            if index < history.positions.len().try_into().unwrap() {
                segment_transform.translation = history.positions[index as usize];
                // Ensure the segment's scale matches the player's radius
                segment_transform.scale = Vec3::new(player.radius, player.radius, 1.0);
            } else {
                segment_transform.translation = player_transform.translation;
            }
        }
    }
}

pub fn calculate_player_radius(score: u32) -> f32
{
    let stages = score / SCORE_PER_RADIUS_STAGE;
    MIN_PLAYER_RADIUS + stages as f32 * RADIUS_GROWTH_PER_STAGE
}
