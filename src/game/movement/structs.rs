use bevy::math::{Vec2, Vec3};
use bevy::prelude::Component;

use crate::game::movement::direction::Direction1D;

#[derive(Component, Default)]
pub struct MoveState {
    pub velocity: Vec3,
}

#[derive(Component, Default)]
pub struct MoveParams {
    pub accel: f32,
    pub max_speed: f32,
    pub boost_mod: f32,
    pub max_fall_speed: f32,
    pub vel_decay_factor: f32,
    pub turn_speed: f32,
    pub flying: bool,
}

#[derive(Default)]
pub struct MoveInput {
    pub xy_plane: Vec2,
    pub up_down: Direction1D,
    pub sprint: bool,
}
