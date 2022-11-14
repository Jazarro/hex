use crate::game::movement::direction::Direction1D;
use crate::io::config::InputConfig;
use crate::io::input::binding::{InputBinding, KeyModifiers, SignalState};
use crate::io::input::InputAction;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use std::marker::PhantomData;

/// Because it is tagged with [`SystemParam`], this struct can serve as a system parameter.
/// Use it when you want to find out if a key binding is used.
#[derive(SystemParam)]
pub struct InputHandler<'w, 's> {
    config: Res<'w, InputConfig>,
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
