use bevy::{prelude::{Component, Bundle}, transform::TransformBundle};

use crate::game::{ movement::structs::{MoveState, MoveParams}};

#[derive(Component, Default)]
pub struct Player;

#[derive(Bundle, Default)]
pub struct PlayerBundle{
    pub(crate) player_tag: Player,
    pub(crate) move_params: MoveParams,
    pub(crate) move_state: MoveState,
    #[bundle]
    pub transfrom: TransformBundle,
}