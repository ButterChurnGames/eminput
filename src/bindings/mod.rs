//! Keybindings module

#[cfg(features = "1DInput")]
pub mod bindings1d;
#[cfg(features = "2DInput")]
pub mod bindings2d;

/// Public exports module
pub mod prelude {
    #[cfg(features = "1DInput")]
    pub use super::bindings1d::prelude::*;

    #[cfg(features = "2DInput")]
    pub use super::bindings2d::prelude::*;
}
