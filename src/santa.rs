use bevy::app::{App, Plugin, PostStartup, Update};
use bevy::core::Name;
use bevy::hierarchy::{BuildChildren, Children};
use bevy::math::{EulerRot, Quat, Vec3};
use bevy::pbr::{SpotLight, SpotLightBundle};
use bevy::prelude::{Color, Commands, Component, Entity, Query, Res, Transform, With};
use bevy::scene::SceneBundle;
use bevy::utils::default;
use bevy_xpbd_3d::components::{AngularDamping, Collider, CollisionLayers, Friction, LinearDamping, RigidBody};
use bevy_xpbd_3d::prelude::PhysicsLayer;
use crate::assets::SantasAssets;
use crate::input::{Controller, KeyboardController, KinematicMovement};

pub struct SantaPlugin;

impl Plugin for SantaPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                PostStartup, (
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
    Snow,
    Nothing,
    Missile,
    House,
    Gift,
}

#[derive(Component)]
pub struct FixChildTransform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

#[derive(Component)]
pub struct NeedsTransformFix;

impl FixChildTransform {
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

#[derive(Component)]
pub struct SantaChild;

#[derive(Component)]
pub struct ParentEntity(pub Entity);

#[derive(Component)]
pub struct Health {
    pub health: i32,
}

impl Health {
    pub fn new(health: i32) -> Self {
        Self {
            health,
        }
    }
}

#[derive(Component)]
pub struct Rudolph;

fn spawn_santa(
    mut commands: Commands,
    santas_assets: Res<SantasAssets>,
) {
    commands.spawn((
        Name::from("Saint Nicholas"),
        Santa {},
        Health::new(100),
        FixChildTransform::new(
            Vec3::new(0.0, 1.0, 0.0),
            Quat::from_euler(
                EulerRot::YXZ,
                0.0, 0.0, 0.0),
            Vec3::new(1.0, 1.0, 1.0),
        ),
        SceneBundle {
            scene: santas_assets.santa.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
        KeyboardController {},
        Controller::new(100.0, 10.0, 5.0, 60.0),
        KinematicMovement {},
        Friction::from(0.0),
        AngularDamping(1.0),
        LinearDamping(0.9),
        RigidBody::Kinematic,
        CollisionLayers::new(
            [CollisionLayer::Santa],
            [
                CollisionLayer::Solid,
                CollisionLayer::Ground,
                CollisionLayer::Missile,
            ]),
    )).with_children(|children|
        { // Spawn the child colliders positioned relative to the rigid body
            children.spawn(
                (
                    NeedsTransformFix,
                    SantaChild {},
                    ParentEntity(children.parent_entity()),
                    Collider::cuboid(1.2, 1.5, 2.0),
                    Transform::from_xyz(0.0, 0.0, 0.0),
                ));
            children.spawn(
                (
                    Rudolph,
                    SpotLightBundle {
                        spot_light: SpotLight {
                            color: Color::rgb(1.0, 0.0, 0.0),
                            intensity: 80000.0, // Roughly a 60W non-halogen incandescent bulb
                            range: 2000.0,
                            radius: 0.0,
                            shadows_enabled: true,
                            inner_angle: std::f32::consts::FRAC_PI_8 / 8.0,
                            outer_angle: std::f32::consts::FRAC_PI_8 / 4.0,
                            ..default()
                        },
                        transform: Transform::from_xyz(0.0, 0.0, 0.5).looking_at(Vec3::new(0.0, -1.0, 2.0), Vec3::Y),
                        ..Default::default()
                    },
                ));
        });
}

pub fn fix_model_transforms(
    mut commands: Commands,
    mut scene_instance_query: Query<(Entity, &FixChildTransform, &Children)>,
    mut child_query: Query<&mut Transform, With<NeedsTransformFix>>,
) {
    for (parent, fix_scene_transform, children) in scene_instance_query.iter_mut() {
        for child in children.iter() {
            if let Ok(mut transform) = child_query.get_mut(*child) {
                transform.translation = fix_scene_transform.translation;
                transform.rotation = fix_scene_transform.rotation;
                transform.scale = fix_scene_transform.scale;
                commands.entity(parent).remove::<FixChildTransform>();
                commands.entity(*child).remove::<NeedsTransformFix>();
            }
        }
    }
}