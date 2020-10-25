//! Bevy input library

#![feature(test)]
extern crate test;

pub mod bindings;
pub mod state;
pub mod systems;

use {self::prelude::*, bevy::prelude::*};

/// Public exports module
pub mod prelude {
    pub use super::{bindings::prelude::*, state::prelude::*, systems::prelude::*};
}

/// Bevy plugin for easily adding eminput to a bevy app
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        #[cfg(feature = "1DInput")]
            app.add_system_to_stage(
            stage::PRE_UPDATE,
            InputUpdateSystem1D::update_input_1d.system(),
        );
        #[cfg(feature = "2DInput")]
            app.add_system_to_stage(
            stage::PRE_UPDATE,
            InputUpdateSystem2D::update_input_2d.system(),
        );
    }
}

// TODO Unit tests
// TODO benches
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
