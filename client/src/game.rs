use bevy::math::vec3;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

use crate::constants::*;
use crate::shared::*;

pub struct GamePlugin;

impl Plugin for GamePlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_systems(OnEnter(GameState::Game), game_setup);
        app.add_systems(
            Update,
            (game_loop, move_player, update_camera)
                .chain()
                .run_if(in_state(GameState::Game)),
        );
        // Cleans up game entities. This is useful so when the player starts a new game, the state is reset.
        app.add_systems(OnExit(GameState::Game), (despawn_screen::<Player>, despawn_screen::<Segment>, despawn_screen::<Orb>));
    }
}

pub fn game_setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>)
{
    // World where we move the  player
    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Rectangle::new(SCREEN_WIDTH, SCREEN_HEIGHT))),
        material: materials.add(Color::srgb(0.2, 0.2, 0.3)),
        ..default()
    });

    // Player
    commands.spawn((
        // todo allow input of player name in ui later
        Player::new("Player".to_string()),
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(25.)).into(),
            material: materials.add(Color::srgb(6.25, 9.4, 9.1)), // RGB values exceed 1 to achieve a bright color for the bloom effect
            transform: Transform {
                translation: vec3(0., 0., 2.),
                ..default()
            },
            ..default()
        },
    ));
}

fn game_loop(time: Res<Time>, mut game_state: ResMut<NextState<GameState>>, keyboard_input: Res<ButtonInput<KeyCode>>)
{
    if keyboard_input.just_pressed(KeyCode::Escape) {
        game_state.set(GameState::Menu);
    }
}

/// Update the camera position by tracking the player.
fn update_camera(
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
)
{
    let Ok(mut camera) = camera.get_single_mut() else {
        return;
    };

    let Ok(player) = player.get_single() else {
        return;
    };

    let Vec3 { x, y, .. } = player.translation;
    let direction = Vec3::new(x, y, camera.translation.z);

    // Applies a smooth effect to camera movement using interpolation between
    // the camera position and the player position on the x and y axes.
    // Here we use the in-game time, to get the elapsed time (in seconds)
    // since the previous update. This avoids jittery movement when tracking
    // the player.
    camera.translation = camera.translation.lerp(direction, time.delta_seconds() * CAM_LERP_FACTOR);
}

/// Update the player position based on the cursor movement
/// TODO - fix this. It does not really work lol.
/// The player moves opposite to the cursor, and movement is not smooth for some reason?
fn move_player(
    mut player: Query<&mut Transform, With<Player>>,
    windows: Query<&mut Window>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    time: Res<Time>,
)
{
    let Ok(mut player) = player.get_single_mut() else {
        return;
    };

    // Get the primary window
    let window = windows.single();

    // Get the most recent cursor movement event
    let Some(cursor_moved) = cursor_moved_events.read().last() else {
        return;
    };

    // Convert cursor position to world coordinates
    // First, get the cursor position within the window
    let cursor_position = Vec2::new(
        cursor_moved.position.x - window.width() / 2.0,
        cursor_moved.position.y - window.height() / 2.0,
    );

    // Get the player's current position in 2D space
    let player_position = Vec2::new(player.translation.x, player.translation.y);

    // Calculate the direction vector from the player to the cursor
    let direction = (cursor_position - player_position).normalize_or_zero();

    // Smooth movement: you can use an interpolation factor like in your camera code
    let move_delta = direction * PLAYER_SPEED * time.delta_seconds();

    // Update player position smoothly
    player.translation = player.translation.lerp(
        (player_position + move_delta).extend(player.translation.z),
        1., // This factor controls how smoothly the player moves; smaller values make the movement more gradual
    );
}
