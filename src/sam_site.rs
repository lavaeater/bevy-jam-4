use bevy::app::{App, Plugin, Update};
use bevy::core::Name;
use bevy::hierarchy::{BuildChildren, DespawnRecursiveExt};
use bevy::math::{Quat, vec3, Vec3};
use bevy::pbr::{PbrBundle};
use bevy::prelude::{Commands, Component, Entity, GlobalTransform, Query, Res, ResMut, Resource, SceneBundle, Transform, With};
use bevy::time::Time;
use bevy_turborand::{DelegatedRng, GlobalRng};
use bevy_xpbd_3d::components::{Collider, CollisionLayers, RigidBody};
use bevy_xpbd_3d::prelude::{LinearVelocity};
use crate::assets::SantasAssets;
use crate::constants::GROUND_PLANE;
use crate::input::{CoolDown};
use crate::santa::{CollisionLayer, ParentEntity, Santa};

pub struct SamSitePlugin;

impl Plugin for SamSitePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SamSiteParams::new(2.0, 1))
            .add_systems(Update,
                         (
                             spawn_sam_sites,
                             fire_sam,
                             kill_missiles,
                             control_missiles,
                             emit_missile_trail,
                             control_missile_trail,
                         ),
            )
        ;
    }
}

#[derive(Resource)]
pub struct SamSiteParams {
    pub time_left: f32,
    pub cool_down_timer: f32,
    pub max_sam_sites: u32,
    pub sam_site_count: u32,
}

impl SamSiteParams {
    pub fn new(cool_down_timer: f32, max_sam_sites: u32) -> Self {
        Self {
            time_left: 0.0,
            cool_down_timer,
            max_sam_sites,
            sam_site_count: 0,
        }
    }
}

impl CoolDown for SamSiteParams {
    fn cool_down(&mut self, delta: f32) -> bool {
        self.time_left -= delta;
        if self.time_left <= 0.0 {
            self.time_left = self.cool_down_timer;
            return true;
        }
        false
    }
}

#[derive(Component)]
pub struct SamSite {
    pub rate_of_fire_per_minute: f32,
    pub time_left: f32,
}

impl CoolDown for SamSite {
    fn cool_down(&mut self, delta: f32) -> bool {
        self.time_left -= delta;
        if self.time_left <= 0.0 {
            self.time_left = 60.0 / self.rate_of_fire_per_minute;
            return true;
        }
        false
    }
}

#[derive(Component)]
pub struct SurfaceToAirMissile {
    pub time_to_live: f32,
    pub acceleration: f32,
    pub velocity: f32,
    pub max_velocity: f32,
}

impl SurfaceToAirMissile {
    pub fn new(time_to_live: f32, acceleration: f32, velocity: f32, max_velocity: f32) -> Self {
        Self {
            time_to_live,
            acceleration,
            velocity,
            max_velocity,
        }
    }
}

#[derive(Component)]
pub struct SamChild;

impl CoolDown for SurfaceToAirMissile {
    fn cool_down(&mut self, delta: f32) -> bool {
        self.time_to_live -= delta;
        self.time_to_live <= 0.0
    }
}

fn kill_missiles(
    mut missiles: Query<(Entity, &mut SurfaceToAirMissile)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut sam) in missiles.iter_mut() {
        if sam.cool_down(time.delta_seconds()) {
            commands.entity(entity).despawn_recursive();
        }
    }
}

#[derive(Component)]
pub struct MissileTrailEmitter {
    pub cool_down: f32,
    pub time_left: f32,
}

impl MissileTrailEmitter {
    pub fn new(cool_down: f32) -> Self {
        Self {
            cool_down,
            time_left: 0.0,
        }
    }
}

impl CoolDown for MissileTrailEmitter {
    fn cool_down(&mut self, delta: f32) -> bool {
        self.time_left -= delta;
        if self.time_left <= 0.0 {
            self.time_left = self.cool_down;
            return true;
        }
        false
    }
}

