#![forbid(unsafe_code)]
#![allow(dead_code, unused_variables)]

extern crate core;

use bevy::log::{Level, LogSettings};
use bevy::prelude::*;
use bevy::window::close_on_esc;
use iyes_loopless::prelude::AppLooplessStateExt;

use crate::game::meshes::debug_lines::LineMaterial;
use crate::states::appstate::AppState;
use crate::states::state_game::GameState;
use crate::states::state_loading::LoadingState;

mod animate_simple;
mod assets;
mod audio;
mod game;
mod states;

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
        .run();
}
