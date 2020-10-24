use bevy::prelude::*;

/// Public exports module
pub mod prelude {
    pub use super::*;
}

/// Keybindings for 1D input
pub struct MvBind1D {
    /// KeyCode representing positive on the axis
    pos: KeyCode,
    /// KeyCode representing negative on the axis
    neg: KeyCode,
}

impl MvBind1D {
    /// Returns an instance of Self using given parameters
    pub fn new(pos: KeyCode, neg: KeyCode) -> Self {
        Self { pos, neg }
    }

    /// Returns the current keycode for positive on the axis
    pub fn get_pos(&self) -> KeyCode {
        self.pos
    }
    /// Returns the current keycode for negative on the axis
    pub fn get_neg(&self) -> KeyCode {
        self.neg
    }
}

impl Default for MvBind1D {
    fn default() -> Self {
        Self::new(KeyCode::D, KeyCode::A)
    }
}
