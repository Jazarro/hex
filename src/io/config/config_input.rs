use std::collections::HashMap;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::io::asset_loading::MergingAsset;
use crate::io::input::binding::InputBinding;

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct InputConfig {
    pub bindings: HashMap<String, HashMap<String, Vec<InputBinding>>>,
}

impl MergingAsset for InputConfig {
    fn merge(&self, accumulator: Option<InputConfig>) -> InputConfig {
        if let Some(accumulator) = accumulator {
            let mut accumulator = accumulator;
            for (key, map) in self.bindings.iter() {
                if !accumulator.bindings.contains_key(key) {
                    accumulator.bindings.insert(key.clone(), HashMap::default());
                }
                for (inner_key, value) in map.iter() {
                    let updated = accumulator
                        .bindings
                        .get_mut(key)
                        .unwrap()
                        .insert(inner_key.clone(), value.clone());
                    if updated.is_some() {
                        debug!(
                            "A mod updated key config for {:?}, new value is {:?}",
                            key.as_str(),
                            map
                        );
                    }
                }
            }
            accumulator
        } else {
            debug!("Loading default key configs");
            self.clone()
        }
    }
}
