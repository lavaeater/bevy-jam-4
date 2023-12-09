mod santa;
mod environment;
mod camera;
mod input;
mod snow;
mod assets;
mod sam_site;
mod collisions;
mod villages;
mod constants;

use bevy::{prelude::*};
use bevy::asset::AssetMetaCheck;
use bevy::window::WindowResolution;
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
use crate::villages::VillagePlugin;

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                resolution: WindowResolution::new(
                    1024.,
                    768.),
                ..default()
            }),
            ..default()
        }))
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
            .add_plugins(VillagePlugin)
            .add_plugins(SantaPlugin)
            .add_plugins(InputPlugin)
            .add_plugins(SamSitePlugin)
            .add_plugins(CollisionsPlugin)
            // .add_plugins(PhysicsDebugPlugin::default())
        ;
    }
}

