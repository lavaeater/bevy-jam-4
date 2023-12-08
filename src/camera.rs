use bevy::app::{App, Plugin, PostUpdate, Startup};
use bevy::math::{Vec3};
use bevy::pbr::{FogFalloff, FogSettings};
use bevy::prelude::{Camera3dBundle, Color, Commands, Component, default, IntoSystemConfigs, Query, Reflect, Transform, With, Without};
use bevy::transform::TransformSystem;
use bevy_xpbd_3d::math::{Vector3};
use bevy_xpbd_3d::PhysicsSet;
use crate::santa::Santa;

pub struct CameraPlugin;

#[derive(Component)]
pub struct GameCamera {}

#[derive(Component, Reflect)]
pub struct CameraOffset(pub Vec3);

#[derive(Component)]
pub struct FollowCamera {
    pub offset: Vector3,
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Startup, (
                    spawn_camera,
                ))
            .add_systems(
                PostUpdate,
                (
                    camera_follow
                        .after(PhysicsSet::Sync)
                        .before(TransformSystem::TransformPropagate),
                ),
            );
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle::default(),
        FogSettings {
            color: Color::rgba(0.25, 0.25, 0.25, 1.0),
            falloff: FogFalloff::Linear {
                start: 50.0,
                end: 150.0,
            },
            ..default()
        },
        // AtmosphereCamera::default(),
        GameCamera {},
        CameraOffset(Vec3::new(0.0, 15.0, -40.0)),
    ));
}

pub fn camera_follow(
    mut camera_query: Query<(&mut Transform, &CameraOffset), (With<GameCamera>, Without<Santa>)>,
    player_position: Query<&Transform, (With<Santa>, Without<GameCamera>)>,
) {
    for (mut camera_transform, offset) in camera_query.iter_mut() {
        for player_position in player_position.iter() {
            //rotate the offset so it is BEHIND the player
            let mut actual_offset = offset.0;
            actual_offset = player_position.rotation.mul_vec3(actual_offset);

            camera_transform.translation = camera_transform.translation.lerp(player_position.translation + actual_offset, 0.9);
            camera_transform.look_at(player_position.translation, Vec3::Y);
        }
    }
}
