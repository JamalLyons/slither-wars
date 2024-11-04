use bevy::prelude::*;
use systems::*;

pub mod components;
pub mod systems;

pub struct OrbPlugin;

impl Plugin for OrbPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_systems(Update, spawn_orbs);
    }
}