#[derive(Component)]
pub struct MissileTrail {
    pub time_to_live: f32,
    pub start_scale: f32,
    pub max_scale: f32,
    pub life_times: u32,
    pub scale_direction: i8,
}

impl MissileTrail {
    pub fn new(time_to_live: f32, start_scale: f32, max_scale: f32) -> Self {
        Self {
            time_to_live,
            start_scale,
            max_scale,
            life_times: 0,
            scale_direction: 1,
        }
    }
}

impl CoolDown for MissileTrail {
    fn cool_down(&mut self, delta: f32) -> bool {
        self.time_to_live -= delta;
        if self.time_to_live <= 0.0 {
            self.life_times += 1;
            self.scale_direction *= -1;
            return true;
        }
        false
    }
}

fn emit_missile_trail(
    mut missiles: Query<(&GlobalTransform, &mut MissileTrailEmitter)>,
    mut commands: Commands,
    time: Res<Time>,
    santas_assets: Res<SantasAssets>,
    mut global_rng: ResMut<GlobalRng>,
) {
    for (global_transform, mut emitter) in missiles.iter_mut() {
        if emitter.cool_down(time.delta_seconds()) {
            let missile_trail = MissileTrail::new(1.0, global_rng.f32(), (global_rng.f32() + 0.5) * 2.5);
            commands.spawn((
                PbrBundle {
                    mesh: santas_assets.sphere_mesh.clone(),
                    material: santas_assets.sphere_material.clone(),
                    transform: Transform::from_xyz(global_transform.translation().x, global_transform.translation().y, global_transform.translation().z).with_scale(Vec3::new(missile_trail.start_scale, missile_trail.start_scale, missile_trail.start_scale)),
                    ..Default::default()
                },
                missile_trail
            ));
        }
    }
}

