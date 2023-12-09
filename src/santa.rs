use bevy::app::{App, Plugin, PostStartup, Update};
use bevy::core::Name;
use bevy::hierarchy::{BuildChildren, Children};
use bevy::math::{EulerRot, Quat, Vec3, vec3};
use bevy::pbr::{SpotLight, SpotLightBundle};
use bevy::prelude::{Color, Commands, Component, Entity, GlobalTransform, Query, Res, Transform, With, Without};
use bevy::scene::SceneBundle;
use bevy::utils::default;
use bevy_xpbd_3d::components::{AngularDamping, Collider, CollisionLayers, Friction, LinearDamping, RigidBody};
use bevy_xpbd_3d::prelude::PhysicsLayer;
use crate::assets::SantasAssets;
use crate::constants::{GROUND_PLANE, SANTA_ACCELERATION, SANTA_MAX_SPEED, SANTA_TURN_SPEED};
use crate::input::{Controller, KeyboardController, KinematicMovement};
use crate::villages::{NeedsGifts, VillageCenter};

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
                    search_for_villages,
                    track_target_village
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
pub struct SantaNeedsTarget;

#[derive(Component)]
pub struct SantaHasTarget {
    pub target: Entity,
}


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
pub struct RudolphsRedNose {
    pub santa_entity: Entity
}

fn spawn_santa(
    mut commands: Commands,
    santas_assets: Res<SantasAssets>,
) {
    let santa_entity = commands.spawn((
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
        Controller::new(SANTA_MAX_SPEED, SANTA_ACCELERATION, SANTA_TURN_SPEED, 60.0),
        KinematicMovement {},
        Friction::from(0.0),
        AngularDamping(1.0),
        LinearDamping(0.9),
        RigidBody::Kinematic,
        SantaNeedsTarget,
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
        }).id();

    commands.spawn(
        (
            RudolphsRedNose { santa_entity },
            SpotLightBundle {
                spot_light: SpotLight {
                    color: Color::rgb(1.0, 0.0, 0.0),
                    intensity: 1000000.0, // Roughly a 60W non-halogen incandescent bulb
                    range: 2000.0,
                    radius: 0.0,
                    shadows_enabled: true,
                    inner_angle: std::f32::consts::FRAC_PI_8 / 16.0,
                    outer_angle: std::f32::consts::FRAC_PI_8 / 8.0,
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 0.5).looking_at(Vec3::new(0.0, -1.0, 10.0), Vec3::Y),
                ..Default::default()
            },
        ));
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

fn search_for_villages(
    village_query: Query<(Entity, &Transform, &VillageCenter), With<NeedsGifts>>,
    santas_position: Query<(Entity, &GlobalTransform), With<SantaNeedsTarget>>,
    mut commands: Commands,
) {
    if let Ok((santa_entity, santas_position)) = santas_position.get_single() {
        let mut closest_village: Option<(Entity, f32)> = None;
        for (village_entity, village_transform, _village_center) in village_query.iter() {
            if closest_village.is_none() {
                closest_village = Some((village_entity, village_transform.translation.distance(santas_position.translation())));
            } else {
                let distance = village_transform.translation.distance(santas_position.translation());
                if distance < closest_village.unwrap().1 {
                    closest_village = Some((village_entity, distance));
                }
            }
        }
        if let Some((close_village, _)) = closest_village {
            commands.entity(santa_entity).insert(SantaHasTarget { target: close_village });
            commands.entity(santa_entity).remove::<SantaNeedsTarget>();
        }
    }
}

fn track_target_village(
    mut rudolphs_nose: Query<(&mut Transform, &RudolphsRedNose)>,
    santa_query: Query<(Entity, &SantaHasTarget, &GlobalTransform), With<Santa>>,
    target_query: Query<(&GlobalTransform, &VillageCenter), (With<NeedsGifts>, Without<RudolphsRedNose>)>,
    mut commands: Commands,
) {
    for (santa_entity, santa_has_target, santa_global) in santa_query.iter() {
        if let Ok((target_position, _)) = target_query.get(santa_has_target.target) {
            for(mut rudolph_local, _) in rudolphs_nose.iter_mut() {
                rudolph_local.translation = santa_global.translation() + vec3(0.0, 0.0, 0.5);


                let target_trans =  (target_position.translation() - rudolph_local.translation).normalize() * 50.0 + rudolph_local.translation;
                rudolph_local.look_at(vec3(target_trans.x, GROUND_PLANE, target_trans.z), Vec3::Y);
            }
        } else {
            commands.entity(santa_entity).remove::<SantaHasTarget>();
            commands.entity(santa_entity).insert(SantaNeedsTarget);
        }
    }
}