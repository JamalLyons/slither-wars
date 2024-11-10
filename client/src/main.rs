#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_variables)]

mod constants;
mod utils;
mod core;

mod bot;
mod orb;
mod player;
mod leaderboard;

use std::time::Duration;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy::winit::WinitSettings;
use bevy_dev_tools::fps_overlay::FpsOverlayPlugin;

use crate::constants::*;
use crate::core::CorePlugin;

fn main() {
    App::new()
        .insert_resource(WinitSettings {
            focused_mode: bevy::winit::UpdateMode::Continuous,
            unfocused_mode: bevy::winit::UpdateMode::reactive_low_power(Duration::from_secs(10)),
        })
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: WINDOW_TITLE.into(),
                    name: Some(WINDOW_NAME.into()),
                    resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                    present_mode: PresentMode::AutoVsync,
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    visible: false,
                    ..default()
                }),
                ..default()
            }),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
            FpsOverlayPlugin::default(),
            CorePlugin,
            player::PlayerPlugin,
            bot::BotPlugin,
            orb::OrbPlugin,
            leaderboard::LeaderboardPlugin,
        ))
        .run();
}
