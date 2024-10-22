use bevy::prelude::*;

use crate::shared::GameMenuState;

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy, Default)]
enum DisplayQuality
{
    Low,
    #[default]
    Medium,
    High,
}

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
struct Volume(u32);

pub struct GameMenuPlugin;

impl Plugin for GameMenuPlugin
{
    fn build(&self, app: &mut App)
    {
        app.insert_resource(DisplayQuality::default());
        app.insert_resource(Volume(100));
        app.init_state::<GameMenuState>();
    }
}
