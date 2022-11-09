use std::collections::HashMap;
use std::marker::PhantomData;

use bevy::ecs::system::SystemParam;
use bevy::log::warn;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::assets::loading::meta::MergingAsset;

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct KeysConfig {
    map: HashMap<KeyBinding, Vec<InputMethod>>,
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
pub enum KeyBinding {
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
pub enum InputMethod {
    Pressed(KeyModifiers, InputButton),
    JustPressed(KeyModifiers, InputButton),
    JustReleased(KeyModifiers, InputButton),
    MouseWheelUp,
    MouseWheelDown,
}
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
#[serde(deny_unknown_fields)]
pub enum InputButton {
    Key(KeyCode),
    Mouse(MouseButton),
}
/// The key should be modified by all of these modifiers.
/// Valid modifiers are: LCtrl, LShift, LAlt, RCtrl, RShift, RAlt, Tab.
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
    pub fn is_active(&self, binding: &KeyBinding) -> bool {
        if let Some(vec) = self.config.map.get(binding) {
            vec.iter().any(|input| match input {
                InputMethod::Pressed(modifiers, InputButton::Key(key)) => {
                    self.modified(modifiers) && self.keys.pressed(*key)
                }
                InputMethod::JustPressed(modifiers, InputButton::Key(key)) => {
                    self.modified(modifiers) && self.keys.just_pressed(*key)
                }
                InputMethod::JustReleased(modifiers, InputButton::Key(key)) => {
                    self.modified(modifiers) && self.keys.just_released(*key)
                }
                InputMethod::Pressed(modifiers, InputButton::Mouse(btn)) => {
                    self.modified(modifiers) && self.mouse_buttons.pressed(*btn)
                }
                InputMethod::JustPressed(modifiers, InputButton::Mouse(btn)) => {
                    self.modified(modifiers) && self.mouse_buttons.just_pressed(*btn)
                }
                InputMethod::JustReleased(modifiers, InputButton::Mouse(btn)) => {
                    self.modified(modifiers) && self.mouse_buttons.just_released(*btn)
                }
                _ => panic!("Not implemented yet."),
            })
        } else {
            warn!("Key-binding lookup failed: {:?} wasn't bound.", binding);
            false
        }
    }

    fn modified(&self, modifiers: &KeyModifiers) -> bool {
        modifiers.iter().all(|key| self.keys.pressed(*key))
    }
}
