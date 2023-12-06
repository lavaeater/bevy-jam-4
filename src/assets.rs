use bevy::app::{App, Plugin, Startup};
use bevy::asset::{Assets, AssetServer, Handle};
use bevy::pbr::StandardMaterial;
use bevy::prelude::{Color, Mesh, ResMut, Resource, shape};
use bevy::scene::Scene;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SantasAssets>()
            .add_systems(Startup, (
                spawn_assets,
            ));
    }
}

#[derive(Resource, Default)]
pub struct SantasAssets {
    pub santa: Handle<Scene>,
    pub snowball_mesh: Handle<Mesh>,
    pub snowball_material: Handle<StandardMaterial>,
}

pub fn spawn_assets(
    asset_server: ResMut<AssetServer>,
    mut santas_assets: ResMut<SantasAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let radius = 0.05;
    *santas_assets = SantasAssets {
        santa: asset_server.load("models/santa_claus-modified.glb#Scene0"),
        snowball_mesh: meshes.add(
            shape::UVSphere {
                radius,
                sectors: 8,
                stacks: 4,
            }.into()),
        snowball_material: materials.add(StandardMaterial {
            base_color: Color::Rgba {
                red: 1.0,
                green: 1.0,
                blue: 1.0,
                alpha: 0.5,},
            emissive: Color::WHITE,
            metallic: 1.0,
            reflectance: 1.0,
            diffuse_transmission: 0.8,
            specular_transmission: 0.5,
            ..Default::default()
        }),
    }
}