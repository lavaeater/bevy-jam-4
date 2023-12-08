use bevy::app::{App, Plugin, Update};
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::math::Vec3;
use bevy::pbr::PbrBundle;
use bevy::prelude::{Commands, Event, EventReader, EventWriter, GlobalTransform, Query, Res, ResMut, Transform, With, Without};
use bevy_turborand::{DelegatedRng, GlobalRng};
use bevy_xpbd_3d::prelude::CollisionStarted;
use crate::assets::SantasAssets;
use crate::sam_site::{MissileTrail, SamChild, SurfaceToAirMissile};
use crate::santa::{Health, ParentEntity, Santa, SantaChild};

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SpawnExplosionAt>()
            .add_systems(Update, (
                collision_handler,
                spawn_explosions
            ))
        ;
    }
}

#[derive(Event)]
pub struct SpawnExplosionAt {
    pub position: Vec3,
}

fn collision_handler(
    mut collision_reader: EventReader<CollisionStarted>,
    mut explosion_ew: EventWriter<SpawnExplosionAt>,
    mut commands: Commands,
    mut santa_query: Query<&mut Health, With<Santa>>,
    missile_query: Query<(&SurfaceToAirMissile, &GlobalTransform)>,
    santa_child_query: Query<&ParentEntity, (With<SantaChild>, Without<SamChild>)>,
    missile_child_query: Query<&ParentEntity, (With<SamChild>, Without<SantaChild>)>,
    mut global_rng: ResMut<GlobalRng>,
) {
    for collision in collision_reader.read() {
        if (santa_child_query.contains(collision.0) || santa_child_query.contains(collision.1))
            && (missile_child_query.contains(collision.0) || missile_child_query.contains(collision.1))
        {
            let (santa_entity, missile_entity) = if santa_child_query.contains(collision.0) {
                (santa_child_query.get(collision.0).unwrap().0, missile_child_query.get(collision.1).unwrap().0)
            } else {
                (santa_child_query.get(collision.1).unwrap().0, missile_child_query.get(collision.0).unwrap().0)
            };

            if let Ok((_, missile_transform)) = missile_query.get(missile_entity) {
                explosion_ew.send(SpawnExplosionAt {
                    position: missile_transform.translation(),
                });
            }
            commands.entity(missile_entity).despawn_recursive();

            if let Ok(mut santa_health) = santa_query.get_mut(santa_entity) {
                santa_health.health -= global_rng.i32(5..=15);
            }
        }
    }
}

fn spawn_explosions(
    mut commands: Commands,
    mut explosion_reader: EventReader<SpawnExplosionAt>,
    mut global_rng: ResMut<GlobalRng>,
    santas_assets: Res<SantasAssets>,
) {
    for explosion in explosion_reader.read() {
        let explosion_size = global_rng.i32(5..=15);
        for _i in (1..explosion_size) {
            let missile_trail = MissileTrail::new(0.5 * global_rng.f32(), global_rng.f32(), (global_rng.f32() + 0.5) * 10.0);
            commands.spawn((
                PbrBundle {
                    mesh: santas_assets.sphere_mesh.clone(),
                    material: santas_assets.sphere_material.clone(),
                    transform: Transform::from_xyz(
                        explosion.position.x + global_rng.f32_normalized() * 5.0,
                        explosion.position.y + global_rng.f32_normalized() * 5.0,
                        explosion.position.z + global_rng.f32_normalized() * 5.0,
                    )
                        .with_scale(Vec3::new(missile_trail.start_scale, missile_trail.start_scale, missile_trail.start_scale)),
                    ..Default::default()
                },
                missile_trail
            ));
        }
    }
}
