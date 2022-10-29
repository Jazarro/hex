use std::collections::HashMap;

use bevy::prelude::{info, KeyCode};
use serde::{Deserialize, Serialize};

use crate::assets::loading::meta::MergingAsset;

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct KeysConfig {
    pub map: HashMap<InputAction, Vec<KeyCode>>,
}

impl MergingAsset for KeysConfig {
    fn merge(&self, accumulator: Option<KeysConfig>) -> KeysConfig {
        if let Some(accumulator) = accumulator {
            let mut merged = accumulator.clone();
            for (key, value) in self.map.iter() {
                let updated = merged.map.insert(key.clone(), value.clone());
                if updated.is_some() {
                    info!(
                        "A mod updated key config for {:?}, new value is {:?}",
                        key, value
                    );
                }
            }
            merged
        } else {
            info!("Loading default key configs: {:?}", self.map);
            self.clone()
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
#[serde(deny_unknown_fields)]
pub enum InputAction {
    Forward,
    Backward,
    Up,
    Down,
}
