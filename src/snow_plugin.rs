use bevy::app::{App, FixedUpdate, Plugin, Startup};
use bevy::asset::{Assets};
use bevy::core::Name;
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::math::Vec3;
use bevy::pbr::{PbrBundle, StandardMaterial};
use bevy::prelude::{Color, Commands, Component, Entity, Fixed, Mesh, Query, Res, ResMut, shape, Time, With};
use bevy_xpbd_3d::components::{Collider, Inertia, LinearDamping, Position, RigidBody};
use bevy_xpbd_3d::parry::transformation::ConvexHullError::InternalError;
use bevy_xpbd_3d::prelude::{Mass, MassPropertiesBundle};
use crate::input::CoolDown;
use crate::santa::Santa;

pub struct SnowPlugin;

impl Plugin for SnowPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Time::<Fixed>::from_seconds(0.05))
            .add_systems(
                FixedUpdate, (
                    spawn_snow,
                ));
    }
}

#[derive(Component)]
pub struct Snow {
    time_to_live: f32,
    time_left: f32
}

impl Snow {
    pub fn new(time_to_live: f32) -> Self {
        Self {
            time_to_live,
            time_left: time_to_live
        }
    }
}

impl CoolDown for Snow {
    fn cool_down(&mut self, delta: f32) -> bool {
        self.time_left -= delta;
        self.time_left <= 0.0
    }
}

fn kill_snow(
    mut commands: Commands,
    mut snow_query: Query<(Entity, &mut Snow)>,
    time: Res<Time>
) {
    for (entity, mut snow) in snow_query.iter_mut() {
        if snow.cool_down(time.delta_seconds())  {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn spawn_snow(
    mut commands: Commands,
    where_is_santa: Query<&Position, With<Santa>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Ok(santa_position) = where_is_santa.get_single() {
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
                Snow {},
                PbrBundle {
                    mesh: snow_mesh,
                    material,
                    ..Default::default()
                },
                Position::new(santa_position.0 + Vec3::new(-2.0,0.0,0.0)),
                LinearDamping(0.9),
                RigidBody::Dynamic,
                MassPropertiesBundle::new_computed(&Collider::ball(radius), density),
            ));
    }
}