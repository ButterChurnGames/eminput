use crate::prelude::*;
use bevy::prelude::*;

pub mod prelude {
    pub use super::InputUpdateSystem1D;
}

pub struct InputUpdateSystem1D;

impl InputUpdateSystem1D {
    /// Bevy system for updating 1D input axis
    pub fn update_input_1d(
        input: Res<Input<KeyCode>>,
        mut query: Query<(&MvBind1D, &mut InputAxisState1D)>,
    ) {
        // Iterate through entities with:
        //  * 'bindings' - Movement binding 1d component
        //  * 'state' - Movement state 1d component
        // TODO consider implementing different SOCD strategies
        for (bindings, mut state) in &mut query.iter() {
            let mut dir = 0.0;

            // Get x direction
            if input.pressed(bindings.get_pos()) {
                dir += 1.0;
            }
            if input.pressed(bindings.get_neg()) {
                dir -= 1.0;
            }
            state.set_n(dir);
        }
    }
}
