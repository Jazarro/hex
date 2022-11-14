#[cfg(feature = "debugwindow")]
pub use self::debug_window::DebugWindowPlugin;
pub use self::window_event_handler::handle_window;

#[cfg(feature = "debugwindow")]
mod debug_window;
mod window_event_handler;
