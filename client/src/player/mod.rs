mod components;
mod systems;

use bevy::prelude::*;
use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_systems(Startup, (spawn_score_text, spawn_player));
        app.add_systems(Update, (move_player, update_player_camera));
    }
}
