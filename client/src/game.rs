use bevy::prelude::*;

use crate::{constants::*, shared::*};

pub struct GamePlugin;

impl Plugin for GamePlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_systems(OnEnter(GameState::Game), game_setup);
        app.add_systems(Update, (game_loop.run_if(in_state(GameState::Game)), move_player.run_if(in_state(GameState::Game))));
        app.add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
    }
}

// Tag component used to tag entities added on the game screen
#[derive(Component)]
struct OnGameScreen;

fn game_setup(mut commands: Commands) {

}

fn game_loop(time: Res<Time>, mut game_state: ResMut<NextState<GameState>>) {}

/// Update the player position based on the cursor movement
fn move_player(mut player: Query<&mut Transform, With<Player>>, windows: Query<&mut Window>, mut cursor_moved_events: EventReader<CursorMoved>, time: Res<Time>)
{
    let Ok(mut player) = player.get_single_mut() else {
        return;
    };

    // Get the primary window
    let window = windows.single();

    // Get the most recent cursor movement event
    let Some(cursor_moved) = cursor_moved_events.read().last() else {
        return;
    };

    // Convert cursor position to world coordinates
    // First, get the cursor position within the window
    let cursor_position = Vec2::new(
        cursor_moved.position.x - window.width() / 2.0, 
        cursor_moved.position.y - window.height() / 2.0,
    );

    // Get the player's current position in 2D space
    let player_position = Vec2::new(player.translation.x, player.translation.y);

    // Calculate the direction vector from the player to the cursor
    let direction = (cursor_position - player_position).normalize_or_zero();

    // Smooth movement: you can use an interpolation factor like in your camera code
    let move_delta = direction * PLAYER_SPEED * time.delta_seconds();

    // Update player position smoothly
    player.translation = player.translation.lerp(
        (player_position + move_delta).extend(player.translation.z),
        1.,  // This factor controls how smoothly the player moves; smaller values make the movement more gradual
    );
}