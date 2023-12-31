use bevy::app::{App, Plugin, Update};
use bevy::hierarchy::{BuildChildren, DespawnRecursiveExt};
use bevy::math::Vec3;
use bevy::pbr::{PbrBundle, PointLight, PointLightBundle};
use bevy::prelude::{Color, Commands, default, Entity, Event, EventReader, EventWriter, GlobalTransform, Query, Res, ResMut, Transform, With, Without};
use bevy_turborand::{DelegatedRng, GlobalRng};
use bevy_xpbd_3d::prelude::CollisionStarted;
use crate::assets::SantasAssets;
use crate::sam_site::{MissileTrail, SamChild, SamSite, SurfaceToAirMissile};
use crate::santa::{GiftChild, SantaStats, ParentEntity, Santa, SantaChild, TargetEvent, TargetEventTypes};
use crate::villages::{House, HouseChild, HouseEvent, HouseEventType, LoadLevel, NeedsGifts, VillageCenter};

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SpawnExplosionAt>()
            .add_event::<LevelFinished>()
            .add_systems(Update, (
                missile_santa_collision_handler,
                gift_house_collision_handler,
                spawn_explosions,
                received_gifts_handler,
                level_finished_handler,
            ))
        ;
    }
}

#[derive(Event)]
pub struct SpawnExplosionAt {
    pub position: Vec3,
}

fn missile_santa_collision_handler(
    mut collision_reader: EventReader<CollisionStarted>,
    mut explosion_ew: EventWriter<SpawnExplosionAt>,
    mut commands: Commands,
    mut santa_query: Query<&mut SantaStats, With<Santa>>,
    missile_query: Query<(&SurfaceToAirMissile, &GlobalTransform)>,
    santa_child_query: Query<&ParentEntity, (With<SantaChild>, Without<SamChild>)>,
    missile_child_query: Query<&ParentEntity, (With<SamChild>, Without<SantaChild>)>,
    mut global_rng: ResMut<GlobalRng>,
) {
    for collision in collision_reader.read() {
        if missile_child_query.contains(collision.0) || missile_child_query.contains(collision.1) {
            if santa_child_query.contains(collision.0) || santa_child_query.contains(collision.1) {
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
}

fn gift_house_collision_handler(
    mut collision_reader: EventReader<CollisionStarted>,
    mut explosion_ew: EventWriter<SpawnExplosionAt>,
    mut commands: Commands,
    missile_query: Query<(&SurfaceToAirMissile, &GlobalTransform)>,
    missile_child_query: Query<&ParentEntity, (With<GiftChild>, Without<HouseChild>)>,
    house_child_query: Query<&ParentEntity, (With<HouseChild>, Without<GiftChild>)>,
    mut house_ew: EventWriter<HouseEvent>,
) {
    for collision in collision_reader.read() {
        if missile_child_query.contains(collision.0) || missile_child_query.contains(collision.1) {
            if house_child_query.contains(collision.0) || house_child_query.contains(collision.1) {
                let (house_child_entity, missile_entity) = if house_child_query.contains(collision.0) {
                    (house_child_query.get(collision.0).unwrap().0, missile_child_query.get(collision.1).unwrap().0)
                } else {
                    (house_child_query.get(collision.1).unwrap().0, missile_child_query.get(collision.0).unwrap().0)
                };
                if let Ok((_, missile_transform)) = missile_query.get(missile_entity) {
                    explosion_ew.send(SpawnExplosionAt {
                        position: missile_transform.translation(),
                    });
                }
                commands.entity(missile_entity).despawn_recursive();

                house_ew.send(HouseEvent(HouseEventType::ReceivedGifts(house_child_entity)));
            }
        }
    }
}

#[derive(Event)]
pub struct LevelFinished(pub u32);

fn received_gifts_handler(
    mut gifts_received_er: EventReader<HouseEvent>,
    house_query: Query<&House>,
    mut village_center_query: Query<&mut VillageCenter>,
    mut commands: Commands,
    mut level_finished_ew: EventWriter<LevelFinished>,
    mut target_event_ew: EventWriter<TargetEvent>
) {
    for gifts_received in gifts_received_er.read() {
        match gifts_received.0 {
            HouseEventType::ReceivedGifts(house_entity) => {
                commands.entity(house_entity).remove::<NeedsGifts>();
                target_event_ew.send(TargetEvent(TargetEventTypes::StopShooting));
                if let Ok(house) = house_query.get(house_entity) {
                    if let Ok(mut village_center) = village_center_query.get_mut(house.belongs_to_village) {
                        village_center.needs_gifts_count -= 1;
                        if village_center.needs_gifts_count <= 0 && village_center.needs_gifts {
                            village_center.needs_gifts = false;
                            level_finished_ew.send(LevelFinished(village_center.level));
                        }
                    }
                }

            }
        }
    }
}

fn level_finished_handler(
    mut level_finished_er: EventReader<LevelFinished>,
    mut load_level_ew: EventWriter<LoadLevel>,
    query: Query<(Entity, &GlobalTransform), With<SamSite>>,
    mut spawn_explosion_at: EventWriter<SpawnExplosionAt>,
    mut commands: Commands,
) {
    for level_finished in level_finished_er.read() {
        load_level_ew.send(LoadLevel(level_finished.0 + 1));
        for (entity, transform) in query.iter() {
            spawn_explosion_at.send(SpawnExplosionAt {
                position: transform.translation(),
            });
            commands.entity(entity).despawn_recursive();
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
        let explosion_size = global_rng.i32(3..=8);
        for _i in 1..explosion_size {
            let missile_trail = MissileTrail::new(0.5 * global_rng.f32(), global_rng.f32(), (global_rng.f32() + 0.5) * 10.0);
            commands.spawn((
                PbrBundle {
                    mesh: santas_assets.trail_mesh.clone(),
                    material: santas_assets.trail_material.clone(),
                    transform: Transform::from_xyz(
                        explosion.position.x + global_rng.f32_normalized() * 5.0,
                        explosion.position.y + global_rng.f32_normalized() * 5.0,
                        explosion.position.z + global_rng.f32_normalized() * 5.0,
                    )
                        .with_scale(Vec3::new(missile_trail.start_scale, missile_trail.start_scale, missile_trail.start_scale)),
                    ..Default::default()
                },
                missile_trail
            )).with_children(|children|
                { // Spawn the child colliders positioned relative to the rigid body
                    children.spawn((
                        PointLightBundle {
                            point_light: PointLight {
                                color: Color::rgb(global_rng.f32(), global_rng.f32(), 0.0),
                                intensity: (global_rng.f32() + 0.5) * 80000.0, // Roughly a 60W non-halogen incandescent bulb
                                range: 40.0,
                                radius: 0.0,
                                shadows_enabled: true,
                                ..default()
                            },
                            ..Default::default()
                        },
                    ));
                });
        }
    }
}
