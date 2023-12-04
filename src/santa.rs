use bevy::app::{App, Plugin, Startup};
use bevy::asset::AssetServer;
use bevy::core::Name;
use bevy::hierarchy::BuildChildren;
use bevy::prelude::{Commands, Component, Res, ResMut, Transform};
use bevy::scene::SceneBundle;
use bevy_xpbd_3d::components::{AngularDamping, Collider, CollisionLayers, Friction, LinearDamping, LockedAxes, RigidBody};
use bevy_xpbd_3d::prelude::PhysicsLayer;
use crate::input::{Controller, DynamicMovement, KeyboardController, KinematicMovement};

pub struct SantaPlugin;

impl Plugin for SantaPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (
                spawn_santa,
            ));
    }
}


#[derive(PhysicsLayer,PartialEq, Eq, Clone, Copy)]
pub enum CollisionLayer {
    Floor,
    Ball,
    Impassable,
    Alien,
    Player,
    AlienSpawnPoint,
    AlienGoal,
    BuildIndicator,
    Sensor,
}

#[derive(Component)]
pub struct Santa;

fn spawn_santa(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn((
        Name::from("Saint Nicholas"),
        Santa {},
        // FixSceneTransform::new(
        //     Vec3::new(0.0, -0.37, 0.0),
        //     Quat::from_euler(
        //         EulerRot::YXZ,
        //         180.0f32.to_radians(), 0.0, 0.0),
        //     Vec3::new(0.5, 0.5, 0.5),
        // ),
        KeyboardController {},
        Controller::new(3.0, 3.0, 60.0),
        KinematicMovement {},
        SceneBundle {
            scene: asset_server.load("models/santa_claus.glb#Scene0"),
            transform: Transform::from_xyz(0.0,0.0,0.0),
            ..Default::default()
        },
        Friction::from(0.0),
        AngularDamping(1.0),
        LinearDamping(0.9),
        RigidBody::Kinematic,
        LockedAxes::new().lock_rotation_x().lock_rotation_z(),
        CollisionLayers::new(
            [CollisionLayer::Player],
            [
                CollisionLayer::Ball,
                CollisionLayer::Impassable,
                CollisionLayer::Floor,
                CollisionLayer::Alien,
                CollisionLayer::Player,
                CollisionLayer::AlienSpawnPoint,
                CollisionLayer::AlienGoal
            ]),
    )).with_children(|children|
        { // Spawn the child colliders positioned relative to the rigid body
            children.spawn((Collider::cuboid(2.0, 2.0, 4.0), Transform::from_xyz(0.0, 0.0, 0.0)));
        });
}