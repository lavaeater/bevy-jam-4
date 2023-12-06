use bevy::app::{App, FixedUpdate, Plugin, Update};
use bevy::asset::{Assets};
use bevy::core::Name;
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::math::{Vec3};
use bevy::pbr::{PbrBundle, StandardMaterial};
use bevy::prelude::{Color, Commands, Component, Entity, Fixed, Mesh, Query, Res, ResMut, Resource, shape, Time, Transform, With};
use bevy_turborand::{DelegatedRng, GlobalRng};
use bevy_xpbd_3d::components::{Collider, LinearDamping, Position, RigidBody};
use bevy_xpbd_3d::prelude::{ExternalForce, MassPropertiesBundle};
use crate::input::CoolDown;
use crate::santa::Santa;

pub struct SnowPlugin;

impl Plugin for SnowPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Time::<Fixed>::from_seconds(0.05))
            .insert_resource(Wind {
                wind_force: Vec3::new(5.0, 2.0, -5.0),
                wind_force_max: Vec3::new(0.0, 2.0, 0.1),
                factor: 0.0,
                factor_max: 0.1,
            })
            .add_systems(
                Update,
                (
                    kill_snow,
                    snow_drag,
                ),
            )
            .add_systems(
                FixedUpdate, (
                    spawn_snow,
                    change_wind,
                ));
    }
}

#[derive(Resource)]
pub struct Wind {
    pub wind_force: Vec3,
    pub wind_force_max: Vec3,
    pub factor: f32,
    pub factor_max: f32,
}

#[derive(Component)]
pub struct Snow {
    time_to_live: f32,
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

fn change_wind(
    mut wind: ResMut<Wind>,
    mut global_rng: ResMut<GlobalRng>,
) {
    // wind.factor += global_rng.f32_normalized();
    // if wind.factor > wind.factor_max {
    //     wind.factor = -wind.factor_max;
    // }
    // if wind.factor < -wind.factor_max {
    //     wind.factor = wind.factor_max;
    // }
    //
    // wind.wind_force = wind.wind_force_max * wind.factor;
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

pub const SNOW_DRAG_FACTOR: f32 = 0.0002;

fn snow_drag(
    mut snow_query: Query<&mut ExternalForce, With<Snow>>,
    wind: Res<Wind>,
    time: Res<Time>,
) {
    for mut force in snow_query.iter_mut() {
        force.set_force(wind.wind_force * time.delta_seconds() * SNOW_DRAG_FACTOR);
    }
}

fn spawn_snow(
    mut commands: Commands,
    where_is_santa: Query<&Transform, With<Santa>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut global_rng: ResMut<GlobalRng>,
) {
    if let Ok(santa_position) = where_is_santa.get_single() {
        for _n in 0..10 {
            let x = global_rng.f32_normalized() * 25.0;
            let z = -global_rng.f32() * 25.0;

            let snow_direction = Vec3::new(0.0, 0.0, 10.0);

            let snow_position = santa_position.translation + santa_position.rotation.mul_vec3(snow_direction) + Vec3::new(x, 10.0, z);

            let material = materials.add(StandardMaterial {
                base_color: Color::WHITE,
                ..Default::default()
            });

            let radius = 0.05;
            let density = 0.01;

            let snow_mesh = meshes.add(
                shape::UVSphere {
                    radius,
                    sectors: 8,
                    stacks: 4,
                }.into());
            commands.spawn(
                (
                    Name::from("SnowFlake"),
                    Snow::new(10.0),
                    PbrBundle {
                        mesh: snow_mesh,
                        material,
                        ..Default::default()
                    },
                    ExternalForce::new(Vec3::ZERO),
                    Position::new(snow_position),
                    LinearDamping(0.9),
                    RigidBody::Dynamic,
                    MassPropertiesBundle::new_computed(&Collider::ball(radius), density),
                ));
        }
    }
}