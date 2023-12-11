use belly::build::{eml, FromWorldAndParams, widget, WidgetContext};
use belly::core::eml::Params;
use belly::prelude::*;
use bevy::prelude::*;
use bevy::app::{App, Plugin, Startup};
use bevy::prelude::{Commands, Entity, Event, EventReader};
use crate::camera::GameCamera;
use crate::santa::{TargetEvent, TargetEventTypes};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(BellyPlugin)
            .insert_resource(UiResources {
                target_color: Color::RED,
            })
            .add_systems(
                Startup,
                spawn_ui,
            )
            .add_systems(
                Update, (
                    target_indicator_system,
                    fellow_system),
            )
        ;
    }
}


#[derive(Resource)]
pub struct UiResources {
    pub target_color: Color,
}

pub fn spawn_ui(mut commands: Commands) {
    commands.add(ess! {
        body {
            // Use the CSS Grid algorithm for laying out this node
            display: grid;
            // Set the grid to have 2 columns with sizes [min-content, minmax(0, 1fr)]
            // - The first column will size to the size of it's contents
            // - The second column will take up the remaining available space
            grid-template-columns: 100%;//min-content; // flex(1.0)
            // Set the grid to have 3 rows with sizes [auto, minmax(0, 1fr), 20px]
            // - The first row will size to the size of it's contents
            // - The second row take up remaining available space (after rows 1 and 3 have both been sized)
            // - The third row will be exactly 20px high
            grid-template-rows: 20% 60% 20%;
            // background-color: white;
        }
        .header {
            // Make this node span two grid columns so that it takes up the entire top tow
            // grid-column: span 2;
            height: 100%;
            font: bold;
            font-size: 8px;
            color: black;
            display: grid;
            padding: 6px;
        }
        .main {
            // Use grid layout for this node
            display: grid;
            height: 100%;
            width: 100%;
            padding: 24px;
            // grid-template-columns: repeat(4, flex(1.0));
            // grid-template-rows: repeat(4, flex(1.0));
            // row-gap: 12px;
            // column-gap: 12px;
            // background-color: #2f2f2f;
        }
        // Note there is no need to specify the position for each grid item. Grid items that are
        // not given an explicit position will be automatically positioned into the next available
        // grid cell. The order in which this is performed can be controlled using the grid_auto_flow
        // style property.
        .cell {
            display: grid;
        }
        // .sidebar {
        //     display: grid;
        //     background-color: black;
        //     // Align content towards the start (top) in the vertical axis
        //     align-items: start;
        //     // Align content towards the center in the horizontal axis
        //     justify-items: center;
        //     padding: 10px;
        //     // Add an fr track to take up all the available space at the bottom of the column so
        //     // that the text nodes can be top-aligned. Normally you'd use flexbox for this, but
        //     // this is the CSS Grid example so we're using grid.
        //     grid-template-rows: auto auto 1fr;
        //     row-gap: 10px;
        //     height: 5%;
        // }
        .text-header {
            font: bold;
            font-size: 24px;
        }
        .footer {
            font: bold;
            font-size: 24px;
            display: grid;
            height: 100%;
            width: 100%;
            padding: 24px;
            grid-template-columns: repeat(4, flex(1.0));
            grid-template-rows: repeat(4, flex(1.0));
            row-gap: 12px;
            column-gap: 12px;
            background-color: #2f2f2faa;
        }
    });
    commands.add(eml! {
        <body>
            <span c:header></span>
            <span c:main>
            </span>
            <span c:footer id="ui-footer">
                // <for color in=COLORS>
                //     <span c:cell s:background-color=color/>
                // </for>
            </span>
        </body>
    });
}

#[derive(Event)]
pub struct AddHealthBar {
    pub entity: Entity,
    pub name: &'static str,
}

pub fn target_indicator_system(
    mut elements: Elements,
    mut target_er: EventReader<TargetEvent>,
) {
    for target_aqcuired in &mut target_er.read() {
        match target_aqcuired.0 {
            TargetEventTypes::Acquired(house) => {
                elements.select("body").add_child(eml! {
                <fellow target=house c:target_indicator>
                    <span c:target_child><label s:color="#ff0000" value="TARGET"/></span>
                </fellow>
        });
            }
            TargetEventTypes::Lost => {
                elements.select(".target_indicator").remove();
            }
            _ => {}
        }
    }
}

#[derive(Component)]
pub struct Fellow {
    pub target: Entity,
}


#[widget]
#[param(target: Entity => Fellow: target)]
fn fellow(ctx: &mut WidgetContext) {
    let content = ctx.content();
    ctx.render(eml! {
        <span s:left=managed() s:top=managed() s:position-type="absolute">
            {content}
        </span>
    })
}

impl FromWorldAndParams for Fellow {
    fn from_world_and_params(_: &mut World, params: &mut Params) -> Self {
        Fellow {
            target: params.try_get("target").expect("Missing required `target` param")
        }
    }
}

pub fn fellow_system(
    mut fellows: Query<(Entity, &Fellow, &mut Style, &Node)>,
    transforms: Query<&GlobalTransform>,
    mut commands: Commands,
    camera_q: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
) {
    if let Ok((camera, camera_global_transform)) = camera_q.get_single() {
        for (entity, follow, mut style, node) in fellows.iter_mut() {
            let Ok(tr) = transforms.get(follow.target) else {
                commands.entity(entity).despawn_recursive();
                continue;
            };
            if let Some(pos) = camera.world_to_viewport(camera_global_transform, tr.translation()) {
                style.left = Val::Px((pos.x - 0.5 * node.size().x).round());
                style.top = Val::Px((pos.y - 0.5 * node.size().y).round());
            }
        }
    }
}