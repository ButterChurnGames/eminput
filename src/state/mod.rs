//! Input state module

// Setup submodules
#[cfg(feature = "1DInput")]
pub mod state1d;
#[cfg(feature = "2DInput")]
pub mod state2d;

/// Common public exports module
pub mod prelude {
    #[cfg(feature = "1DInput")]
    pub use super::state1d::prelude::*;
    #[cfg(feature = "2DInput")]
    pub use super::state2d::prelude::*;
}
