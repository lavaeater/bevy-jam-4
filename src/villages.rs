use bevy::app::{App, Plugin, Startup, Update};
use bevy::asset::{Assets, AssetServer, Handle};
use bevy::pbr::{AlphaMode, StandardMaterial};
use bevy::prelude::{Color, Commands, default, Event, EventReader, EventWriter, Mesh, Res, ResMut, Resource, Scene, shape};
use bevy_turborand::GlobalRng;
use crate::sam_site::SamSiteParams;

pub struct VillagePlugin;

impl Plugin for VillagePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<LevelAssets>()
            .insert_resource(GameTracker {
                level: 1,
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

fn load_level(
    mut commands: Commands,
    mut load_level_er: EventReader<LoadLevel>,
    level_assets: Res<LevelAssets>,
    mut global_rng: ResMut<GlobalRng>,
) {
    
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