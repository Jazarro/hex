use bevy::input::Input;
use bevy::prelude::*;

/// Process input for a walking character.
pub fn input_movement_walk() {}

/// Process input for a flying character.
pub fn input_movement_floating(keys: Res<Input<KeyCode>>) {
    let left = keys.any_pressed([KeyCode::A, KeyCode::Left]);
    let right = keys.any_pressed([KeyCode::D, KeyCode::Right]);
    let backward = keys.any_pressed([KeyCode::S, KeyCode::Down]);
    let forward = keys.any_pressed([KeyCode::W, KeyCode::Up]);
    let down = keys.any_pressed([KeyCode::LControl, KeyCode::C]);
    let up = keys.any_pressed([KeyCode::Space]);
    let fast = keys.any_pressed([KeyCode::LShift]);
}
