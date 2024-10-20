mod plugins;

use bevy::math::vec3;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

use crate::plugins::Player;

fn main()
{
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(plugins::camera::CameraPlugin)
        .add_plugins(plugins::fps::FpsPlugin)
        .add_systems(Startup, (setup_scene, setup_instructions))
        .run();
}

fn setup_scene(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>)
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

fn setup_instructions(mut commands: Commands)
{
    commands.spawn(
        TextBundle::from_section(
            "Move the light with WASD.\nThe camera will smoothly track the light.",
            TextStyle::default(),
        )
            .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(12.0),
                left: Val::Px(12.0),
                ..default()
            }),
    );
}
