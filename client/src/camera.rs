use bevy::core_pipeline::bloom::BloomSettings;
use bevy::input::mouse::*;
use bevy::prelude::*;

use crate::constants::*;
use crate::shared::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_systems(Startup, setup);
        app.add_systems(Update, (print_mouse_events_system, move_player, update).chain());
    }
}

fn setup(mut commands: Commands)
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

/// Update the camera position by tracking the player.
fn update(
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

/// Update the player position with keyboard inputs.
fn move_player(mut player: Query<&mut Transform, With<Player>>, time: Res<Time>, kb_input: Res<ButtonInput<KeyCode>>)
{
    let Ok(mut player) = player.get_single_mut() else {
        return;
    };

    let mut direction = Vec2::ZERO;

    if kb_input.pressed(KeyCode::KeyW) {
        direction.y += 1.;
    }

    if kb_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.;
    }

    if kb_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.;
    }

    if kb_input.pressed(KeyCode::KeyD) {
        direction.x += 1.;
    }

    // Progressively update the player's position over time. Normalize the
    // direction vector to prevent it from exceeding a magnitude of 1 when
    // moving diagonally.
    let move_delta = direction.normalize_or_zero() * PLAYER_SPEED * time.delta_seconds();
    player.translation += move_delta.extend(0.);
}

/// This system prints out all mouse events as they come in
fn print_mouse_events_system(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut cursor_moved_events: EventReader<CursorMoved>,
) {
    // We will use this to control player boost speed when the left mouse button is pressed
    for event in mouse_button_input_events.read() {
        info!("{:?}", event);
    }

    // We will use this to control player movement when the cursor moves a direction
    for event in cursor_moved_events.read() {
        info!("{:?}", event);
    }
}