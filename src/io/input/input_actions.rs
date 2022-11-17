use serde::{Deserialize, Serialize};

use hex_derive::InputAction;

/// This trait serves to abstract away things like keyboard keys and mouse buttons, the normal
/// game code should be completely agnostic of those specific input types.
///
/// Identifies an action that you can request user input for.
/// Each input action may be bound to multiple combinations of keyboard or mouse input.
///
/// For example, "Forward" can be an InputAction, and it can be bound to the W and Up keys.
/// If either binding triggers (IE if the W-key or the Up-arrow-key is pressed) the InputAction
/// is considered to be active.
///
/// You may derive this trait for enums containing ONLY unit-like enum values.
pub trait InputAction {
    /// The name of the enum. In the config file, all InputActions are grouped under this header.
    fn group_id(&self) -> &'static str;
    /// The enum value.
    fn item_id(&self) -> &'static str;
}

#[derive(InputAction, Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(deny_unknown_fields)]
pub enum MovementInput {
    /// Player movement.
    Forward,
    Backward,
    Left,
    Right,
    Up,
    Down,
    /// Enable faster movement while this is active.
    Sprint,
}

#[derive(InputAction, Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(deny_unknown_fields)]
pub enum DayNightInput {
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
