use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowMode};

/// Handle some general behaviour related to the window that should be executed in any State.
pub fn handle_window(
    mut keys: ResMut<Input<KeyCode>>,
    mut query: Query<&mut Window, With<PrimaryWindow>>,
) {
    let Ok(mut primary) = query.get_single_mut() else {
        return;
    };
    // Toggle fullscreen:
    if keys.clear_just_pressed(KeyCode::F11) {
        primary.mode = if primary.mode != WindowMode::Windowed {
            WindowMode::Windowed
        } else {
            WindowMode::BorderlessFullscreen
        };
    }
}
