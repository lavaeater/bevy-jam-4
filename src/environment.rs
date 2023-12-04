use bevy::app::{App, Plugin, Startup};
use bevy::core::Name;
use bevy::math::{EulerRot, Quat};
use bevy::pbr::{CascadeShadowConfigBuilder, DirectionalLight, DirectionalLightBundle};
use bevy::prelude::{Commands, default, Transform};
use bevy_atmosphere::plugin::AtmospherePlugin;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(AtmospherePlugin)
            .add_systems(Startup, (
                spawn_lights,
            ))
        ;
    }
}

pub fn spawn_lights(
    mut commands: Commands,
) {
    commands.spawn((
        Name::from("Directional Light"),
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 5000.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform {
                rotation: Quat::from_euler(EulerRot::XYZ, -0.5, 0.2, 0.4),
                ..default()
            },
// The default cascade config is designed to handle large scenes.
// As this example has a much smaller world, we can tighten the shadow
// bounds for better visual quality.
            cascade_shadow_config: CascadeShadowConfigBuilder {
                first_cascade_far_bound: 4.0,
                maximum_distance: 10.0,
                ..default()
            }
                .into(),
            ..default()
        }));
}