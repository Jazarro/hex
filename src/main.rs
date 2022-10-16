#![allow(dead_code)]

use bevy::prelude::*;
use bevy::window::close_on_esc;
use iyes_loopless::prelude::AppLooplessStateExt;

use crate::meshes::debug_lines::LineMaterial;
use crate::states::appstate::AppState;
use crate::states::state_game::GameState;
use crate::states::state_loading::LoadingState;

mod animate_simple;
mod config;
mod meshes;
mod states;
mod coords;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MaterialPlugin::<LineMaterial>::default())
        .add_system(close_on_esc)
        .add_loopless_state(AppState::Loading)
        .add_plugin(LoadingState)
        .add_plugin(GameState)
        .run();
}
