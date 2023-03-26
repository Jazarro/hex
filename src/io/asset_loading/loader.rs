use crate::io::asset_loading::meta::{LoaderHandles, MetaAsset};
use crate::io::asset_loading::MergingAsset;
use crate::io::config::Config;
use bevy::asset::{AssetServer, LoadState};
use bevy::prelude::*;

/// These are sub-states that the loading state works through.
#[derive(Default, Debug, Clone, Eq, PartialEq, Hash, States)]
pub enum LoadProcess {
    #[default]
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

/// Executes during LoadingState if LoadOrder isn't present as resource.
pub fn load_mod_order(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut next_state: ResMut<NextState<LoadProcess>>,
) {
    info!("Loading mod_order.meta.ron");
    info!("Loading file_structure.meta.ron");
    let handles = LoaderHandles {
        mod_order: server.load("mod_order.meta.ron"),
        file_structure: server.load("file_structure.meta.ron"),
        ..default()
    };
    commands.insert_resource(handles);
    next_state.set(LoadProcess::WaitForModOrder);
}

pub fn check_mod_order_is_present(
    mut commands: Commands,
    handles: Res<LoaderHandles>,
    server: Res<AssetServer>,
    mut next_state: ResMut<NextState<LoadProcess>>,
) {
    let mod_order_loaded = match server.get_load_state(&handles.mod_order) {
        LoadState::Loaded => true,
        LoadState::Failed => {
            error!("Failed to load mod_order.meta.ron!");
            false
        }
        _ => false,
    };
    let manifest_loaded = match server.get_load_state(&handles.file_structure) {
        LoadState::Loaded => true,
        LoadState::Failed => {
            error!("Failed to load file_structure.meta.ron!");
            false
        }
        _ => false,
    };
    if mod_order_loaded && manifest_loaded {
        next_state.set(LoadProcess::LoadManifests);
    }
}

pub fn load_manifests(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut handles: ResMut<LoaderHandles>,
    assets: Res<Assets<MetaAsset>>,
    mut next_state: ResMut<NextState<LoadProcess>>,
) {
    info!("Loading mod manifests...");
    let mod_order = assets.get(&handles.mod_order);
    let mod_order = mod_order
        .expect("mod_order.meta.ron wasn't loaded (yet)!")
        .as_mod_order();
    for mod_name in mod_order.mods.iter() {
        info!("\t...loading mods/{}/manifest.meta.ron", mod_name);
        handles.manifests.insert(
            mod_name.to_string(),
            server.load(format!("{}/manifest.meta.ron", mod_name)),
        );
    }
    next_state.set(LoadProcess::WaitForManifests);
}

pub fn check_manifests_are_present(
    mut commands: Commands,
    handles: Res<LoaderHandles>,
    server: Res<AssetServer>,
    mut assets: ResMut<Assets<MetaAsset>>,
    mut next_state: ResMut<NextState<LoadProcess>>,
) {
    let mod_order = assets.get_mut(&handles.mod_order);
    let mod_order = mod_order
        .expect("mod_order.meta.ron wasn't loaded (yet)!")
        .as_mod_order_mut();
    let all_finished = handles.manifests.keys().fold(true, |acc, mod_name| {
        match server.get_load_state(handles.manifests.get(mod_name).unwrap()) {
            LoadState::Loaded => acc,
            LoadState::Failed => {
                error!(
                    "Failed to load a {}'s mod manifest, skipping that mod...",
                    mod_name
                );
                if let Some((index, _)) = mod_order
                    .mods
                    .iter()
                    .enumerate()
                    .find(|(_, item)| item == &mod_name)
                {
                    mod_order.mods.remove(index);
                }
                acc
            }
            _ => false,
        }
    });
    if all_finished {
        next_state.set(LoadProcess::LoadFiles);
    }
}

pub fn load_files(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut handles: ResMut<LoaderHandles>,
    assets: Res<Assets<MetaAsset>>,
    mut next_state: ResMut<NextState<LoadProcess>>,
) {
    let mod_order = assets.get(&handles.mod_order);
    let mod_order = mod_order
        .expect("mod_order.meta.ron wasn't loaded (yet)!")
        .as_mod_order();
    let file_structure = assets.get(&handles.file_structure);
    let file_structure = file_structure
        .expect("file_structure.meta.ron wasn't loaded (yet)!")
        .as_file_structure();
    for mod_name in mod_order.mods.iter() {
        for filename in file_structure.configs.iter() {
            let path = format!("{}/{}", mod_name, filename);
            if server.asset_io().get_metadata((path).as_ref()).is_ok() {
                // Path must exist, so we'll try to load it.
                handles.put_config(filename, server.load(&path));
            }
        }
    }
    next_state.set(LoadProcess::WaitForFiles);
}

pub fn check_files_are_present(
    mut commands: Commands,
    handles: Res<LoaderHandles>,
    server: Res<AssetServer>,
    mut next_state: ResMut<NextState<LoadProcess>>,
) {
    match server.get_group_load_state(handles.all_handles()) {
        LoadState::Loaded => {
            next_state.set(LoadProcess::ResolveMods);
        }
        LoadState::Failed => {
            error!("Failed to load assets!");
        }
        _ => (),
    };
}

pub fn resolve_mods(
    mut commands: Commands,
    handles: Res<LoaderHandles>,
    meta_assets: Res<Assets<MetaAsset>>,
    configs: Res<Assets<Config>>,
    mut next_state: ResMut<NextState<LoadProcess>>,
) {
    let file_structure = meta_assets.get(&handles.file_structure);
    let file_structure = file_structure
        .expect("file_structure.meta.ron wasn't loaded (yet)!")
        .as_file_structure();
    for config_type in file_structure.configs.iter() {
        let config = handles
            .configs
            .get(config_type)
            .unwrap()
            .iter()
            .map(Some)
            .fold(None, |acc, handle| {
                let config = configs
                    .get(handle.unwrap())
                    .expect("A config wasn't loaded (yet)!");
                Some(config.merge(acc))
            });
        if let Some(config) = config {
            config.insert(&mut commands);
        } else {
            panic!("Failed to load resource {}.", config_type);
        }
    }
    next_state.set(LoadProcess::DoneLoading);
}
