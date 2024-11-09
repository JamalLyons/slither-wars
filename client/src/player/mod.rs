pub mod components;
pub mod systems;

use bevy::prelude::*;
use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_systems(Startup, (spawn_score_text, spawn_player).chain())
            .add_systems(Update, (move_player, collect_orb, update_player_camera, update_score_text));
    }
}
