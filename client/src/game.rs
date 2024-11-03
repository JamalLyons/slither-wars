use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::bot::{bot_setup, move_bots, Bot};
use crate::constants::*;
use crate::enums::GameState;
use crate::orb;
use crate::orb::{orb_collection_system, orb_setup, Orb};
use crate::player::{move_player, setup_player, Player};
use crate::segments::{PositionHistory, Segment};
use crate::settings::GameSettings;
use crate::utils::despawn_screen;

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
        app.add_systems(OnEnter(GameState::Game), orb_setup);
        app.add_systems(
            Update,
            (
                collision_system,
                move_bots,
                move_player,
                orb_collection_system,
                update_camera,
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
                despawn_screen::<Bot>,
                despawn_screen::<Orb>,
                despawn_screen::<Segment>,
                despawn_screen::<PositionHistory>,
                despawn_screen::<ScoreText>,
            ),
        );
    }
}

pub fn game_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_settings: Res<GameSettings>,
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
                translation: Vec3::new(0.0, 0.0, Z_BACKGROUND), // Set z to -1.0
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

    // todo - see why spawning bots makes our player look glichy af ???
    bot_setup(&mut commands, &mut meshes, &mut materials, &game_settings);
    setup_player(&mut commands, &mut meshes, &mut materials);
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

pub fn update_score_text(mut player_query: Query<&Player>, mut text_query: Query<&mut Text, With<ScoreText>>)
{
    if let Ok(mut player) = player_query.get_single_mut() {
        if let Ok(mut text) = text_query.get_single_mut() {
            text.sections[0].value = format!("Score: {}", player.score);
        }
    }
}

pub fn collision_system(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform, &mut Player)>,
    bot_query: Query<(Entity, &Transform, &Bot)>,
    mut game_state: ResMut<NextState<GameState>>,
)
{
    if let Ok((player_entity, player_transform, mut player)) = player_query.get_single_mut() {
        for (bot_entity, bot_transform, bot) in bot_query.iter() {
            let distance = player_transform
                .translation
                .truncate()
                .distance(bot_transform.translation.truncate());

            if distance < player.radius + bot.radius {
                println!("Player collided with bot {}!", bot.name);

                // Despawn the player and their segments
                // todo drop food on player death
                commands.entity(player_entity).despawn_recursive();
                for segment_entity in player.segments.iter() {
                    commands.entity(*segment_entity).despawn_recursive();
                }

                // Transition to Game Over state or handle accordingly
                // game_state.set(GameState::Menu);

                break;
            }
        }
    }
}
