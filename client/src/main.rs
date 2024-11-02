#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_variables)]

mod fps;
mod constants;
mod game;
mod menu;
mod player;
mod settings;
mod enums;
mod segments;
mod orb;
mod utils;

use std::time::Duration;

use bevy::core::FrameCount;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowTheme};
use bevy::winit::WinitSettings;
use constants::*;

use enums::GameState;

pub fn setup_scene(mut commands: Commands)
{
    // Setup the camera.
    // This needs to be spawned before anything else.
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true, // HDR is required for the bloom effect
                ..default()
            },
            ..default()
        },
        BloomSettings::NATURAL,
    ));
}

/// Make the window visible in the next 3 frames.
/// We use this to avoid the white window that shows up before the GPU is ready to render the app.
/// This happens so fast the the user will not see it.
fn make_visible(mut window: Query<&mut Window>, frames: Res<FrameCount>)
{
    // The delay may be different for your app or system.
    if frames.0 == 3 {
        // At this point the gpu is ready to show the app so we can make the window visible.
        // Alternatively, you could toggle the visibility in Startup.
        // It will work, but it will have one white frame before it starts rendering
        window.single_mut().visible = true;
    }
}

/// Update winit config based on the current game state
/// This keeps the game responsive, when removed fps is unstable for some reason lol
fn update_winit(mode: Res<GameState>, mut winit_config: ResMut<WinitSettings>)
{
    match *mode {
        GameState::Menu => {
            winit_config.focused_mode = bevy::winit::UpdateMode::reactive_low_power(Duration::from_millis(10));
            winit_config.unfocused_mode = bevy::winit::UpdateMode::reactive_low_power(Duration::from_secs(1));
        }
        GameState::Game => {
            winit_config.focused_mode = bevy::winit::UpdateMode::Continuous;
            winit_config.unfocused_mode = bevy::winit::UpdateMode::reactive_low_power(Duration::from_millis(10));
        }
        GameState::Splash => {
            winit_config.focused_mode = bevy::winit::UpdateMode::reactive_low_power(Duration::from_millis(10));
            winit_config.unfocused_mode = bevy::winit::UpdateMode::reactive_low_power(Duration::from_secs(1));
        }
    }
}

fn main()
{
    App::new()
        .insert_resource(GameState::default())
        .insert_resource(WinitSettings {
            focused_mode: bevy::winit::UpdateMode::Reactive {
                wait: Duration::from_millis(250),
                react_to_device_events: false,
                react_to_user_events: false,
                react_to_window_events: true,
            },
            unfocused_mode: bevy::winit::UpdateMode::reactive_low_power(Duration::from_millis(10)),
        })
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Slither Wars Client".into(),
                    name: Some("slither-wars.app".into()),
                    resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                    // Turns off vsync to maximize CPU/GPU usage
                    present_mode: PresentMode::AutoVsync,
                    // Tells wasm to resize the window according to the available canvas
                    fit_canvas_to_parent: true,
                    // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    enabled_buttons: bevy::window::EnabledButtons {
                        // maximize: true,
                        ..Default::default()
                    },
                    // This will spawn an invisible window
                    // The window will be made visible in the make_visible() system after 3 frames.
                    // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
                    visible: false,
                    ..default()
                }),
                ..default()
            }),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
        ))
        .add_plugins(fps::FpsPlugin)
        .add_plugins(menu::GameMenuPlugin)
        .add_plugins(game::GamePlugin)
        .add_systems(Startup, setup_scene)
        .add_systems(Update, (make_visible, update_winit))
        .run();
}
