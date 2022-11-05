use bevy::prelude::{Query, Res, With};
use crate::game::actors::structs::Player;
use crate::game::movement::structs::{MoveState, MoveParams};
use crate::MoveInput;

// note: this can easily be modified to move all actors that require precise control
pub fn player_movement_system(mut q: Query<(&mut MoveState, &mut MoveParams), With<Player>>, input: Res<MoveInput>){
    
}