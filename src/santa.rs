use bevy::app::{App, Plugin, PostStartup, Update};
use bevy::core::Name;
use bevy::hierarchy::{BuildChildren, Children};
use bevy::math::{EulerRot, Quat, Vec3, vec3};
use bevy::pbr::{SpotLight, SpotLightBundle};
use bevy::prelude::{Color, Commands, Component, Entity, Event, EventReader, EventWriter, GlobalTransform, Query, Res, Time, Transform, With, Without};
use bevy::scene::SceneBundle;
use bevy::utils::default;
use bevy_xpbd_3d::components::{AngularDamping, Collider, CollisionLayers, Friction, LinearDamping, LinearVelocity, RigidBody};
use bevy_xpbd_3d::prelude::PhysicsLayer;
use crate::assets::SantasAssets;
use crate::constants::{GROUND_PLANE, SAM_ACCELERATION, SAM_MAX_SPEED, SAM_TIME_TO_LIVE, SANTA_ACCELERATION, SANTA_MAX_SPEED, SANTA_MISSILE_RANGE, SANTA_TURN_SPEED};
use crate::input::{Controller, CoolDown, KeyboardController, KinematicMovement};
use crate::sam_site::{MissileTrailEmitter, SamChild, SamTarget, SurfaceToAirMissile};
use crate::villages::{House, NeedsGifts};

pub struct SantaPlugin;

