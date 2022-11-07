use bevy::asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset};
use bevy::prelude::Commands;
use bevy::reflect::TypeUuid;
use serde::{Deserialize, Serialize};

use crate::assets::config::config_audio::AudioConfig;
use crate::assets::config::config_debug::DebugConfig;
use crate::assets::config::config_keys::KeysConfig;
use crate::assets::config::config_world::WorldConfig;
use crate::assets::loading::meta::MergingAsset;

/// This wrapper around the different config types is needed to create a single AssetLoader for
/// all configs. Otherwise we'd need to implement a separate AssetLoader per config type.
#[derive(Debug, Deserialize, Serialize, TypeUuid, Clone)]
#[serde(deny_unknown_fields)]
#[uuid = "fda5258c-c2ee-4975-823d-cd4f9756b380"]
pub enum Config {
    Audio(AudioConfig),
    Debug(DebugConfig),
    Keys(KeysConfig),
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
            Config::Keys(value) => {
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
            Config::Keys(value) => {
                if let Some(Config::Keys(accumulator)) = accumulator {
                    Config::Keys(value.merge(Some(accumulator)))
                } else if let None = accumulator {
                    Config::Keys(value.merge(None))
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
