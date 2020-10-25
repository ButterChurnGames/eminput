use crate::prelude::*;
use bevy::prelude::*;

pub mod prelude {
    pub use super::InputUpdateSystem2D;
}

pub struct InputUpdateSystem2D;

impl InputUpdateSystem2D {
    /// Bevy system for updating 2d input axis
    pub fn update_input_2d(
        input: Res<Input<KeyCode>>,
        mut query: Query<(&MvBind2D, &mut InputAxisState2D)>,
    ) {
        // Iterate through entities with:
        //  * 'bindings' - Movement binding 2d component
        //  * 'state' - Movement state 2d component
        // TODO consider implementing different SOCD strategies
        for (bindings, mut state) in &mut query.iter() {
            let mut dirx = 0.0;
            let mut diry = 0.0;

            // Get x direction
            if input.pressed(bindings.get_x_pos()) {
                dirx += 1.0;
            }
            if input.pressed(bindings.get_x_neg()) {
                dirx -= 1.0;
            }
            // Get y direction
            if input.pressed(bindings.get_y_pos()) {
                diry += 1.0;
            }
            if input.pressed(bindings.get_y_neg()) {
                diry -= 1.0;
            }

            // TODO look into faster implementation than into
            state.set_dir((dirx, diry).into());
        }
    }
}
