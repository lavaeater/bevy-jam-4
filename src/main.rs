mod santa;
mod environment;
mod camera;
mod input;

use bevy::{prelude::*};
use bevy_xpbd_3d::plugins::{PhysicsDebugPlugin, PhysicsPlugins};
use crate::camera::CameraPlugin;
use crate::input::InputPlugin;
use crate::santa::SantaPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin)
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
            .add_plugins(PhysicsDebugPlugin::default());
    }
}

