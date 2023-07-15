use crate::game::hex_grid::chunk_loading::ChunkLoader;
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
    pub(crate) chunk_loader: ChunkLoader,
    pub transform: TransformBundle,
}
