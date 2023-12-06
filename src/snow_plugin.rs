use bevy::app::{App, FixedUpdate, Plugin, Update};
use bevy::core::Name;
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::math::{Vec3};
use bevy::pbr::{PbrBundle};
use bevy::prelude::{Commands, Component, Entity, Fixed, Query, Res, ResMut, Time, Transform, With};
use bevy_turborand::{DelegatedRng, GlobalRng};
use bevy_xpbd_3d::components::{CollisionLayers, Position, RigidBody};
use bevy_xpbd_3d::prelude::{ExternalForce, LinearVelocity};
use crate::assets::SantasAssets;
use crate::input::CoolDown;
use crate::santa::{CollisionLayer, Santa};

pub struct SnowPlugin;

impl Plugin for SnowPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Time::<Fixed>::from_seconds(0.05))
            .add_systems(
                Update,
                (
                    kill_snow,
                ),
            )
            .add_systems(
                FixedUpdate, (
                    spawn_snow,
                ));
    }
}

#[derive(Component)]
pub struct Snow {
    time_to_live: f32
}

impl Snow {
    pub fn new(time_to_live: f32) -> Self {
        Self {
            time_to_live,
        }
    }
}

impl CoolDown for Snow {
    fn cool_down(&mut self, delta: f32) -> bool {
        self.time_to_live -= delta;
        self.time_to_live <= 0.0
    }
}

fn kill_snow(
    mut commands: Commands,
    mut snow_query: Query<(Entity, &mut Snow)>,
    time: Res<Time>,
) {
    for (entity, mut snow) in snow_query.iter_mut() {
        if snow.cool_down(time.delta_seconds()) {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn spawn_snow(
    mut commands: Commands,
    where_is_santa: Query<&Transform, With<Santa>>,
    santas_assets: Res<SantasAssets>,
    mut global_rng: ResMut<GlobalRng>,
) {
    if let Ok(santa_position) = where_is_santa.get_single() {
        for _n in 0..10 {
            let x = global_rng.f32_normalized() * 25.0;
            let z = global_rng.f32() * 25.0;
            let y = global_rng.f32() * 10.0;

            let snow_direction = Vec3::new(x, y, z);

            let snow_position = santa_position.translation + santa_position.rotation.mul_vec3(snow_direction);

            commands.spawn(
                (
                    Name::from("SnowFlake"),
                    Snow::new(10.0),
                    PbrBundle {
                        mesh: santas_assets.snowball_mesh.clone(),
                        material: santas_assets.snowball_material.clone(),
                        ..Default::default()
                    },
                    ExternalForce::new(Vec3::ZERO),
                    Position::new(snow_position),
                    RigidBody::Kinematic,
                    LinearVelocity::from(Vec3::new(global_rng.f32() * 5.0, -global_rng.f32() * 3.0, global_rng.f32_normalized() * 2.0)),
                    CollisionLayers::new(
                        [CollisionLayer::Snow],
                        [
                            CollisionLayer::Nothing,
                        ]),
                ));
        }
    }
}