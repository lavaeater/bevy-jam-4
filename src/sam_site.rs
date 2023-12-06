use bevy::app::{App, Plugin, Update};
use bevy::core::Name;
use bevy::hierarchy::BuildChildren;
use bevy::math::{vec3};
use bevy::pbr::PbrBundle;
use bevy::prelude::{Commands, Component, GlobalTransform, Query, Res, ResMut, Resource, SceneBundle, Transform, With};
use bevy::time::Time;
use bevy_xpbd_3d::components::{Collider, CollisionLayers, RigidBody};
use crate::assets::SantasAssets;
use crate::input::{CoolDown};
use crate::santa::{CollisionLayer, Santa};

pub struct SamSitePlugin;

impl Plugin for SamSitePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SamSiteParams::new(10.0))
            .add_systems(Update,
                         (
                             spawn_sam_sites,
                         ),
            )
        ;
    }
}

#[derive(Resource)]
pub struct SamSiteParams {
    pub time_left: f32,
    pub cool_down_timer: f32,
}

impl SamSiteParams {
    pub fn new(cool_down_timer: f32) -> Self {
        Self {
            time_left: 0.0,
            cool_down_timer,
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

fn spawn_sam_sites(
    mut sam_site_params: ResMut<SamSiteParams>,
    santas_assets: Res<SantasAssets>,
    time: Res<Time>,
    mut commands: Commands,
    where_is_santa: Query<&GlobalTransform, With<Santa>>,
) {
    if sam_site_params.cool_down(time.delta_seconds()) {
        if let Ok(santas_transform) = where_is_santa.get_single() {

            let sam_site_position = santas_transform.translation() + -santas_transform.forward() * 1000.0 + vec3(0.0, -50.0, 0.0);

            commands
                .spawn((
                    Name::from("SAM Site"),
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
