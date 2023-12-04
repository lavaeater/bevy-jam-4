mod santa;
mod environment;
mod camera;
mod input;

use bevy::{prelude::*};
use crate::camera::CameraPlugin;
use crate::input::InputPlugin;
use crate::santa::SantaPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(SantaPlugin)
        .add_plugins(InputPlugin)
        .run();
}

