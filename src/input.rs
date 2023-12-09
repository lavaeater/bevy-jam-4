use bevy::app::{App, Plugin, Update};
use bevy::input::ButtonState;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::{Component, EventReader, KeyCode, Query, Res, With};
use bevy::reflect::Reflect;
use bevy::time::Time;
use bevy::utils::HashSet;
use bevy_xpbd_3d::components::{AngularVelocity, LinearVelocity, Rotation};
use bevy_xpbd_3d::math::Vector3;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update, (
                    input_control,
                    kinematic_movement,
                    dynamic_movement,
                ),
            );
    }
}

pub trait CoolDown {
    /// Returns true if the cool down is finished
    fn cool_down(&mut self, delta: f32) -> bool;
}

#[derive(Component, Reflect)]
pub struct KeyboardController {}

#[derive(Hash, PartialEq, Eq, Clone, Reflect)]
pub enum ControlCommands {
    FirePrimary,
    Jump,
    Build,
}


#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug, Reflect)]
pub enum ControlRotation {
    Left,
    Right,
}

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug, Reflect)]
pub enum ControlDirection {
    Forward,
    Backward,
    StrafeLeft,
    StrafeRight,
}

pub trait Opposite {
    fn opposite(&self) -> Self;
}

impl Opposite for ControlDirection {
    fn opposite(&self) -> Self {
        match self {
            ControlDirection::Forward => ControlDirection::Backward,
            ControlDirection::Backward => ControlDirection::Forward,
            ControlDirection::StrafeLeft => ControlDirection::StrafeRight,
            ControlDirection::StrafeRight => ControlDirection::StrafeLeft,
        }
    }
}

impl Opposite for ControlRotation {
    fn opposite(&self) -> Self {
        match self {
            ControlRotation::Left => ControlRotation::Right,
            ControlRotation::Right => ControlRotation::Left,
        }
    }
}

#[derive(Component, Reflect)]
pub struct Controller {
    pub triggers: HashSet<ControlCommands>,
    pub rotations: HashSet<ControlRotation>,
    pub directions: HashSet<ControlDirection>,
    pub has_thrown: bool,
    pub speed: f32,
    pub acceleration: f32,
    pub max_speed: f32,
    pub turn_speed: f32,
    pub max_turn_speed: f32,
    pub rate_of_fire_per_minute: f32,
    pub fire_cool_down: f32,
}

impl Controller {
    pub fn new(speed: f32, acceleration: f32, turn_speed: f32, rate_of_fire_per_minute: f32) -> Self {
        Self {
            triggers: HashSet::default(),
            rotations: HashSet::default(),
            directions: HashSet::default(),
            has_thrown: false,
            speed,
            acceleration,
            max_speed: speed,
            turn_speed,
            max_turn_speed: turn_speed,
            rate_of_fire_per_minute,
            fire_cool_down: 0.0,
        }
    }
}

impl CoolDown for Controller {
    fn cool_down(&mut self, delta: f32) -> bool {
        self.fire_cool_down -= delta;
        if self.fire_cool_down <= 0.0 {
            self.fire_cool_down = 60.0 / self.rate_of_fire_per_minute;
            return true;
        }
        false
    }
}


#[derive(Component)]
pub struct DynamicMovement {}


#[derive(Component)]
pub struct KinematicMovement {}

pub fn input_control(
    mut key_evr: EventReader<KeyboardInput>,
    mut query: Query<&mut Controller, With<KeyboardController>>,
) {
    if let Ok(mut controller) = query.get_single_mut() {
        for ev in key_evr.read() {
            match ev.state {
                ButtonState::Pressed => match ev.key_code {
                    Some(KeyCode::B) => {
                        if controller.triggers.contains(&ControlCommands::Build) {} else {
                            controller.triggers.insert(ControlCommands::Build);
                        }
                    }
                    Some(KeyCode::Escape) => {
                        if controller.triggers.contains(&ControlCommands::Build) {}
                    }
                    Some(KeyCode::A) => {
                        controller.rotations.insert(ControlRotation::Left);
                    }
                    Some(KeyCode::D) => {
                        controller.rotations.insert(ControlRotation::Right);
                    }
                    Some(KeyCode::W) => {
                        controller.directions.insert(ControlDirection::Forward);
                    }
                    Some(KeyCode::S) => {
                        controller.directions.insert(ControlDirection::Backward);
                    }
                    Some(KeyCode::Space) => {
                        if controller.triggers.contains(&ControlCommands::FirePrimary) {
                            controller.triggers.remove(&ControlCommands::FirePrimary);
                        } else {
                            controller.triggers.insert(ControlCommands::FirePrimary);
                        }
                    }
                    _ => {}
                },
                ButtonState::Released => match ev.key_code {
                    Some(KeyCode::A) => {
                        controller.rotations.remove(&ControlRotation::Left);
                    }
                    Some(KeyCode::D) => {
                        controller.rotations.remove(&ControlRotation::Right);
                    }
                    Some(KeyCode::W) => {
                        controller.directions.remove(&ControlDirection::Forward);
                    }
                    Some(KeyCode::S) => {
                        controller.directions.remove(&ControlDirection::Backward);
                    }
                    Some(KeyCode::Left) => {}
                    Some(KeyCode::Right) => {}
                    _ => {}
                }
            }
            if controller.directions.is_empty() && controller.rotations.is_empty() {}
        }
    }
}

pub fn dynamic_movement(
    mut query: Query<(&mut LinearVelocity, &mut AngularVelocity, &Rotation, &Controller), With<DynamicMovement>>,
) {
    for (mut linear_velocity, mut angular_velocity, rotation, controller) in query.iter_mut() {
        let mut force = Vector3::ZERO;
        let mut torque = Vector3::ZERO;

        if controller.directions.contains(&ControlDirection::Forward) {
            force.z = -1.0;
        }
        if controller.directions.contains(&ControlDirection::Backward) {
            force.z = 1.0;
        }
        if controller.rotations.contains(&ControlRotation::Left) {
            torque.y = 1.0;
        }
        if controller.rotations.contains(&ControlRotation::Right) {
            torque.y = -1.0;
        }
        force = rotation.mul_vec3(force) * controller.speed;
        linear_velocity.x = force.x;
        linear_velocity.z = force.z;
        angular_velocity.0 = torque * controller.turn_speed;
    }
}


pub fn kinematic_movement(
    mut query: Query<(&mut LinearVelocity, &mut AngularVelocity, &Rotation, &mut Controller), With<KinematicMovement>>,
    time: Res<Time>
) {
    for (
        mut linear_velocity,
        mut angular_velocity,
        rotation,
        mut controller) in query.iter_mut() {
        let mut force = Vector3::ZERO;
        let mut torque = Vector3::ZERO;

        if controller.directions.contains(&ControlDirection::Forward) {
            force.z = 1.0;
        }
        if controller.directions.contains(&ControlDirection::Backward) {
            force.z = -1.0;
        }
        if controller.rotations.contains(&ControlRotation::Left) {
            torque.y = 1.0;
        }
        if controller.rotations.contains(&ControlRotation::Right) {
            torque.y = -1.0;
        }
        force = rotation.mul_vec3(force);


        controller.speed += controller.acceleration * time.delta_seconds();
        if controller.speed > controller.max_speed {
            controller.speed = controller.max_speed;
        }

        linear_velocity.0 = force * controller.speed;
        angular_velocity.0 = torque * controller.turn_speed;
    }
}