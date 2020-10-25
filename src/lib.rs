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
pub struct EMInputPlugin;

impl Plugin for EMInputPlugin {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[ignore]
    /// Tests a bevy app by adding input plugin and running it once
    /// NOTE requires running in single threaded test mode
    /// ie: cargo test -- --test-threads 1 --ignored
    /// This requirement is why it is ignored by default
    // TODO Setup integration tests to run this as a seperate
    // invocation of cargo test
    fn test_bevy_eminput_plugin() {
        App::build()
            .add_plugin(bevy::input::InputPlugin)
            .add_plugin(bevy::app::ScheduleRunnerPlugin::run_once())
            .add_plugin(EMInputPlugin)
            .run();
    }

    // TODO add integration tests for input bindings etc
}
