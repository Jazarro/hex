use std::collections::HashMap;

use bevy::asset::{AssetLoader, BoxedFuture, Handle, HandleId, LoadContext, LoadedAsset};
use bevy::reflect::TypeUuid;
use serde::{Deserialize, Serialize};

use crate::io::config::Config;

/// Implement for any assets that can merge together different versions from different mods.
/// Instead of wholesale replacing the file with the overriding one from a mod later in the mod
/// loading order, the overriding file will be merged with the accumulator.
///
/// The accumulator is the result of merging versions of the file from mods earlier in the mod
/// loading order.
pub trait MergingAsset {
    fn merge(&self, accumulator: Option<Self>) -> Self
        where
            Self: Sized;
}

/// Used to temporarily store asset handles during loading time.
#[derive(Default)]
pub struct LoaderHandles {
    pub mod_order: Handle<MetaAsset>,
    pub file_structure: Handle<MetaAsset>,
    pub manifests: HashMap<String, Handle<MetaAsset>>,
    /// Maps config name to a list of configs from different mods. It always picks the last one.
    pub configs: HashMap<String, Vec<Handle<Config>>>,
}

impl LoaderHandles {
    pub fn put_config(&mut self, filename: &str, handle: Handle<Config>) {
        if !self.configs.contains_key(filename) {
            self.configs.insert(filename.to_string(), Vec::new());
        }
        self.configs.get_mut(filename).unwrap().push(handle);
    }
    /// Temporary function, used to wait until all assets are loaded.
    /// TODO: Replace with something more sophisticated that can actually report what files are not
    ///     loaded yet and what files errorred while loading.
    pub fn all_handles(&self) -> Vec<HandleId> {
        let mut vec = vec![];
        for key in self.configs.keys() {
            self.configs
                .get(key)
                .unwrap()
                .iter()
                .map(|handle| handle.id)
                .for_each(|id| vec.push(id));
        }
        vec
    }
}

#[derive(Debug, Deserialize, Serialize, TypeUuid, Clone)]
#[serde(deny_unknown_fields)]
#[uuid = "068dac31-04ec-475e-86fb-6a1272c46f88"]
pub enum MetaAsset {
    ModOrder(ModOrder),
    FileStructure(FileStructure),
    Manifest(Manifest),
}

impl MetaAsset {
    pub fn as_mod_order(&self) -> &ModOrder {
        if let Self::ModOrder(value) = self {
            value
        } else {
            panic!("Tried to unwrap MetaAsset as ModOrder but the asset contained something else.");
        }
    }
    pub fn as_mod_order_mut(&mut self) -> &mut ModOrder {
        if let Self::ModOrder(value) = self {
            value
        } else {
            panic!("Tried to unwrap MetaAsset as ModOrder but the asset contained something else.");
        }
    }
    pub fn as_file_structure(&self) -> &FileStructure {
        if let Self::FileStructure(value) = self {
            value
        } else {
            panic!("Tried to unwrap MetaAsset as FileStructure but the asset contained something else.");
        }
    }
    pub fn as_manifest(&self) -> &Manifest {
        if let Self::Manifest(value) = self {
            value
        } else {
            panic!("Tried to unwrap MetaAsset as Manifest but the asset contained something else.");
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ModOrder {
    /// Collection of names of mods in the order they should be loaded.
    pub mods: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct FileStructure {
    /// A list of config files. Each file will be loaded by the ConfigLoader.
    pub configs: Vec<String>,
    /// Maps SfxId.group_id() and SfxId.item_id() to an asset directory of sound effect files.
    pub sfx: HashMap<String, HashMap<String, String>>,
    /// Maps SfxId.group_id() and SfxId.item_id() to an asset directory of music files.
    pub music: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub name: String,
    pub description: String,
    pub load_order_hint: i32,
}

#[derive(Default)]
pub struct MetaLoader;

impl AssetLoader for MetaLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let custom_asset = ron::de::from_bytes::<MetaAsset>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["meta.ron"]
    }
}
