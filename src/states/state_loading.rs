use bevy::app::{App, Plugin};
use bevy::asset::AddAsset;
use bevy::prelude::Commands;
use iyes_loopless::condition::ConditionSet;
use iyes_loopless::prelude::*;

use crate::assets::config::configs::{Config, ConfigLoader};
use crate::assets::loading::loader::{
    check_files_are_present, check_manifests_are_present, check_mod_order_is_present, load_files,
    load_manifests, load_mod_order, resolve_mods,
};
use crate::assets::loading::meta::{MetaAsset, MetaLoader};
use crate::states::appstate::AppState;

pub struct LoadingState;

impl Plugin for LoadingState {
    fn build(&self, app: &mut App) {
        app.add_asset::<MetaAsset>()
            .init_asset_loader::<MetaLoader>()
            .add_asset::<Config>()
            .init_asset_loader::<ConfigLoader>()
            .add_loopless_state(LoadProcess::StartLoading)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(AppState::Loading)
                    .run_in_state(LoadProcess::StartLoading)
                    .with_system(init_load_process)
                    .into(),
            )
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(AppState::Loading)
                    .run_in_state(LoadProcess::LoadModOrder)
                    .with_system(load_mod_order)
                    .into(),
            )
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(AppState::Loading)
                    .run_in_state(LoadProcess::WaitForModOrder)
                    .with_system(check_mod_order_is_present)
                    .into(),
            )
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(AppState::Loading)
                    .run_in_state(LoadProcess::LoadManifests)
                    .with_system(load_manifests)
                    .into(),
            )
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(AppState::Loading)
                    .run_in_state(LoadProcess::WaitForManifests)
                    .with_system(check_manifests_are_present)
                    .into(),
            )
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(AppState::Loading)
                    .run_in_state(LoadProcess::LoadFiles)
                    .with_system(load_files)
                    .into(),
            )
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(AppState::Loading)
                    .run_in_state(LoadProcess::WaitForFiles)
                    .with_system(check_files_are_present)
                    .into(),
            )
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(AppState::Loading)
                    .run_in_state(LoadProcess::ResolveMods)
                    .with_system(resolve_mods)
                    .into(),
            )
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(AppState::Loading)
                    .run_in_state(LoadProcess::DoneLoading)
                    .with_system(exit_loading_state)
                    .into(),
            );
    }
}

/// These are sub-states that the loading state works through.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum LoadProcess {
    StartLoading,
    LoadModOrder,
    WaitForModOrder,
    LoadManifests,
    WaitForManifests,
    LoadFiles,
    WaitForFiles,
    ResolveMods,
    DoneLoading,
}

fn init_load_process(mut commands: Commands) {
    commands.insert_resource(NextState(LoadProcess::LoadModOrder));
}

fn exit_loading_state(mut commands: Commands) {
    commands.insert_resource(NextState(AppState::Game));
}
