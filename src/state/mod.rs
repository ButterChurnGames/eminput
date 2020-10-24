//! Input state module

#[cfg(features = "1DInput")]
pub mod state1d;
#[cfg(features = "2DInput")]
pub mod state2d;

use bevy::prelude::*;

/// Common public exports module
pub mod prelude {
    #[cfg(features = "1DInput")]
    pub use super::state1d::prelude::*;
    #[cfg(features = "2DInput")]
    pub use super::state2d::prelude::*;
}
