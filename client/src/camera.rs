use bevy::core_pipeline::bloom::BloomSettings;
use bevy::prelude::*;

use crate::constants::*;
use crate::shared::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_systems(Startup, setup);
        app.add_systems(
            Update,
            (
                update.run_if(in_state(GameState::Game)),
            )
                .chain(),
        );
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