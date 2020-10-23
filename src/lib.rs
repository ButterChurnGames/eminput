use crate::systems::prelude::InputUpdateSystem2D;
use bevy::prelude::*;

pub mod bindings;
pub mod state;
pub mod systems;

pub mod prelude {
    pub use super::{bindings::prelude::*, state::prelude::*, systems::prelude::*};
}

/// Bevy plugin for easily adding eminput-bevy to a bevy app
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        #[cfg(not(features = "No2DInput"))]
        app.add_system_to_stage(stage::PRE_UPDATE, InputUpdateSystem1D.system());
        #[cfg(not(features = "No2DInput"))]
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
