use crate::io::asset_loading::MergingAsset;
use bevy::asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset};
use bevy::prelude::Commands;
use bevy::reflect::{TypePath, TypeUuid};
use serde::{Deserialize, Serialize};

use crate::io::config::{AudioConfig, DebugConfig, InputConfig, WorldConfig};

/// This wrapper around the different config types is needed to create a single AssetLoader for
/// all configs. Otherwise we'd need to implement a separate AssetLoader per config type.
#[derive(Debug, Deserialize, Serialize, TypeUuid,TypePath,  Clone)]
#[serde(deny_unknown_fields)]
#[uuid = "fda5258c-c2ee-4975-823d-cd4f9756b380"]
pub enum Config {
    Audio(AudioConfig),
    Debug(DebugConfig),
    Input(InputConfig),
    World(WorldConfig),
}

impl Config {
    pub fn insert(self, commands: &mut Commands) {
        match self {
            Config::Audio(value) => {
                commands.insert_resource(value);
            }
            Config::Debug(value) => {
                commands.insert_resource(value);
            }
            Config::Input(value) => {
                commands.insert_resource(value);
            }
            Config::World(value) => {
                commands.insert_resource(value);
            }
        }
    }
}

impl MergingAsset for Config {
    fn merge(&self, accumulator: Option<Self>) -> Self {
        match self {
            Config::Audio(_) => self.clone(),
            Config::Debug(_) => self.clone(),
            Config::World(_) => self.clone(),
            Config::Input(value) => {
                if let Some(Config::Input(accumulator)) = accumulator {
                    Config::Input(value.merge(Some(accumulator)))
                } else if accumulator.is_none() {
                    Config::Input(value.merge(None))
                } else {
                    panic!(
                        "Something went wrong, accumulator was not the same type as the \
                    merging asset. This indicates a bug in the loader code."
                    );
                }
            }
        }
    }
}

#[derive(Default)]
pub struct ConfigLoader;

impl AssetLoader for ConfigLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let custom_asset = ron::de::from_bytes::<Config>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["config.ron"]
    }
}
