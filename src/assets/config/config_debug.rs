use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct DebugConfig {
    pub origin_lines_display: OriginLinesDisplay,
    pub origin_lines_length: f32,
    pub origin_lines_cone_scale: f32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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
