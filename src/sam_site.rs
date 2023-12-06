use bevy::app::{App, Plugin, PostStartup, Update};
use bevy::prelude::{Commands, Res, ResMut, Resource};
use bevy::time::Time;
use crate::assets::SantasAssets;
use crate::input::CoolDown;

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

fn spawn_sam_sites(
    mut sam_site_params: ResMut<SamSiteParams>,
    santas_assets: Res<SantasAssets>,
    time: Res<Time>,
    mut commands: Commands,
) {
    if sam_site_params.cool_down(time.delta_seconds()) {
        commands
            .spawn(santas_assets.turret.clone())
            .insert(SamSite)
            .insert(santas_assets.snowball_mesh.clone())
            .insert(santas_assets.snowball_material.clone())
            .insert(sam_site_params.clone())
        ;
    }
}
