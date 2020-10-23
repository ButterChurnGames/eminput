//! Input systems module

#[cfg(not(features = "No1DInput"))]
pub mod update1d;
#[cfg(not(features = "No2DInput"))]
pub mod update2d;

use bevy::prelude::*;

pub mod prelude {
    #[cfg(not(features = "No2DInput"))]
    pub use super::update2d::prelude::*;
}
