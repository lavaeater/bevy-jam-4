use bevy::app::{App, Plugin, Startup, Update};
use bevy::asset::AssetServer;
use bevy::core::Name;
use bevy::hierarchy::{BuildChildren, Children};
use bevy::math::{EulerRot, Quat, Vec3};
use bevy::prelude::{Commands, Component, Entity, Query, Res, Transform, Visibility, With};
use bevy::scene::SceneBundle;
use bevy_xpbd_3d::components::{AngularDamping, Collider, CollisionLayers, Friction, LinearDamping, RigidBody};
use bevy_xpbd_3d::prelude::PhysicsLayer;
use crate::input::{Controller, KeyboardController, KinematicMovement};

pub struct SantaPlugin;

impl Plugin for SantaPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Startup, (
                    spawn_santa,
                ))
            .add_systems(
                Update, (
                    fix_model_transforms,
                ),
            )
        ;
    }
}


#[derive(PhysicsLayer, PartialEq, Eq, Clone, Copy)]
pub enum CollisionLayer {
    Santa,
    Ground,
    Solid,
    Sensor,
}

#[derive(Component)]
pub struct FixSceneTransform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl FixSceneTransform {
    pub fn new(translation: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self {
            translation,
            rotation,
            scale,
        }
    }
}

#[derive(Component)]
pub struct Santa;

fn spawn_santa(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Name::from("Saint Nicholas"),
        Santa {},
        FixSceneTransform::new(
            Vec3::new(0.0, -1.0, 0.0),
            Quat::from_euler(
                EulerRot::YXZ,
                0.0, 0.0, 0.0),
            Vec3::new(1.0, 1.0, 1.0),
        ),
        KeyboardController {},
        Controller::new(3.0, 3.0, 60.0),
        KinematicMovement {},
        SceneBundle {
            scene: asset_server.load("models/santa_claus-modified.glb#Scene0"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
        Friction::from(0.0),
        AngularDamping(1.0),
        LinearDamping(0.9),
        RigidBody::Kinematic,
        // LockedAxes::new().lock_rotation_x().lock_rotation_z(),
        CollisionLayers::new(
            [CollisionLayer::Santa],
            [
                CollisionLayer::Solid,
                CollisionLayer::Ground,
            ]),
    )).with_children(|children|
        { // Spawn the child colliders positioned relative to the rigid body
            children.spawn((Collider::cuboid(1.2, 1.5, 2.0), Transform::from_xyz(0.0, 0.0, 0.0)));
        });
}

pub fn fix_model_transforms(
    mut commands: Commands,
    mut scene_instance_query: Query<(Entity, &FixSceneTransform, &Children)>,
    mut child_query: Query<&mut Transform, With<Visibility>>,
) {
    for (parent, fix_scene_transform, children) in scene_instance_query.iter_mut() {
        for child in children.iter() {
            if let Ok(mut transform) = child_query.get_mut(*child) {
                transform.translation = fix_scene_transform.translation;
                transform.rotation = fix_scene_transform.rotation;
                transform.scale = fix_scene_transform.scale;
                commands.entity(parent).remove::<FixSceneTransform>();
            }
        }
    }
}