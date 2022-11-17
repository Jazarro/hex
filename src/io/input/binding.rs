use bevy::prelude::{KeyCode, MouseButton};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
#[serde(deny_unknown_fields)]
pub enum InputBinding {
    /// A single keyboard key without modifiers.
    /// This option is syntactic sugar to make key-binding config files less cluttered.
    Key(KeyCode, SignalState),
    /// A single keyboard key with keyboard modifiers.
    /// The modifiers can technically be any key, though the settings menu might want to
    /// constrain users to the conventional ones (shifts, alts, controls and tab).
    KeyModified(KeyModifiers, KeyCode, SignalState),
    /// A single mouse button without modifiers.
    /// This option is syntactic sugar to make key-binding config files less cluttered.
    Mouse(MouseButton, SignalState),
    /// A single mouse button with keyboard modifiers.
    /// The modifiers can technically be any key, though the settings menu might want to
    /// constrain users to the conventional ones (shifts, alts, controls and tab).
    MouseModified(KeyModifiers, MouseButton, SignalState),
    // TODO: Scroll wheel Up & Down
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(deny_unknown_fields)]
pub enum SignalState {
    /// Key is down, doesn't matter how long it's been down.
    Pressed,
    /// This is the first tick we're detecting this key to be down.
    JustPressed,
    /// This is the first tick we're detecting this key to be up, after at least one tick of it being down.
    JustReleased,
}

/// All modifier keys must be pressed for the modifier to be considered active.
///
/// Examples of conventional modifiers are: LCtrl, LShift, LAlt, RCtrl, RShift, RAlt, Tab,
/// though any keyboard key is technically valid.
pub type KeyModifiers = Vec<KeyCode>;
