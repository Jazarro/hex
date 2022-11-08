use crate::game::movement::structs::MoveInput;
use bevy::input::Input;
use bevy::prelude::*;

// Removed distinction between flying and walking characters.
// Input is read raw, and appropriate systems will decide how to interpret it.
// -- Festus

pub fn read_movement_input(keys: Res<Input<KeyCode>>, mut input: ResMut<MoveInput>) {
    let left = keys.any_pressed([KeyCode::A, KeyCode::Left]);
    let right = keys.any_pressed([KeyCode::D, KeyCode::Right]);
    let backward = keys.any_pressed([KeyCode::S, KeyCode::Down]);
    let forward = keys.any_pressed([KeyCode::W, KeyCode::Up]);
    input.down_held = keys.any_pressed([KeyCode::LControl, KeyCode::C]);
    input.down_just_pressed = keys.any_just_pressed([KeyCode::LControl, KeyCode::C]);
    input.up_held = keys.any_pressed([KeyCode::Space]);
    input.up_just_pressed = keys.just_pressed(KeyCode::Space);
    input.fast_held = keys.any_pressed([KeyCode::LShift]);
    input.fast_just_pressed = keys.just_pressed(KeyCode::LShift);

    let mut planar_mov = Vec2::new(0.0, 0.0);
    if left {
        planar_mov.x -= 1.0;
    }
    if right {
        planar_mov.x += 1.0;
    }
    if forward {
        planar_mov.y += 1.0;
    }
    if backward {
        planar_mov.y -= 1.0;
    }

    input.xy_plane = planar_mov;
}
