mod santa;
mod environment;
mod camera;
mod input;
mod snow;
mod assets;
mod sam_site;
mod collisions;

use bevy::{prelude::*};
use bevy_turborand::prelude::RngPlugin;
use bevy_xpbd_3d::plugins::{PhysicsDebugPlugin, PhysicsPlugins};
use crate::assets::AssetsPlugin;
use crate::camera::CameraPlugin;
use crate::collisions::CollisionsPlugin;
use crate::environment::EnvironmentPlugin;
use crate::input::InputPlugin;
use crate::sam_site::SamSitePlugin;
use crate::santa::SantaPlugin;
use crate::snow::SnowPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin)
        .run();
}

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(AssetsPlugin)
            .add_plugins(PhysicsPlugins::default())
            .add_plugins(RngPlugin::default())
            .add_plugins(EnvironmentPlugin)
            .add_plugins(SnowPlugin)
            .add_plugins(CameraPlugin)
            .add_plugins(SantaPlugin)
            .add_plugins(InputPlugin)
            .add_plugins(SamSitePlugin)
            .add_plugins(CollisionsPlugin)
            .add_plugins(PhysicsDebugPlugin::default())
        ;
    }
}

