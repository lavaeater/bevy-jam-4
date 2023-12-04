use bevy::app::{App, Plugin, Startup};
use bevy::prelude::{Camera3dBundle, Commands};
use bevy_atmosphere::plugin::{AtmosphereCamera, AtmospherePlugin};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(AtmospherePlugin)
            .add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle::default(),
        AtmosphereCamera::default(),
    ));
}
