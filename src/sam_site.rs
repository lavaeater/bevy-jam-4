use bevy::app::{App, Plugin, PostStartup, Update};
use bevy::core::Name;
use bevy::hierarchy::BuildChildren;
use bevy::math::{EulerRot, Quat, Vec3};
use bevy::prelude::{Commands, Component, GlobalTransform, Query, Res, ResMut, Resource, SceneBundle, Transform, With};
use bevy::time::Time;
use bevy_xpbd_3d::components::{AngularDamping, Collider, CollisionLayers, Friction, LinearDamping, RigidBody};
use crate::assets::SantasAssets;
use crate::input::{Controller, CoolDown, KeyboardController, KinematicMovement};
use crate::santa::{CollisionLayer, FixSceneTransform, Santa};

pub struct SamSite;

impl Plugin for SamSite {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SamSiteParams::new(1.0))
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
    pub rate_of_fire_per_minute: f32
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
            let forward = santas_transform.forward() .rotation.forward();
        }

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
                SceneBundle {
                    scene: santas_assets.turret.clone(),
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    ..Default::default()
                },
                Friction::from(0.0),
                AngularDamping(1.0),
                LinearDamping(0.9),
                RigidBody::Kinematic,
                // LockedAxes::new().lock_rotation_x().lock_rotation_z(),
                CollisionLayers::new(
                    [CollisionLayer::Santa],
                    [
                        CollisionLayer::Solid,
                        CollisionLayer::Ground,
                    ]),
            )).with_children(|children|
            { // Spawn the child colliders positioned relative to the rigid body
                children.spawn((Collider::cuboid(1.2, 1.5, 2.0), Transform::from_xyz(0.0, 0.0, 0.0)));
            })
        ;
    }
}
