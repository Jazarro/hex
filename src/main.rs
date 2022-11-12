#![forbid(unsafe_code)]
#![allow(
    dead_code,
    unused_variables,
    clippy::type_complexity,
    clippy::too_many_arguments
)]

extern crate core;

#[cfg(feature = "debugwindow")]
use crate::debug_window::DebugWindowPlugin;

use bevy::log::{Level, LogSettings};
use bevy::prelude::*;
use bevy::window::close_on_esc;
use iyes_loopless::prelude::{AppLooplessStateExt, CurrentState};

use crate::game::meshes::debug_lines::LineMaterial;
use crate::game::movement::structs::MoveInput;
use crate::states::appstate::AppState;
use crate::states::state_game::GameState;
use crate::states::state_loading::LoadingState;
use crate::window_event_handler::handle_window;

mod animate_simple;
mod assets;
mod audio;
mod debug_window;
mod game;
mod states;
mod window_event_handler;

fn main() {
    let mut app = App::new();
    // this code is compiled only if debug assertions are enabled (debug mode)
    #[cfg(debug_assertions)]
    app.insert_resource(LogSettings {
        // This filter sets everything to info, except for some crates that are further specified:
        // - wgpu is super spammy and restricted to error
        // - symphonia occasionally logs spam to info level so is restricted to warn
        // - Our own crate hex is set to debug, so is actually allowed to be more spammy than the default info level.
        filter: "info,wgpu=error,symphonia_core=warn,symphonia_format_ogg=warn,symphonia_codec_vorbis=warn,symphonia_bundle_mp3=warn,hex=debug".into(),
        level: Level::TRACE,
    });
    // this code is compiled only if debug assertions are disabled (release mode)
    #[cfg(not(debug_assertions))]
    app.insert_resource(LogSettings {
        filter: "error".into(),
        level: Level::ERROR,
    });
    app.add_plugins(DefaultPlugins)
        .add_plugin(MaterialPlugin::<LineMaterial>::default())
        .add_system(close_on_esc)
        .add_loopless_state(AppState::Loading)
        .add_plugin(LoadingState)
        .add_plugin(GameState)
        .add_system(handle_window)
        .add_system(log_state_changes);
    // Show debug window only if the debugwindow feature is enabled:
    #[cfg(feature = "debugwindow")]
    app.add_plugin(DebugWindowPlugin);
    app.run();
}

pub fn log_state_changes(state: Res<CurrentState<AppState>>) {
    if state.is_changed() {
        info!("Switching to game state {:?}!", state.0);
    }
}
