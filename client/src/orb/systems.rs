use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use super::components::Orb;
use crate::constants::*;
use crate::core::resources::GlobalGameState;
use crate::utils::*;

pub fn spawn_orbs(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    global_game_state: Res<GlobalGameState>,
    existing_orbs: Query<Entity, With<Orb>>,
)
{
    let desired_orb_count = calculate_desired_orb_count(&global_game_state);

    let current_orb_count = existing_orbs.iter().count();

    if current_orb_count < desired_orb_count {
        let orbs_to_spawn = desired_orb_count - current_orb_count;

        for _ in 0..orbs_to_spawn {
            let position = generate_random_position_within_radius(MAP_RADIUS);
            let color = generate_random_color();

            spawn_singlular_orb(
                &mut commands,
                &mut meshes,
                &mut materials,
                color,
                position,
                ORB_RADIUS,
                ORB_VALUE,
            );
        }
    }
}

pub fn spawn_singlular_orb(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    color: Color,
    position: Vec2,
    radius: f32,
    value: u32,
) -> Entity
{
    commands
        .spawn((
            Orb { radius, value },
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle::new(ORB_RADIUS)).into(), // Orbs have a radius of 5.0
                material: materials.add(ColorMaterial::from(color)),
                transform: Transform {
                    translation: position.extend(Z_ORBS),
                    ..default()
                },
                ..default()
            },
        ))
        .id()
}

/// Calculates the desired number of orbs to spawn based on the number of snakes and total orbs in the game
fn calculate_desired_orb_count(global_game_state: &GlobalGameState) -> usize
{
    // Base orbs per player
    let base_orbs = ORB_SPAWN_PER_PLAYER * global_game_state.total_snakes;

    // Cap or adjust total orbs based on map size or other factors if needed
    base_orbs.min(global_game_state.total_orbs)
}
