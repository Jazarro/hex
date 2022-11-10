use std::collections::HashMap;
use std::marker::PhantomData;

use bevy::ecs::system::SystemParam;
use bevy::log::warn;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::assets::loading::meta::MergingAsset;
use crate::game::movement::direction::Direction1D;

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct KeysConfig {
    map: HashMap<InputAction, Vec<InputBinding>>,
}

impl MergingAsset for KeysConfig {
    fn merge(&self, accumulator: Option<KeysConfig>) -> KeysConfig {
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
            debug!("Loading default key configs: {:?}", self.map);
            self.clone()
        }
    }
}

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
}

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

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
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

/// Because it is tagged with [`SystemParam`], this struct can serve as a system parameter.
/// Use it when you want to find out if a key binding is used.
#[derive(SystemParam)]
pub struct InputHandler<'w, 's> {
    config: Res<'w, KeysConfig>,
    keys: Res<'w, Input<KeyCode>>,
    mouse_buttons: Res<'w, Input<MouseButton>>,

    #[system_param(ignore)]
    marker: PhantomData<&'s usize>,
}

impl InputHandler<'_, '_> {
    pub fn is_active(&self, binding: &InputAction) -> bool {
        if let Some(vec) = self.config.map.get(binding) {
            vec.iter().any(|input| match input {
                // Keyboard:
                InputBinding::Key(key, SignalState::Pressed) => self.keys.pressed(*key),
                InputBinding::Key(key, SignalState::JustPressed) => self.keys.just_pressed(*key),
                InputBinding::Key(key, SignalState::JustReleased) => self.keys.just_released(*key),
                InputBinding::KeyModified(modifiers, key, SignalState::Pressed) => {
                    self.modified(modifiers) && self.keys.pressed(*key)
                }
                InputBinding::KeyModified(modifiers, key, SignalState::JustPressed) => {
                    self.modified(modifiers) && self.keys.just_pressed(*key)
                }
                InputBinding::KeyModified(modifiers, key, SignalState::JustReleased) => {
                    self.modified(modifiers) && self.keys.just_released(*key)
                }
                // Mouse:
                InputBinding::Mouse(btn, SignalState::Pressed) => self.mouse_buttons.pressed(*btn),
                InputBinding::Mouse(btn, SignalState::JustPressed) => {
                    self.mouse_buttons.just_pressed(*btn)
                }
                InputBinding::Mouse(btn, SignalState::JustReleased) => {
                    self.mouse_buttons.just_released(*btn)
                }
                InputBinding::MouseModified(modifiers, btn, SignalState::Pressed) => {
                    self.modified(modifiers) && self.mouse_buttons.pressed(*btn)
                }
                InputBinding::MouseModified(modifiers, btn, SignalState::JustPressed) => {
                    self.modified(modifiers) && self.mouse_buttons.just_pressed(*btn)
                }
                InputBinding::MouseModified(modifiers, btn, SignalState::JustReleased) => {
                    self.modified(modifiers) && self.mouse_buttons.just_released(*btn)
                }
            })
        } else {
            // To leave a input action unbound without this warning triggering;
            // add the action to the key bindings config file but leave the vector of InputBindings empty.
            warn!("Key-binding lookup failed: {:?} wasn't bound.", binding);
            false
        }
    }

    fn modified(&self, modifiers: &KeyModifiers) -> bool {
        modifiers.iter().all(|key| self.keys.pressed(*key))
    }

    pub fn direction(&self, negative: &InputAction, positive: &InputAction) -> Direction1D {
        Direction1D::from_input(self.is_active(negative), self.is_active(positive))
    }
}
