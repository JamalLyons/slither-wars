use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::constants::*;
use crate::enums::GameState;
use crate::orb::{orb_collection_system, spawn_orbs_system, Orb};
use crate::player::{move_player, Player};
use crate::segments::{PositionHistory, Segment};
use crate::utils::{despawn_screen, generate_random_color, generate_random_position_within_radius};

#[derive(Component)]
struct GameWorld;

#[derive(Component)]
pub struct ScoreText;

pub struct GamePlugin;

impl Plugin for GamePlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_systems(OnEnter(GameState::Game), game_setup);
        app.add_systems(
            Update,
            (
                move_player,
                update_camera,
                spawn_orbs_system,
                orb_collection_system,
                update_score_text,
            )
                .chain()
                .run_if(in_state(GameState::Game)),
        );
        // Cleans up game entities. This is useful so when the player starts a new game, the state is reset.
        app.add_systems(
            OnExit(GameState::Game),
            (
                despawn_screen::<GameWorld>,
                despawn_screen::<Player>,
                despawn_screen::<Orb>,
                despawn_screen::<Segment>,
                despawn_screen::<PositionHistory>,
                despawn_screen::<ScoreText>,
                despawn_screen::<Text>,
            ),
        );
    }
}

pub fn game_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
)
{
    // Spawn the map boundary
    commands.spawn((
        GameWorld,
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

    // Spawn the score text
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

pub fn update_score_text(player_query: Query<&Player>, mut text_query: Query<&mut Text, With<ScoreText>>)
{
    let player = player_query.single();
    let mut text = text_query.single_mut();

    text.sections[0].value = format!("Score: {}", player.score);
}
