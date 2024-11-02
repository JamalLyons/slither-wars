use bevy::prelude::Component;

/// An orb that the player can collect to increase their length
#[derive(Component, Clone, Debug)]
pub struct Orb
{
    pub radius: f32,
}