pub mod components;
pub mod systems;

use bevy::prelude::*;
pub use systems::*;

pub struct BotPlugin;

impl Plugin for BotPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_systems(Startup, spawn_bots)
           .add_systems(Update, (bot_movement, bot_eating));
    }
}
