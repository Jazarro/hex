use crate::game::meshes::debug_lines::OriginLinesDisplay;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct DebugConfig {
    pub origin_lines_display: OriginLinesDisplay,
    pub origin_lines_length: f32,
    pub origin_lines_cone_scale: f32,
    pub pause_time_at_game_start: bool,
}
