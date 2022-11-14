#![forbid(unsafe_code)]
#![allow(
    dead_code,
    unused_variables,
    clippy::type_complexity,
    clippy::too_many_arguments
)]

extern crate core;

#[cfg(feature = "debugwindow")]
use crate::io::window::DebugWindowPlugin;

use crate::game::hex_grid::axial::{ChunkId, Pos};
use crate::game::meshes::debug_lines::LineMaterial;
use crate::io::window::handle_window;
use crate::states::*;
use bevy::log::{Level, LogSettings};
use bevy::prelude::*;
use bevy::window::close_on_esc;
use iyes_loopless::prelude::{AppLooplessStateExt, CurrentState};

mod game;
mod io;
mod states;

fn main() {
    // This is just setting up some static constants:
    ChunkId::setup();
    Pos::setup();

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
