use bevy::prelude::{Resource, States};

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States, Resource)]
pub enum GameState
{
    #[default]
    Splash,
    Menu,
    Game,
}