mod santa;
mod environment;
mod camera;
mod input;
mod snow_plugin;

use bevy::{prelude::*};
use bevy_turborand::prelude::RngPlugin;
use bevy_xpbd_3d::plugins::{PhysicsDebugPlugin, PhysicsPlugins};
use crate::camera::CameraPlugin;
use crate::environment::EnvironmentPlugin;
use crate::input::InputPlugin;
use crate::santa::SantaPlugin;
use crate::snow_plugin::SnowPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin)
        .add_plugins(EnvironmentPlugin)
        .add_plugins(SnowPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(SantaPlugin)
        .add_plugins(InputPlugin)
        .run();
}

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(PhysicsPlugins::default())
            .add_plugins(RngPlugin::default())
            // .add_plugins(PhysicsDebugPlugin::default())
        ;
    }
}

