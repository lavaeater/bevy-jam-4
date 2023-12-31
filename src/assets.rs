use bevy::app::{App, Plugin, Startup};
use bevy::asset::{Assets, AssetServer, Handle};
use bevy::pbr::{AlphaMode, StandardMaterial};
use bevy::prelude::{Color, Mesh, ResMut, Resource, shape};
use bevy::scene::Scene;
use bevy::utils::default;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SantasAssets>()
            .add_systems(Startup, (
                load_assets,
            ));
    }
}

#[derive(Resource, Default)]
pub struct SantasAssets {
    pub santa: Handle<Scene>,
    pub turret: Handle<Mesh>,
    pub turret_material: Handle<StandardMaterial>,
    pub snowball_mesh: Handle<Mesh>,
    pub snowball_material: Handle<StandardMaterial>,
    pub missile: Handle<Scene>,
    pub trail_mesh: Handle<Mesh>,
    pub trail_material: Handle<StandardMaterial>
}

pub fn load_assets(
    asset_server: ResMut<AssetServer>,
    mut santas_assets: ResMut<SantasAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let radius = 0.05;
    *santas_assets = SantasAssets {
        santa: asset_server.load("models/santa_claus-modified.glb#Scene0"),
        turret: meshes.add(
            shape::Cube {
                size: 5.0,
            }.into()),
        turret_material: materials.add(StandardMaterial {
            base_color: Color::GREEN,
            ..default()
        }),
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
            emissive: Color::Rgba {
                red: 0.5,
                green: 0.5,
                blue: 0.5,
                alpha: 1.0},
            metallic: 1.0,
            reflectance: 1.0,
            diffuse_transmission: 0.8,
            specular_transmission: 0.5,
            ..Default::default()
        }),
        missile: asset_server.load("models/missile.glb#Scene0"),
        trail_mesh: meshes.add(
            shape::UVSphere {
                radius: 1.0,
                sectors: 16,
                stacks: 8,
            }.into()),
        trail_material: materials.add(StandardMaterial {
            base_color: Color::Rgba {
                red: 1.0,
                green: 1.0,
                blue: 0.0,
                alpha: 0.1,},
            // emissive: Color::YELLOW,
            // metallic: 1.0,
            // reflectance: 0.0,
            diffuse_transmission: 1.0,
            alpha_mode: AlphaMode::Blend,
            ..Default::default()
        }),
    }
}