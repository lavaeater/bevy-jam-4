use bevy::app::{App, Plugin, Startup};
use bevy::core::Name;
use bevy::math::{EulerRot, Quat};
use bevy::pbr::{CascadeShadowConfigBuilder, DirectionalLight, DirectionalLightBundle};
use bevy::prelude::{Commands, default, Transform};

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_plugins(AtmospherePlugin)
            // .insert_resource(AtmosphereModel::new(Nishita {
            //     sun_position: Vec3::new(1.0, 0.5, 1.0),
            //     ..default() }))
            // .insert_resource(CycleTimer(Timer::new(
            //     Duration::from_millis(500), // Update our atmosphere every 50ms (in a real game, this would be much slower, but for the sake of an example we use a faster update)
            //     TimerMode::Repeating,
            // )))
            .add_systems(Startup, (
                spawn_lights,
            ))
            // .add_systems(Update, daylight_cycle)
        ;
    }
}

// fn daylight_cycle(
//     mut atmosphere: AtmosphereMut<Nishita>,
//     mut timer: ResMut<CycleTimer>,
//     time: Res<Time>,
// ) {
//     timer.0.tick(time.delta());
//
//     if timer.0.finished() {
//         // let t = (time.elapsed_seconds_wrapped() / 50.0) / 2.0;
//         atmosphere.sun_position = Quat::from_euler(EulerRot::YXZ, 0.0, 5f32.to_radians(), 0.0).mul_vec3(atmosphere.sun_position);
//     }
// }
//
// // Timer for updating the daylight cycle (updating the atmosphere every frame is slow, so it's better to do incremental changes)
// #[derive(Resource)]
// struct CycleTimer(Timer);

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