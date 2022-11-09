use bevy::{
    prelude::{Bundle, Component},
    transform::TransformBundle,
};

use crate::game::movement::structs::{MoveParams, MoveState};

#[derive(Component, Default)]
pub struct Player;

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub(crate) player_tag: Player,
    pub(crate) move_params: MoveParams,
    pub(crate) move_state: MoveState,
    #[bundle]
    pub transform: TransformBundle,
}
