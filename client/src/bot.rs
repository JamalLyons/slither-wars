use std::collections::VecDeque;

use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::constants::*;
use crate::player::Player;
use crate::segments::{PositionHistory, Segment};
use crate::settings::GameSettings;
use crate::utils::{calculate_player_radius, generate_random_color, generate_random_position_within_radius};

#[derive(Component, Clone, Debug)]
pub struct Bot
{
    pub name: String,
    pub score: u32,
    pub length: u32,
    pub radius: f32,
    pub color: Color,
    pub direction: Vec3,
    pub speed: f32,
    pub segment_count: u32,
    pub segments: VecDeque<Entity>,
}

pub fn bot_setup(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    game_settings: &Res<GameSettings>,
)
{
    for i in 0..game_settings.bot_count {
        let bot_color = generate_random_color();
        let bot_radius = Vec3::new(PLAYER_DEFAULT_RADIUS, PLAYER_DEFAULT_RADIUS, 2.0);
        let bot_position = generate_random_position_within_radius(game_settings.map_radius);

        // Spawn the Bot entity
        let bot_entity = commands
            .spawn((
                Bot {
                    name: format!("Bot {}", i + 1),
                    score: 0,
                    length: PLAYER_DEFAULT_LENGTH,
                    radius: PLAYER_DEFAULT_RADIUS,
                    color: bot_color,
                    direction: Vec3::new(rand::random::<f32>() * 2.0 - 1.0, rand::random::<f32>() * 2.0 - 1.0, 0.0)
                        .normalize(),
                    speed: BOT_SPEED,
                    segment_count: PLAYER_DEFAULT_LENGTH,
                    segments: VecDeque::new(),
                },
                MaterialMesh2dBundle {
                    mesh: meshes.add(Circle::new(1.0)).into(),
                    material: materials.add(ColorMaterial::from(bot_color)),
                    transform: Transform {
                        translation: bot_position.extend(Z_BOT_HEAD),
                        scale: bot_radius,
                        ..default()
                    },
                    ..default()
                },
                PositionHistory::default(),
            ))
            .id();

        // Spawn bot segments
        for j in 0..PLAYER_DEFAULT_LENGTH {
            commands.spawn((
                Segment {
                    radius: PLAYER_DEFAULT_RADIUS,
                    index: j,
                },
                MaterialMesh2dBundle {
                    mesh: meshes.add(Circle::new(1.0)).into(),
                    material: materials.add(ColorMaterial::from(bot_color)),
                    transform: Transform {
                        translation: bot_position.extend(Z_BOT_SEGMENTS),
                        scale: bot_radius,
                        ..default()
                    },
                    ..default()
                },
            ));
        }
    }
}

pub fn move_bots(
    mut commands: Commands,
    time: Res<Time>,
    mut bot_query: Query<(&mut Transform, &mut PositionHistory, &mut Bot)>,
    mut segment_query: Query<(&mut Transform, &Segment), (Without<Player>, Without<Bot>)>,
)
{
    let delta_seconds = time.delta_seconds();

    for (mut bot_transform, mut history, mut bot) in bot_query.iter_mut() {
        let mut direction = bot.direction;

        // Randomly change direction
        if rand::random::<f32>() < BOT_DIRECTION_CHANGE_CHANCE {
            direction = Vec3::new(rand::random::<f32>() * 2.0 - 1.0, rand::random::<f32>() * 2.0 - 1.0, 0.0).normalize();
            bot.direction = direction;
        }

        let new_translation = bot_transform.translation + direction * bot.speed * delta_seconds;

        // Boundary check
        let distance_from_center = new_translation.truncate().length();
        if distance_from_center + bot.radius <= MAP_RADIUS {
            bot_transform.translation = new_translation;
        } else {
            // Reverse direction upon hitting boundary
            bot.direction = -bot.direction;
        }

        // Record position
        history.positions.push_front(bot_transform.translation);
        if history.positions.len() > MAX_SEGMENT_HISTORY as usize {
            history.positions.pop_back();
        }

        let new_radius = calculate_player_radius(bot.score);

        if (new_radius - bot.radius).abs() > f32::EPSILON {
            bot.radius = new_radius;
            bot_transform.scale = Vec3::new(bot.radius, bot.radius, 1.0);
        }

        // Update segments
        for (mut segment_transform, segment) in segment_query.iter_mut() {
            let index = (segment.index + 1) * POSITIONS_PER_SEGMENT;
            if index < history.positions.len().try_into().unwrap() {
                segment_transform.translation = history.positions[index as usize];
                // Ensure the segment's scale matches the player's radius
                segment_transform.scale = Vec3::new(bot.radius, bot.radius, 1.0);
            } else {
                segment_transform.translation = bot_transform.translation;
            }
        }
    }
}
