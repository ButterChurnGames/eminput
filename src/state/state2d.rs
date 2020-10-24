/// Module implementing state for 2D movement
use bevy::prelude::*;

/// Common imports module
pub mod prelude {
    pub use super::InputAxisState2D;
}

/// State for 2D input
/// Intended for platformers and similar 2d games
/// Also works for fps/tps input
#[derive(Default)]
pub struct InputAxisState2D {
    /// Direction the player is inputting
    dir: Vec2,
}

impl InputAxisState2D {
    /// Return instance of Self from given Vec2
    pub fn new(dir: Vec2) -> Self {
        Self { dir }
    }

    /// Returns the direction of the input axis
    pub fn get_dir(&self) -> Vec2 {
        self.dir
    }

    /// Returns the direction of the input axis as mutable
    pub fn get_dir_mut(&mut self) -> &mut Vec2 {
        &mut self.dir
    }

    /// Set value of dir
    pub fn set_dir(&mut self, val: Vec2) {
        self.dir = val;
    }

    /// Returns the horizontal component of the input axis
    pub fn x(&self) -> f32 {
        return self.dir.x();
    }

    /// Returns the vertical component of the input axis
    pub fn y(&self) -> f32 {
        return self.dir.y();
    }
}
