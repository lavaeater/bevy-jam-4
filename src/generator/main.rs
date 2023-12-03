mod generator;

use bevy::prelude::*;
use crate::generator::terrain_common::{Terrain, TerrainImageLoadOptions, TerrainMeshResource};
use crate::generator::terrain_rtin::{rtin_load_terrain, RtinParams};


fn main() {

    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        .init_resource::<TerrainMeshResource>()
        .init_resource::<RtinParams>()
        .add_systems(Startup,setup)
        .run();
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut rtin_params: ResMut<RtinParams>,
    mut terrain_mesh_res: ResMut<TerrainMeshResource>,
) {

    let image_filename = "terrain.png";

    rtin_params.error_threshold = 0.2;
    rtin_params.load_options = TerrainImageLoadOptions {
        max_image_height : 20f32,
        pixel_side_length: 1f32
    };

    let (mut terrain_shaded_mesh, terrain_wireframe_mesh) =
        rtin_load_terrain(image_filename,
            &rtin_params);

    let terrain_shaded_mesh_handle = meshes.add(terrain_shaded_mesh);
    let terrain_wireframe_mesh_handle = meshes.add(terrain_wireframe_mesh);

    terrain_mesh_res.shaded = terrain_shaded_mesh_handle;
    terrain_mesh_res.wireframe = terrain_wireframe_mesh_handle;
    // if let Some(VertexAttributeValues::Float32x3(positions)) =
    //     terrain_shaded_mesh.attribute(Mesh::ATTRIBUTE_POSITION)
    // {
    //     let colors: Vec<[f32; 4]> = positions
    //         .iter()
    //         .map(|[r, g, b]| [(1. - *r) / 2., (1. - *g) / 2., (1. - *b) / 2., 1.])
    //         .collect();
    //     terrain_shaded_mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    // }


    commands
        .spawn(
         (   PbrBundle {
                mesh: terrain_mesh_res.shaded.clone(),
                material: materials.add(Color::rgb(1., 1., 1.).into()),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),

            ..Default::default()
        },
             Terrain {}));
    commands
        .spawn(DirectionalLightBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 4.0, 0.0)),
            ..Default::default()
        });
    commands
        // camera
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 20.0, 0.0))
                .looking_at(Vec3::default(), Vec3::Y),
            ..Default::default()
        });
        // .with(FlyCamera{
        //     pitch: 180.0,
        //     ..Default::default()
        // });

    // add_axis_gizmo(commands, meshes, materials, 
    //     Transform::from_translation(Vec3::new(0f32, 0f32, 0f32)));

    // setup_ui(commands,
    //     asset_server,
    //     color_materials,
    //     button_materials, rtin_params);
}
