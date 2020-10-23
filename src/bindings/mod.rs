//! Keybindings module

#[cfg(not(features = "No1DInput"))]
pub mod bindings1d;
#[cfg(not(features = "No2DInput"))]
pub mod bindings2d;

/// Public exports module
pub mod prelude {
    // TODO support features here
    pub use super::{bindings1d::prelude::*, bindings2d::prelude::*};
}
