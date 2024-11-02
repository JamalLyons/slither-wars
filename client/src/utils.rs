use bevy::color::Color;
use bevy::math::Vec2;
use bevy::prelude::{Commands, Component, DespawnRecursiveExt, Entity, Query, With};
use rand::Rng;

/// Despawn's all entities with the given component
pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands)
{
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn generate_random_position_within_radius(radius: f32) -> Vec2
{
    let mut rng = rand::thread_rng();
    let angle = rng.gen_range(0.0..std::f32::consts::TAU);
    let distance = rng.gen_range(0.0..radius);

    Vec2::new(distance * angle.cos(), distance * angle.sin())
}

pub fn generate_random_color() -> Color
{
    let colors = vec![
        Color::srgb(1.0, 0.0, 0.0),  // Red
        Color::srgb(0.0, 1.0, 0.0),  // Green
        Color::srgb(0.0, 0.0, 1.0),  // Blue
        Color::srgb(1.0, 1.0, 0.0),  // Yellow
        Color::srgb(1.0, 0.65, 0.0), // Orange
        Color::srgb(0.5, 0.0, 0.5),  // Purple
        Color::srgb(0.0, 1.0, 1.0),  // Cyan
        Color::srgb(1.0, 0.75, 0.8), // Pink
    ];

    let mut rng = rand::thread_rng();
    colors[rng.gen_range(0..colors.len())]
}
