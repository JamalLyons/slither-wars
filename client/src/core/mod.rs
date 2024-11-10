pub mod components;
pub mod resources;
pub mod systems;

use bevy::prelude::*;

use systems::*;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<resources::GlobalGameState>()
           .add_systems(Startup, (spawn_camera, spawn_game_world))
           .add_systems(Update, (
               make_window_visible,
               check_snake_collisions,
               cleanup_dead_snakes,
               orb_collection,
               update_segment_sizes,
           ));
    }
} 