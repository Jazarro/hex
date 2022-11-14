use std::collections::HashMap;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::io::asset_loading::MergingAsset;
use crate::io::input::binding::InputBinding;
use crate::io::input::InputAction;

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct InputConfig {
    pub map: HashMap<InputAction, Vec<InputBinding>>,
}

impl MergingAsset for InputConfig {
    fn merge(&self, accumulator: Option<InputConfig>) -> InputConfig {
        if let Some(accumulator) = accumulator {
            let mut accumulator = accumulator;
            for (key, value) in self.map.iter() {
                let updated = accumulator.map.insert(key.clone(), value.clone());
                if updated.is_some() {
                    debug!(
                        "A mod updated key config for {:?}, new value is {:?}",
                        key, value
                    );
                }
            }
            accumulator
        } else {
            debug!("Loading default key configs");
            self.clone()
        }
    }
}