fn control_missile_trail(
    mut trails: Query<(&mut MissileTrail, &mut Transform, Entity)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (mut trail, mut transform, entity) in trails.iter_mut() {
        let target_scale = if trail.scale_direction.is_positive() { trail.max_scale } else { trail.start_scale };
        transform.scale = transform.scale.lerp(Vec3::new(target_scale, target_scale, target_scale), 0.1);
        if trail.cool_down(time.delta_seconds()) && trail.life_times > 2 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn control_missiles(
    mut missiles: Query<(Entity, &GlobalTransform, &mut Transform, &mut LinearVelocity, &mut SurfaceToAirMissile)>,
    santa_position: Query<&GlobalTransform, With<Santa>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    if let Ok(santa_pos) = santa_position.get_single() {
        for (entity, missile_global_transform, mut transform, mut sam_velocity, mut sam) in missiles.iter_mut() {
            if sam.cool_down(time.delta_seconds()) {
                commands.entity(entity).despawn_recursive();
                // Spawn explosion, bro
            } else {
                if sam.velocity < sam.max_velocity {
                    sam.velocity += sam.acceleration * time.delta_seconds();
                }
                let missile_forward = missile_global_transform.forward();
                let desired_forward = missile_forward.lerp((santa_pos.translation() - missile_global_transform.translation()).normalize(), 0.1);

                sam_velocity.0 = desired_forward * sam.velocity;
                let q = Quat::from_rotation_arc(missile_forward, desired_forward);
                transform.rotate(q);
            }
        }
    }
}

fn fire_sam(
    mut commands: Commands,
    mut sam_sites: Query<(&mut SamSite, &GlobalTransform)>,
    santa_position: Query<&GlobalTransform, With<Santa>>,
    santas_assets: Res<SantasAssets>,
    time: Res<Time>,
) {
    for (mut sam_site, global_transform) in sam_sites.iter_mut() {
        if sam_site.cool_down(time.delta_seconds()) {
            let santa_pos = santa_position.get_single().unwrap();

            let sam_site_position = global_transform.translation();

            let missile_direction = (santa_pos.translation() - sam_site_position).normalize();
            let mut t = Transform::from_xyz(
                sam_site_position.x,
                sam_site_position.y,
                sam_site_position.z);
            t.rotation = Quat::from_rotation_arc(vec3(0.0, 0.0, 1.0), missile_direction);
            t.scale = Vec3::new(0.25, 0.25, 0.25);
            let missile_velocity = missile_direction * 50.0;

            commands
                .spawn((
                    Name::from("Surface2Air, Bro!"),
                    SurfaceToAirMissile::new(20.0, 25.0, 25.0, 100.0),
                    SceneBundle {
                        scene: santas_assets.missile.clone(),
                        transform: t,
                        ..Default::default()
                    },
                    MissileTrailEmitter::new(0.02),
                    RigidBody::Kinematic,
                    CollisionLayers::new(
                        [CollisionLayer::Missile],
                        [
                            CollisionLayer::Santa,
                        ]),
                    LinearVelocity::from(missile_velocity),
                )).with_children(|children|
                { // Spawn the child colliders positioned relative to the rigid body
                    children.spawn((
                        SamChild,
                        ParentEntity(children.parent_entity()),
                        Collider::ball(1.0),
                    ));
                    // children.spawn(
                    //     SpotLightBundle {
                    //         transform: Transform::from_xyz(0.0, 0.0, -0.5)
                    //             .looking_at(t.back(), t.up()),
                    //         spot_light: SpotLight {
                    //             intensity: 200000.0, // lumens
                    //             color: Color::WHITE,
                    //             shadows_enabled: true,
                    //             inner_angle: PI / 4.0 * 0.85,
                    //             outer_angle: PI / 4.0,
                    //             ..default()
                    //         },
                    //         ..default()
                    //     });
                });
        }
    }
}

fn spawn_sam_sites(
    mut sam_site_params: ResMut<SamSiteParams>,
    santas_assets: Res<SantasAssets>,
    time: Res<Time>,
    mut commands: Commands,
    where_is_santa: Query<&GlobalTransform, With<Santa>>,
) {
    if sam_site_params.cool_down(time.delta_seconds()) && sam_site_params.sam_site_count < sam_site_params.max_sam_sites {
        sam_site_params.sam_site_count += 1;
        if let Ok(santas_transform) = where_is_santa.get_single() {
            let forward = vec3(-santas_transform.forward().x, 0.0, -santas_transform.forward().z) * 100.0;
            let sam_site_position = santas_transform.translation() + forward + vec3(0.0, GROUND_PLANE, 0.0);

            commands
                .spawn((
                    Name::from("SAM Site"),
                    SamSite {
                        rate_of_fire_per_minute: 12.0,
                        time_left: 0.0,
                    },
                    // FixSceneTransform::new(
                    //     Vec3::new(0.0, -1.0, 0.0),
                    //     Quat::from_euler(
                    //         EulerRot::YXZ,
                    //         0.0, 0.0, 0.0),
                    //     Vec3::new(1.0, 1.0, 1.0),
                    // ),
                    PbrBundle {
                        mesh: santas_assets.turret.clone(),
                        material: santas_assets.turret_material.clone(),
                        transform: Transform::from_xyz(sam_site_position.x, sam_site_position.y, sam_site_position.z),
                        ..Default::default()
                    },
                    RigidBody::Static,
                    CollisionLayers::new(
                        [CollisionLayer::Solid],
                        [
                            CollisionLayer::Santa,
                            CollisionLayer::Missile,
                        ]),
                )).with_children(|children|
                { // Spawn the child colliders positioned relative to the rigid body
                    children.spawn((
                        Collider::cuboid(1.0, 1.0, 1.0),
                        Transform::from_xyz(0.0, 0.0, 0.0),
                    ));
                })
            ;
        }
    }
}
