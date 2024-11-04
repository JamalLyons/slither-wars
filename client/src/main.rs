#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_variables)]

mod constants;
mod resources;
mod systems;
mod utils;

mod orb;
mod player;

use std::time::Duration;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy::winit::WinitSettings;
use bevy_dev_tools::fps_overlay::FpsOverlayPlugin;
use resources::GlobalGameState;

use crate::constants::*;
use crate::systems::*;

fn main()
{
    App::new()
        // Custom window settings
        .insert_resource(WinitSettings {
            // When the main window is focused, I want to run the game at max speeds.
            // later when I add a working menu, I will make it so this is configued
            // for optimal performance when not in game.
            // see - https://github.com/bevyengine/bevy/blob/release-0.14.2/examples/window/low_power.rs
            focused_mode: bevy::winit::UpdateMode::Continuous,
            // When the window is not focused, the game will run at lower fps.
            unfocused_mode: bevy::winit::UpdateMode::reactive_low_power(Duration::from_secs(10)),
        })
        .insert_resource(GlobalGameState::default())
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: WINDOW_TITLE.into(),
                    name: Some(WINDOW_NAME.into()),
                    resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                    present_mode: PresentMode::AutoVsync,
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    visible: false, // Make this visible after later
                    ..default()
                }),
                ..default()
            }),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
        ))
        .add_plugins(FpsOverlayPlugin::default())
        .add_plugins(player::PlayerPlugin)
        .add_plugins(orb::OrbPlugin)
        .add_systems(Startup, (spawn_camera, spawn_game_world))
        .add_systems(Update, make_window_visible)
        .run();
}
