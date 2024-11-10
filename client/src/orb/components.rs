use bevy::prelude::*;

#[derive(Component, Clone, Debug)]
pub struct Orb
{
    pub value: u32,
    pub radius: f32,
}
