use bevy::app::{App, Plugin};
use bevy::asset::AddAsset;
use bevy::prelude::{in_state, Commands, NextState, Update, ResMut};

use crate::io::asset_loading::*;
use crate::io::config::{Config, ConfigLoader};
use crate::io::input::process_input_bindings;
use crate::states::appstate::AppState;
use bevy::prelude::IntoSystemConfigs;

pub struct LoadingState;

impl Plugin for LoadingState {
    fn build(&self, app: &mut App) {
        app.add_asset::<MetaAsset>()
            .init_asset_loader::<MetaLoader>()
            .add_asset::<Config>()
            .init_asset_loader::<ConfigLoader>()
            .add_state::<LoadProcess>();

        app.add_systems(Update,
            init_load_process
                .run_if(in_state(AppState::Loading))
                .run_if(in_state(LoadProcess::StartLoading)),
        );
        app.add_systems(Update,
            load_mod_order
                .run_if(in_state(AppState::Loading))
                .run_if(in_state(LoadProcess::LoadModOrder)),
        );
        app.add_systems(Update,
            check_mod_order_is_present
                .run_if(in_state(AppState::Loading))
                .run_if(in_state(LoadProcess::WaitForModOrder)),
        );
        app.add_systems(Update,
            load_manifests
                .run_if(in_state(AppState::Loading))
                .run_if(in_state(LoadProcess::LoadManifests)),
        );
        app.add_systems(Update,
            check_manifests_are_present
                .run_if(in_state(AppState::Loading))
                .run_if(in_state(LoadProcess::WaitForManifests)),
        );
        app.add_systems(Update,
            load_files
                .run_if(in_state(AppState::Loading))
                .run_if(in_state(LoadProcess::LoadFiles)),
        );
        app.add_systems(Update,
            check_files_are_present
                .run_if(in_state(AppState::Loading))
                .run_if(in_state(LoadProcess::WaitForFiles)),
        );
        app.add_systems(Update,
            resolve_mods
                .run_if(in_state(AppState::Loading))
                .run_if(in_state(LoadProcess::ResolveMods)),
        );
        app.add_systems(Update,
            (exit_loading_state, process_input_bindings)
                .run_if(in_state(AppState::Loading))
                .run_if(in_state(LoadProcess::DoneLoading)),
        );
    }
}

fn init_load_process(mut commands: Commands) {
    commands.insert_resource(NextState(Some(LoadProcess::LoadModOrder)));
}

fn exit_loading_state(commands: Commands, mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::Game);
}
