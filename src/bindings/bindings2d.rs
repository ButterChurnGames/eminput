use bevy::prelude::*;

/// Public exports module
pub mod prelude {
    pub use super::*;
}

/// Keybindings for 2D input
pub struct MvBind2D {
    /// Keycode representing positive on the X axis
    xpos: KeyCode,
    /// Key representing negative on the X axis
    xneg: KeyCode,
    /// Keycode representing positive on the Y axis
    ypos: KeyCode,
    /// Keycode representing negative on the Y axis
    yneg: KeyCode,
}

impl MvBind2D {
    /// Returns an instance of Self from given parameters
    pub fn new(xpos: KeyCode, xneg: KeyCode, ypos: KeyCode, yneg: KeyCode) -> Self {
        Self {
            xpos,
            xneg,
            ypos,
            yneg,
        }
    }

    /// Returns the current Xaxis positive keycode
    pub fn get_x_pos(&self) -> KeyCode {
        self.xpos
    }
    /// Returns the current Xaxis negative keycode
    pub fn get_x_neg(&self) -> KeyCode {
        self.xneg
    }
    /// Returns the current Yaxis positive keycode
    pub fn get_y_pos(&self) -> KeyCode {
        self.ypos
    }
    /// Returns the current Yaxis negative keycode
    pub fn get_y_neg(&self) -> KeyCode {
        self.yneg
    }
}

impl Default for MvBind2D {
    fn default() -> Self {
        Self::new(KeyCode::D, KeyCode::A, KeyCode::W, KeyCode::S)
    }
}
