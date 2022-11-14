// use bevy::utils::define_label;
use serde::{Deserialize, Serialize};

// define_label!(
//     /// A strongly-typed class of labels used to identify input actions.
//     InputAction,
//     /// Strongly-typed identifier for an input action.
//     InputActionId,
// );

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
#[serde(deny_unknown_fields)]
pub enum InputAction {
    /// Player movement.
    Forward,
    Backward,
    Left,
    Right,
    Up,
    Down,
    /// Enable faster movement while this is active.
    Sprint,

    /// Pause the day-night cycle. For debugging purposes.
    PauseTime,
    /// Speed up the day-night cycle. For debugging purposes.
    SpeedUpTime,
    /// Speed down the day-night cycle. For debugging purposes.
    SpeedDownTime,
    /// Sets time of day backward in increments of three hours.
    SetTimeBack,
    /// Sets time of day forward in increments of three hours.
    SetTimeForward,
}
