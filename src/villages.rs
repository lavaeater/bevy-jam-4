use bevy::app::{App, Plugin, PostStartup, Startup, Update};
use bevy::asset::{Assets, AssetServer, Handle};
use bevy::core::Name;
use bevy::hierarchy::BuildChildren;
use bevy::math::{EulerRot, Quat, Vec3};
use bevy::pbr::{PbrBundle, StandardMaterial};
use bevy::prelude::{Color, Commands, Component, Entity, Event, EventReader, EventWriter, GlobalTransform, Mesh, Query, Res, ResMut, Resource, Scene, shape, Transform, With};
use bevy::scene::SceneBundle;
use bevy::utils::default;
use bevy_turborand::{DelegatedRng, GlobalRng};
use bevy_xpbd_3d::components::{Collider, CollisionLayers, RigidBody};
use bevy_xpbd_3d::math::PI;
use crate::constants::{GROUND_PLANE, HOUSE_RADIUS, HOUSE_SPAWN_DISTANCE};
use crate::sam_site::SpawnSamSiteAt;
use crate::santa::{CollisionLayer, FixChildTransform, GameEvent, GameEventTypes, NeedsTransformFix, ParentEntity, Santa};

pub struct VillagePlugin;

impl Plugin for VillagePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<LevelAssets>()
            .insert_resource(GameTracker {
                level: 0,
                score: 0,
                lives: 3,
            })
            .add_event::<LoadLevel>()
            .add_event::<HouseEvent>()
            .add_systems(Startup,
                         load_level_assets,
            )
            .add_systems(PostStartup,
                         create_ground,
            )
            .add_systems(Update,
                         (
                             load_level,
                         ),
            )
        ;
    }
}

#[derive(Event)]
pub struct LoadLevel(pub u32);

#[derive(Event)]
pub struct HouseEvent(pub HouseEventType);

pub enum HouseEventType {
    ReceivedGifts(Entity),
}

#[derive(Resource)]
pub struct GameTracker {
    pub level: u32,
    pub score: u32,
    pub lives: u32,
}

#[derive(Component)]
pub struct VillageCenter {
    pub level: u32,
    pub needs_gifts_count: i32,
    pub needs_gifts: bool,
}


#[derive(Component)]
pub struct NeedsGifts;

#[derive(Component)]
pub struct House {
    pub belongs_to_village: Entity,
}

impl House {
    pub fn new(belongs_to_village: Entity) -> Self {
        Self {
            belongs_to_village,
        }
    }
}

fn create_ground(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
) {
    commands.spawn((
        Name::from("Ground"),
        PbrBundle {
            mesh: level_assets.ground_mesh.clone(),
            material: level_assets.ground_material.clone(),
            transform: Transform::from_xyz(0.0, GROUND_PLANE, 0.0),
            ..Default::default()
        },
    ));
}

#[derive(Component)]
pub struct HouseChild;

