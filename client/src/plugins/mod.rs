///! Author: CodingWithJamal
///! Date: 10/20/24
///!
///! Description:
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

pub mod camera;
pub mod constants;
pub mod fps;

#[derive(Component)]
pub struct Player;

pub fn setup_scene(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>)
{
    // World where we move the player
    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Rectangle::new(1000., 700.))),
        material: materials.add(Color::srgb(0.2, 0.2, 0.3)),
        ..default()
    });

    // Player
    commands.spawn((
        Player,
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
