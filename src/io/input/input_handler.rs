use bevy::ecs::system::Resource;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy::utils::HashMap;
use std::marker::PhantomData;

use crate::game::movement::direction::Direction1D;
use crate::io::config::InputConfig;
use crate::io::input::binding::{InputBinding, KeyModifiers, SignalState};
use crate::io::input::InputAction;

/// Because it is tagged with [`SystemParam`], this struct can serve as a system parameter.
/// Use it when you want to find out if a key binding is used.
#[derive(SystemParam)]
pub struct InputHandler<'w, 's> {
    bindings: Res<'w, ProcessedBindings>,
    keys: Res<'w, Input<KeyCode>>,
    mouse_buttons: Res<'w, Input<MouseButton>>,

    #[system_param(ignore)]
    marker: PhantomData<&'s usize>,
}

impl InputHandler<'_, '_> {
    /// Use this if there's two strictly opposing inputs, such as move backwards/forwards, turn left/right, etc.
    /// It returns a direction, which is:
    ///  - negative if only the first InputAction is active.
    ///  - positive if only the second InputAction is active.
    ///  - neutral if both or neither InputActions are active.
    pub fn direction(&self, negative: impl InputAction, positive: impl InputAction) -> Direction1D {
        Direction1D::from_input(self.is_active(negative), self.is_active(positive))
    }
    /// Check if the given InputAction is active.
    pub fn is_active(&self, action: impl InputAction) -> bool {
        if let Some(binding) = self
            .bindings
            .get_binding(action.group_id(), action.item_id())
        {
            binding.iter().any(|input| match input {
                ProcessedBinding::Key(params, key) => {
                    self.key_active(key, &params.signal_state)
                        && self.modified(&params.modifiers)
                        && self.not_blocked(&params.blockers)
                }
                ProcessedBinding::Mouse(params, btn) => {
                    self.mouse_btn_active(btn, &params.signal_state)
                        && self.modified(&params.modifiers)
                        && self.not_blocked(&params.blockers)
                }
            })
        } else {
            // To leave a input action unbound without this warning spamming the log, add the
            // action to the key bindings config file but leave the vector of InputBindings empty.
            warn!(
                "Key-binding lookup failed: item {:?}::{:?} wasn't bound.",
                action.group_id(),
                action.item_id()
            );
            false
        }
    }

    fn modified(&self, modifiers: &KeyModifiers) -> bool {
        modifiers.iter().all(|key| self.keys.pressed(*key))
    }
    fn not_blocked(&self, blockers: &[KeyModifiers]) -> bool {
        blockers.iter().all(|blocker| !self.modified(blocker))
    }
    fn key_active(&self, key: &KeyCode, signal_state: &SignalState) -> bool {
        match signal_state {
            SignalState::Pressed => self.keys.pressed(*key),
            SignalState::JustPressed => self.keys.just_pressed(*key),
            SignalState::JustReleased => self.keys.just_released(*key),
        }
    }
    fn mouse_btn_active(&self, btn: &MouseButton, signal_state: &SignalState) -> bool {
        match signal_state {
            SignalState::Pressed => self.mouse_buttons.pressed(*btn),
            SignalState::JustPressed => self.mouse_buttons.just_pressed(*btn),
            SignalState::JustReleased => self.mouse_buttons.just_released(*btn),
        }
    }
}

/// We can create ProcessedInput from InputConfig. We perform pre-processing on the input
/// config to auto-generate modifier exclusions (see BindingParameters::blocked). This keeps the
/// config file syntax nice and clean while also making it easy and fast to query the data at runtime.
#[derive(Resource)]
pub struct ProcessedBindings(HashMap<String, ProcessedBindingGroup>);

/// All bindings within a certain group.
type ProcessedBindingGroup = HashMap<String, Vec<ProcessedBinding>>;

enum ProcessedBinding {
    Key(BindingParameters, KeyCode),
    Mouse(BindingParameters, MouseButton),
}

struct BindingParameters {
    /// The signal state required for the base key or button. (Pressed, just pressed, etc)
    signal_state: SignalState,
    /// The modifiers that MUST be active.
    modifiers: KeyModifiers,
    /// A list of modifiers that MUST NOT be active.
    /// If any of these KeyModifiers is active, then this binding is not active.
    ///
    /// This is to prevent a binding from triggering when you intended to trigger a different
    /// binding that uses the same base key. For example; if you hit Ctrl-S, you want the
    /// SAVE-binding to trigger, but you don't want to type the letter s simultaneously.
    ///
    /// These blockers are the main reason we use processed bindings instead of taking them
    /// straight from the InputConfig; because we auto-generate the exclusions based on the other
    /// bindings that exist.
    blockers: Vec<KeyModifiers>,
}

impl BindingParameters {
    fn new(signal_state: SignalState, modifiers: KeyModifiers, blocked: Vec<KeyModifiers>) -> Self {
        Self {
            signal_state,
            modifiers,
            blockers: blocked,
        }
    }
}

