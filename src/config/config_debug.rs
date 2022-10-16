use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;

use bevy::prelude::error;
use serde::{Deserialize, Serialize};

use crate::config::file_utils::{get_config_default_dir, get_config_override_dir};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DebugConfig {
    pub origin_lines_display: OriginLinesDisplay,
    pub origin_lines_length: f32,
    pub origin_lines_cone_scale: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum OriginLinesDisplay {
    /// Don't display at all.
    Disabled,
    /// Display lines going into the positive directions.
    Positive,
    /// Display lines going into both positive and negative directions.
    Both,
}

impl Default for OriginLinesDisplay {
    fn default() -> Self {
        OriginLinesDisplay::Positive
    }
}

impl DebugConfig {
    #[must_use]
    pub fn load_from_file() -> DebugConfig {
        let override_file = get_config_override_dir().join("debug.ron");
        if override_file.exists() {
            load_from_path(&override_file)
        } else {
            load_from_path(&get_config_default_dir().join("debug.ron"))
        }
    }
}

fn load_from_path(path: &Path) -> DebugConfig {
    fs::read_to_string(path)
        .and_then(|data| ron::de::from_str::<DebugConfig>(&data).map_err(|error| Error::new(ErrorKind::Other, error)))
        .unwrap_or_else(|error| {
            error!(
                    "Failed to load the debug config file from {:?}! Falling back to DebugConfig::default(). Error: {:?}",
                    path, error
                );
            DebugConfig::default()
        })
}
