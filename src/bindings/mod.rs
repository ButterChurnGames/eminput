//! Keybindings module

#[cfg(feature = "1DInput")]
pub mod bindings1d;
#[cfg(feature = "2DInput")]
pub mod bindings2d;

/// Public exports module
pub mod prelude {
    #[cfg(feature = "1DInput")]
    pub use super::bindings1d::prelude::*;

    #[cfg(feature = "2DInput")]
    pub use super::bindings2d::prelude::*;
}
