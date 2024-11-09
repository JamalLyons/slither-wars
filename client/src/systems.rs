use bevy::core::FrameCount;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use components::GameWorld;

use crate::*;

pub fn spawn_game_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
)
{
    commands.spawn((
        GameWorld,
        Name::new("Map Boundary"),
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(MAP_RADIUS)).into(),
            material: materials.add(Color::srgb(0.1, 0.1, 0.1)),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, Z_BACKGROUND),
                ..default()
            },
            ..default()
        },
    ));
}

pub fn spawn_camera(mut commands: Commands)
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

/// We use this to avoid the white window that shows up before the GPU is ready to render the app.
/// This happens so fast the the user will not see it.
pub fn make_window_visible(mut window: Query<&mut Window>, frames: Res<FrameCount>)
{
    if frames.0 == 3 {
        // At this point the gpu is ready to show the app so we can make the window visible.
        window.single_mut().visible = true;
    }
}

fn spawn_death_orbs(commands: &mut Commands, dead_entity: Entity, snake_length: u32, killer_entity: Entity, was_player: bool)
{
    // TODO: Implement orb spawning on death
    // This could include:
    // 1. Converting snake length into a number of orbs
    // 2. Spawning those orbs in a pattern around the death location
    // 3. Potentially giving bonus points/orbs to the killer
}
