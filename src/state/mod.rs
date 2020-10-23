//! Input state module


#[cfg(not(features = "No1DInput"))]
pub mod state1d;
#[cfg(not(features = "No2DInput"))]
pub mod state2d;

use bevy::prelude::*;

/// Common public exports module
pub mod prelude {
    #[cfg(not(features = "No1DInput"))]
    pub use super::state1d::prelude::*;
    #[cfg(not(features = "No2DInput"))]
    pub use super::state2d::prelude::*;
}