impl ProcessedBindings {
    fn process(config: &InputConfig) -> Self {
        let mods = GatheredModifiers::new(config);
        let groups = config
            .bindings
            .iter()
            .map(|(group_id, group_data)| {
                let group: HashMap<String, Vec<ProcessedBinding>> = group_data
                    .iter()
                    .map(|(item_id, item_data)| {
                        let vec: Vec<ProcessedBinding> = item_data
                            .iter()
                            .map(|input| {
                                match input {
                                    // Keyboard:
                                    InputBinding::Key(key, signal_state) => ProcessedBinding::Key(
                                        BindingParameters::new(
                                            *signal_state,
                                            vec![],
                                            mods.blocked_for_key(*key, &vec![]),
                                        ),
                                        *key,
                                    ),
                                    InputBinding::KeyModified(modifiers, key, signal_state) => {
                                        ProcessedBinding::Key(
                                            BindingParameters::new(
                                                *signal_state,
                                                modifiers.clone(),
                                                mods.blocked_for_key(*key, modifiers),
                                            ),
                                            *key,
                                        )
                                    }
                                    // Mouse:
                                    InputBinding::Mouse(btn, signal_state) => {
                                        ProcessedBinding::Mouse(
                                            BindingParameters::new(
                                                *signal_state,
                                                vec![],
                                                mods.blocked_for_mouse_btn(*btn, &vec![]),
                                            ),
                                            *btn,
                                        )
                                    }
                                    InputBinding::MouseModified(modifiers, btn, signal_state) => {
                                        ProcessedBinding::Mouse(
                                            BindingParameters::new(
                                                *signal_state,
                                                modifiers.clone(),
                                                mods.blocked_for_mouse_btn(*btn, modifiers),
                                            ),
                                            *btn,
                                        )
                                    }
                                }
                            })
                            .collect();
                        (item_id.clone(), vec)
                    })
                    .collect();
                (group_id.clone(), group)
            })
            .collect();
        Self(groups)
    }

    /// Returns None if the action was not bound, and Some(empty vec) if the action was explicitly
    /// mentioned in the input config file as being bound to zero bindings.
    fn get_binding(
        &self,
        group_id: &'static str,
        item_id: &'static str,
    ) -> Option<&Vec<ProcessedBinding>> {
        self.0.get(group_id)?.get(item_id)
    }
}

/// Struct that is temporarily used during input processing, for convenience.
struct GatheredModifiers {
    keys: HashMap<KeyCode, Vec<KeyModifiers>>,
    mouse_buttons: HashMap<MouseButton, Vec<KeyModifiers>>,
}

impl GatheredModifiers {
    fn new(config: &InputConfig) -> GatheredModifiers {
        let mut keys = HashMap::default();
        let mut mouse_buttons = HashMap::default();
        config.bindings.iter().for_each(|(group_id, group)| {
            group.iter().for_each(|(item_id, action_bindings)| {
                action_bindings.iter().for_each(|binding| {
                    match binding {
                        // Keyboard:
                        InputBinding::Key(key, signal_state) => {
                            if !keys.contains_key(key) {
                                keys.insert(*key, vec![]);
                            }
                        }
                        InputBinding::KeyModified(modifiers, key, signal_state) => {
                            if !keys.contains_key(key) {
                                keys.insert(*key, vec![]);
                            }
                            keys.get_mut(key)
                                .expect("Cannot fail.")
                                .push(modifiers.clone());
                        }
                        // Mouse:
                        InputBinding::Mouse(btn, signal_state) => {
                            if !mouse_buttons.contains_key(btn) {
                                mouse_buttons.insert(*btn, vec![]);
                            }
                        }
                        InputBinding::MouseModified(modifiers, btn, signal_state) => {
                            if !mouse_buttons.contains_key(btn) {
                                mouse_buttons.insert(*btn, vec![]);
                            }
                            mouse_buttons
                                .get_mut(btn)
                                .expect("Cannot fail.")
                                .push(modifiers.clone());
                        }
                    }
                });
            });
        });
        GatheredModifiers {
            keys,
            mouse_buttons,
        }
    }
    fn blocked_for_key(&self, key: KeyCode, mods: &KeyModifiers) -> Vec<KeyModifiers> {
        self.keys
            .get(&key)
            .expect("Cannot fail.")
            .iter()
            .filter(|&other_mods| other_mods.iter().any(|mod_key| !mods.contains(mod_key)))
            .cloned()
            .collect()
    }
    fn blocked_for_mouse_btn(
        &self,
        mouse_button: MouseButton,
        mods: &KeyModifiers,
    ) -> Vec<KeyModifiers> {
        self.mouse_buttons
            .get(&mouse_button)
            .expect("Cannot fail.")
            .iter()
            .filter(|&other_mods| other_mods.iter().any(|mod_key| !mods.contains(mod_key)))
            .cloned()
            .collect()
    }
}

/// Call this during loading time, to initialise resources needed by the input handler.
pub fn process_input_bindings(mut commands: Commands, config: Res<InputConfig>) {
    commands.insert_resource(ProcessedBindings::process(&config));
}
