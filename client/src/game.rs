use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::constants::*;
use crate::shared::*;

pub struct GamePlugin;

impl Plugin for GamePlugin
{
    fn build(&self, app: &mut App)
    {
        app.insert_resource(GameSettings::default());
        app.add_systems(OnEnter(GameState::Game), game_setup);
        app.add_systems(
            Update,
            (move_player, update_camera, spawn_orbs_system, orb_collection_system)
                .chain()
                .run_if(in_state(GameState::Game)),
        );
        // Cleans up game entities. This is useful so when the player starts a new game, the state is reset.
        app.add_systems(
            OnExit(GameState::Game),
            (despawn_screen::<Player>, despawn_screen::<Segment>, despawn_screen::<Orb>),
        );
    }
}

pub fn game_setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>)
{
    commands.spawn((
        Name::new("Map Boundary"),
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(MAP_RADIUS)).into(),
            material: materials.add(Color::srgb(0.1, 0.1, 0.1)),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, -1.0), // Set z to -1.0
                ..default()
            },
            ..default()
        },
    ));

    // Spawn the Player entity
    let player_entity = commands
        .spawn((
            Player::new("Player".to_string(), generate_random_color()),
            SpatialBundle::default(),
            PositionHistory::default(),
        ))
        .id();

    // Spawn segments and attach them to the player
    for i in 0..PLAYER_DEFAULT_LENGTH {
        let segment_entity = commands
            .spawn((
                Segment {
                    radius: PLAYER_DEFAULT_RADIUS,
                },
                MaterialMesh2dBundle {
                    mesh: meshes.add(Circle::new(12.5)).into(),
                    material: materials.add(Color::srgb(0.0, 1.0, 0.0)),
                    transform: Transform {
                        translation: Vec3::new(-(i as f32) * SEGMENT_SPACING, 0.0, 0.0),
                        ..default()
                    },
                    ..default()
                },
            ))
            .id();
    }
}

pub fn move_player(
    mut commands: Commands,
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut player_query: Query<(&mut Transform, &mut PositionHistory, &mut Player), With<Player>>,
    mut segment_query: Query<&mut Transform, (With<Segment>, Without<Player>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
)
{
    // End game if escape is pressed
    if keyboard_input.just_pressed(KeyCode::Escape) {
        game_state.set(GameState::Menu);
    }

    let delta_seconds = time.delta_seconds();

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
            speed = PLAYER_SPEED; // Reset speed
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
            }

            // Accumulate time for orb spawning
            player.orb_spawn_timer += delta_seconds;

            // Spawn orbs at intervals during boosting
            if player.orb_spawn_timer >= PLAYER_ORB_SPAWN_INTERVAL {
                if direction != Vec3::ZERO {
                    direction = direction.normalize();
                } else {
                    // Use previous movement direction
                    direction = history
                        .positions
                        .get(1)
                        .map_or(Vec3::ZERO, |prev_pos| (player_transform.translation - *prev_pos).normalize());
                }

                let orb_position = player_transform.translation - direction * (player.radius + 4.0 + 1.0);

                spawn_orb(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    player.color,
                    orb_position.truncate(),
                );

                player.orb_spawn_timer -= PLAYER_ORB_SPAWN_INTERVAL;
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
            if history.positions.len() > MAX_SEGMENT_HISTORY {
                history.positions.pop_back();
            }
        }

        // Record player's position
        history.positions.push_front(player_transform.translation);
        if history.positions.len() > MAX_SEGMENT_HISTORY {
            history.positions.pop_back();
        }

        // Update segments
        for (i, mut segment_transform) in segment_query.iter_mut().enumerate() {
            let index = (i + 1) * POSITIONS_PER_SEGMENT;
            if index < history.positions.len() {
                segment_transform.translation = history.positions[index];
            } else {
                // If not enough history, set position to player's current position
                segment_transform.translation = player_transform.translation;
            }
        }
    }
}

/// Update the camera position by tracking the player.
fn update_camera(
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

fn spawn_orbs_system(
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

            spawn_orb(&mut commands, &mut meshes, &mut materials, color, position);
        }
    }
}

/// Util function to spawn an orb
fn spawn_orb(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    color: Color,
    position: Vec2,
) -> Entity
{
    commands
        .spawn((
            Orb,
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle::new(5.0)).into(), // Orbs have a radius of 5.0
                material: materials.add(ColorMaterial::from(color)),
                transform: Transform {
                    translation: position.extend(0.0), // z = 0.0
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
    let base_orbs = 20 * game_settings.total_players;

    // Cap or adjust total orbs based on map size or other factors if needed
    base_orbs.min(game_settings.total_orbs)
}

fn orb_collection_system(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut Player)>,
    orb_query: Query<(Entity, &Transform), With<Orb>>,
)
{
    for (player_transform, mut player) in player_query.iter_mut() {
        for (orb_entity, orb_transform) in orb_query.iter() {
            let distance = player_transform
                .translation
                .truncate()
                .distance(orb_transform.translation.truncate());

            if distance < player.radius + 5.0 {
                // Collect the orb
                commands.entity(orb_entity).despawn();
                player.score += SCORE_PER_ORB;

                // Optionally, grow the player or add segments
                // ...
            }
        }
    }
}