impl Plugin for SantaPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<TargetEvent>()
            .add_systems(
                PostStartup, (
                    spawn_santa,
                ))
            .add_systems(
                Update, (
                    fix_model_transforms,
                    search_for_targets,
                    track_target,
                    toggle_santa_shooting,
                    shoot_gifts_at_target,
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
    pub is_shooting: bool,
    pub cool_down: f32,
    pub rate_of_fire_per_minute: f32
}

impl CoolDown for SantaHasTarget {
    fn cool_down(&mut self, delta: f32) -> bool {
        self.cool_down -= delta;
        if self.cool_down <= 0.0 {
            self.cool_down = 60.0 / self.rate_of_fire_per_minute;
            true
        } else {
            false
        }
    }
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
    pub santa_entity: Entity,
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
                CollisionLayer::House,
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

// fn search_for_villages(
//     village_query: Query<(Entity, &Transform, &VillageCenter), With<NeedsGifts>>,
//     santas_position: Query<(Entity, &GlobalTransform), With<SantaNeedsTarget>>,
//     mut commands: Commands,
// ) {
//     if let Ok((santa_entity, santas_position)) = santas_position.get_single() {
//         let mut closest_village: Option<(Entity, f32)> = None;
//         for (village_entity, village_transform, _village_center) in village_query.iter() {
//             if closest_village.is_none() {
//                 closest_village = Some((village_entity, village_transform.translation.distance(santas_position.translation())));
//             } else {
//                 let distance = village_transform.translation.distance(santas_position.translation());
//                 if distance < closest_village.unwrap().1 {
//                     closest_village = Some((village_entity, distance));
//                 }
//             }
//         }
//         if let Some((close_village, _)) = closest_village {
//             commands.entity(santa_entity).insert(SantaHasTarget { target: close_village });
//             commands.entity(santa_entity).remove::<SantaNeedsTarget>();
//         }
//     }
// }

pub enum TargetEventTypes {
    Acquired(Entity),
    Lost,
    StartShooting,
    StopShooting,
}

#[derive(Event)]
pub struct TargetEvent(pub TargetEventTypes);

fn search_for_targets(
    house_query: Query<(Entity, &Transform, &House), With<NeedsGifts>>,
    santas_position: Query<(Entity, &GlobalTransform), With<SantaNeedsTarget>>,
    mut commands: Commands,
    mut target_ew: EventWriter<TargetEvent>,
) {
    if let Ok((santa_entity, santas_position)) = santas_position.get_single() {
        let mut closest_house: Option<(Entity, f32)> = None;
        for (house_entity, house_transform, _house) in house_query.iter() {
            if closest_house.is_none() {
                closest_house = Some((house_entity, house_transform.translation.distance(santas_position.translation())));
            } else {
                let distance = house_transform.translation.distance(santas_position.translation());
                if distance < closest_house.unwrap().1 {
                    closest_house = Some((house_entity, distance));
                }
            }
        }
        if let Some((close_house, _)) = closest_house {
            commands.entity(santa_entity).insert(SantaHasTarget { target: close_house, is_shooting: false, cool_down: 0.0, rate_of_fire_per_minute: 20.0 });
            commands.entity(santa_entity).remove::<SantaNeedsTarget>();
            target_ew.send(TargetEvent(TargetEventTypes::Acquired(close_house)));
        }
    }
}

fn track_target(
    mut rudolphs_nose: Query<(&mut Transform, &RudolphsRedNose)>,
    mut santa_query: Query<(Entity, &SantaHasTarget, &GlobalTransform), With<Santa>>,
    target_query: Query<&GlobalTransform, (With<NeedsGifts>, Without<RudolphsRedNose>)>,
    mut commands: Commands,
    mut target_ew: EventWriter<TargetEvent>,
) {
    for (santa_entity, santa_has_target, santa_global) in santa_query.iter_mut() {
        if let Ok(target_position) = target_query.get(santa_has_target.target) {
            for (mut rudolph_local, _) in rudolphs_nose.iter_mut() {
                rudolph_local.translation = santa_global.translation() + vec3(0.0, 0.0, 0.5);

                let vector_to_target = target_position.translation() - rudolph_local.translation;
                let distance = vector_to_target.length();
                if distance < SANTA_MISSILE_RANGE {
                    target_ew.send(TargetEvent(TargetEventTypes::StartShooting));
                } else {
                    target_ew.send(TargetEvent(TargetEventTypes::StopShooting));
                }

                let target_trans = (vector_to_target).normalize() * 50.0 + rudolph_local.translation;
                rudolph_local.look_at(vec3(target_trans.x, GROUND_PLANE, target_trans.z), Vec3::Y);
            }
        } else {
            commands.entity(santa_entity).remove::<SantaHasTarget>();
            commands.entity(santa_entity).insert(SantaNeedsTarget);
            target_ew.send(TargetEvent(TargetEventTypes::Lost));
        }
    }
}

fn toggle_santa_shooting(
    mut target_er: EventReader<TargetEvent>,
    mut santa_query: Query<(&mut SantaHasTarget), With<Santa>>,
) {
    for target_event in target_er.read() {
        match target_event.0 {
            TargetEventTypes::StartShooting => {
                for mut santa_has_target in santa_query.iter_mut() {
                    santa_has_target.is_shooting = true;
                }
            }
            TargetEventTypes::StopShooting => {}
            _ => {
                for mut santa_has_target in santa_query.iter_mut() {
                    santa_has_target.is_shooting = false;
                }
            }
        }
    }
}

fn shoot_gifts_at_target(
    mut santa_query: Query<(Entity, &mut SantaHasTarget, &GlobalTransform), With<Santa>>,
    mut commands: Commands,
    santas_assets: Res<SantasAssets>,
    time: Res<Time>,

) {
    for (_santa_entity, mut santa_has_target, global_transform) in santa_query.iter_mut() {
        if santa_has_target.is_shooting && santa_has_target.cool_down(time.delta_seconds()) {
            let target_entity = santa_has_target.target;

            let santas_position = global_transform.translation();

            let missile_direction = Vec3::Z;
            let mut t = Transform::from_xyz(
                santas_position.x,
                santas_position.y - 1.0,
                santas_position.z);
            t.rotation = Quat::from_rotation_arc(vec3(0.0, 0.0, 1.0), missile_direction);
            t.scale = Vec3::new(0.25, 0.25, 0.25);
            let missile_velocity = missile_direction * 10.0;

            commands
                .spawn((
                    Name::from("Air2Surface, Bro!"),
                    SurfaceToAirMissile::new(SAM_TIME_TO_LIVE, SAM_ACCELERATION, 10.0, SAM_MAX_SPEED),
                    SamTarget(target_entity),
                    SceneBundle {
                        scene: santas_assets.missile.clone(),
                        transform: t,
                        ..Default::default()
                    },
                    MissileTrailEmitter::new(0.02),
                    RigidBody::Kinematic,
                    CollisionLayers::new(
                        [CollisionLayer::Gift],
                        [
                            CollisionLayer::House,
                        ]),
                    LinearVelocity::from(missile_velocity),
                )).with_children(|children|
                { // Spawn the child colliders positioned relative to the rigid body
                    children.spawn((
                        SamChild,
                        ParentEntity(children.parent_entity()),
                        Collider::ball(1.0),
                    ));
                });
        }
    }
}