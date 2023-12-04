use bevy::app::{App, Plugin, Startup, Update};
use bevy::core::Name;
use bevy::math::{Quat, Rect, Vec2, Vec3};
use bevy::prelude::{Camera3dBundle, Commands, Component, default, OrthographicProjection, Query, Reflect, Transform, With};
use bevy::prelude::Projection::Orthographic;
use bevy::render::camera::ScalingMode;
use bevy_atmosphere::plugin::{AtmosphereCamera, AtmospherePlugin};
use bevy_xpbd_3d::components::Position;
use bevy_xpbd_3d::math::{PI, Vector3};
use crate::santa::Santa;

pub struct CameraPlugin;

#[derive(Component)]
pub struct GameCamera {}

#[derive(Component, Reflect)]
pub struct CameraOffset(pub Vec3);

#[derive(Component)]
pub struct FollowCamera {
    pub offset: Vector3
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, camera_follow)
        ;
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle::default(),
        AtmosphereCamera::default(),
        GameCamera {},
        CameraOffset(Vec3::new(0.0, 2.5, -10.0)),
    ));
}

pub fn spawn_orthographic_camera(mut commands: Commands) {
    commands.spawn((
        Name::from("Camera"),
        CameraOffset(Vec3::new(2.0, 1.5, 2.0)),
        Camera3dBundle {
            projection: Orthographic(OrthographicProjection {
                scale: 2.0,
                near: -1000.0,
                far: 1000.0,
                viewport_origin: Vec2::new(0.5, 0.5),
                scaling_mode: ScalingMode::FixedVertical(2.0),
                area: Rect::new(-1.0, -1.0, 1.0, 1.0),
            }),
            transform: Transform {
                rotation: Quat::from_rotation_x(-PI / 4.),
                ..default()
            },
            ..default()
        },
        GameCamera {},
    ));
}

pub fn camera_follow(
    mut camera_query: Query<(&mut Transform, &CameraOffset), With<GameCamera>>,
    player_position: Query<&Position, With<Santa>>,
) {
    for (mut camera_transform, offset) in camera_query.iter_mut() {
        for player_position in player_position.iter() {
            camera_transform.translation = camera_transform.translation.lerp(player_position.0 + offset.0, 0.9);
            camera_transform.look_at(player_position.0, Vec3::Y);
        }
    }
}