fn load_level(
    mut commands: Commands,
    mut load_level_er: EventReader<LoadLevel>,
    level_assets: Res<LevelAssets>,
    mut global_rng: ResMut<GlobalRng>,
    santas_global_transform: Query<&GlobalTransform, With<Santa>>,
    mut spawn_sam_sites_ew: EventWriter<SpawnSamSiteAt>,
    mut game_won_ew: EventWriter<GameEvent>
) {
    for load_level in load_level_er.read() {
        if load_level.0 > 10 {
            game_won_ew.send(GameEvent{ event_type: GameEventTypes::Won });
            continue;
        }
        let number_of_houses: i32 = (load_level.0 * 2 + load_level.0) as i32;

        let number_of_sam_sites = load_level.0 * load_level.0 - 1;

        let santas_position = santas_global_transform.single();

        let village_direction = (if load_level.0 == 1 { Vec3::Z } else {  Quat::from_euler(EulerRot::ZYX, global_rng.f32_normalized() * PI, 0.0 ,0.0).mul_vec3(santas_position.forward()) }) * global_rng.u32(2..10) as f32 * HOUSE_SPAWN_DISTANCE;

        let village_center_position = Vec3::new(global_rng.i32(-HOUSE_RADIUS..=HOUSE_RADIUS) as f32 + village_direction.x, GROUND_PLANE, global_rng.i32(-HOUSE_RADIUS..=HOUSE_RADIUS) as f32 + village_direction.z);
        let village_entity = commands.spawn(
            (
                VillageCenter {
                    level: load_level.0,
                    needs_gifts_count: number_of_houses,
                    needs_gifts: true,
                },
                SceneBundle {
                    scene: level_assets.christmas_tree.clone(),
                    transform: Transform::from_translation(village_center_position),
                    ..default()
                }
        ))
        .id();

        let mut min_x = 0.0;
        let mut max_x = 0.0;
        let mut min_y  = 0.0;
        let mut max_y = 0.0;

        for n in 0..number_of_houses {
            let house_type = global_rng.i32(0..3);
            let house =
                match house_type {
                    0 => level_assets.house_small.clone(),
                    1 => level_assets.house_town.clone(),
                    2 => level_assets.house_large.clone(),
                    _ => panic!("Invalid house type"),
                };
            let x_i: i32 = n % (number_of_houses / 2) - number_of_houses / 2;
            let z_i: i32 = n / (number_of_houses / 2) - number_of_houses / 2;
            let x = village_center_position.x + x_i as f32 * 30.0;
            let z = village_center_position.z + z_i as f32 * 30.0;
            let y = village_center_position.y;

            if x < min_x {
                min_x = x;
            }
            if x > max_x {
                max_x = x;
            }

            if y < min_y {
                min_y = y;
            }
            if y > max_y {
                max_y = y;
            }

            commands.spawn(
                (
                    FixChildTransform::new(
                        Vec3::new(0.0, 2.0, 0.0),
                        Quat::from_euler(
                            EulerRot::YXZ,
                            0.0, 0.0, 0.0),
                        Vec3::new(1.0, 1.0, 1.0),
                    ),
                    SceneBundle {
                        transform: Transform::from_xyz(x, y, z),
                        scene: house,
                        ..Default::default()
                    },
                    RigidBody::Kinematic,
                    House::new(village_entity),
                    NeedsGifts,
                    CollisionLayers::new(
                        [CollisionLayer::House],
                        [
                            CollisionLayer::Gift,
                            CollisionLayer::Santa,
                        ]),
                )).with_children(|children|
                { // Spawn the child colliders positioned relative to the rigid body
                    children.spawn(
                        (
                            ParentEntity(children.parent_entity()),
                            HouseChild,
                            NeedsTransformFix,
                            Collider::cuboid(10.0, 10.0, 10.0),
                            Transform::from_xyz(0.0, 0.0, 0.0),
                        ));
                });
        }
        for _ in 0..number_of_sam_sites {

            let x = global_rng.i32(min_x as i32..max_x as i32) as f32;
            let y = global_rng.i32(min_y as i32..max_y as i32) as f32;

            spawn_sam_sites_ew.send(SpawnSamSiteAt {
                position: village_center_position + Vec3::new(x, 0.0, y),
                belongs_to: village_entity
            });
        }
    }
}

#[derive(Resource, Default)]
pub struct LevelAssets {
    pub house_small: Handle<Scene>,
    pub house_town: Handle<Scene>,
    pub house_large: Handle<Scene>,
    pub ground_mesh: Handle<Mesh>,
    pub ground_material: Handle<StandardMaterial>,
    pub christmas_tree: Handle<Scene>,
}

pub fn load_level_assets(
    asset_server: ResMut<AssetServer>,
    mut level_assets: ResMut<LevelAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    *level_assets = LevelAssets {
        house_small: asset_server.load("models/houses/house.glb#Scene0"),
        house_town: asset_server.load("models/houses/house-town.glb#Scene0"),
        house_large: asset_server.load("models/houses/house-large.glb#Scene0"),
        ground_mesh: meshes.add(Mesh::from(shape::Plane { size: 100000.0, subdivisions: 4 })),
        ground_material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.0, 0.1, 0.0),
            ..default()
        }),
        christmas_tree:asset_server.load("models/christmas-tree.glb#Scene0"),
    }
}