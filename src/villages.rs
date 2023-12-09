use bevy::app::{App, Plugin, Startup, Update};
use bevy::asset::{AssetServer, Handle};
use bevy::hierarchy::BuildChildren;
use bevy::math::{EulerRot, Quat, Vec3};
use bevy::prelude::{Commands, Component, Event, EventReader, EventWriter, Res, ResMut, Resource, Scene, Transform};
use bevy::scene::SceneBundle;
use bevy_turborand::{DelegatedRng, GlobalRng};
use bevy_xpbd_3d::components::{Collider, CollisionLayers, RigidBody};
use crate::constants::GROUND_PLANE;
use crate::santa::{CollisionLayer, FixChildTransform, NeedsTransformFix};

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
            .add_systems(Startup,
                         load_level_assets,
            )
            .add_systems(Update,
                         (
                             track_game,
                             load_level,
                         ),
            )
        ;
    }
}

#[derive(Event)]
pub struct LoadLevel(u32);

#[derive(Resource)]
pub struct GameTracker {
    pub level: u32,
    pub score: u32,
    pub lives: u32,
}

fn track_game(
    mut game_tracker: ResMut<GameTracker>,
    mut load_level_ew: EventWriter<LoadLevel>,
) {
    if game_tracker.level == 0 {
        game_tracker.level += 1;
        load_level_ew.send(LoadLevel(game_tracker.level));
    }
}

#[derive(Component)]
pub struct VillageCenter;

#[derive(Component)]
pub struct House {
    pub needs_gifts: bool,
}

impl Default for House {
    fn default() -> Self {
        Self {
            needs_gifts: true,
        }
    }
}


pub const HOUSE_RADIUS: i32 = 100;

fn load_level(
    mut commands: Commands,
    mut load_level_er: EventReader<LoadLevel>,
    level_assets: Res<LevelAssets>,
    mut global_rng: ResMut<GlobalRng>,
) {
    for load_level in load_level_er.read() {
        let number_of_houses: i32 = (load_level.0 * 2 + load_level.0) as i32;

        let village_center_position = Vec3::new(global_rng.i32(-HOUSE_RADIUS..=HOUSE_RADIUS) as f32, GROUND_PLANE, global_rng.i32(-HOUSE_RADIUS..=HOUSE_RADIUS) as f32);
        commands.spawn((
            VillageCenter,
            Transform::from_translation(village_center_position)
        ));
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
                    House::default(),
                    CollisionLayers::new(
                        [CollisionLayer::House],
                        [
                            CollisionLayer::Gift,
                        ]),
                )).with_children(|children|
                { // Spawn the child colliders positioned relative to the rigid body
                    children.spawn(
                        (
                            NeedsTransformFix,
                            Collider::cuboid(5.0, 5.0, 5.0),
                            Transform::from_xyz(0.0, 0.0, 0.0),
                        ));
                });
        }
    }
}

#[derive(Resource, Default)]
pub struct LevelAssets {
    pub house_small: Handle<Scene>,
    pub house_town: Handle<Scene>,
    pub house_large: Handle<Scene>,
}

pub fn load_level_assets(
    asset_server: ResMut<AssetServer>,
    mut level_assets: ResMut<LevelAssets>,
) {
    *level_assets = LevelAssets {
        house_small: asset_server.load("models/houses/house.glb#Scene0"),
        house_town: asset_server.load("models/houses/house-town.glb#Scene0"),
        house_large: asset_server.load("models/houses/house-large.glb#Scene0"),

    }
}