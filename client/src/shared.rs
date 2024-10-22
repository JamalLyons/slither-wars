use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameMenuState
{
    #[default]
    Splash,
    Menu,
    Game,
}

#[derive(Component)]
pub struct Player;
