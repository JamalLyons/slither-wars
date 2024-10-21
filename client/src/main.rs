///! Author: CodingWithJamal
///! Date: 10/20/24
///!
///! Description:
mod plugins;
mod ui;

use bevy::prelude::*;

use crate::plugins::setup_scene;

fn main()
{
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(plugins::camera::CameraPlugin)
        .add_plugins(plugins::fps::FpsPlugin)
        .add_systems(Startup, setup_scene)
        .run();
}
