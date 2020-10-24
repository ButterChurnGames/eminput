//! Input systems module

#[cfg(features = "1DInput")]
pub mod update1d;
#[cfg(features = "2DInput")]
pub mod update2d;

use bevy::prelude::*;

pub mod prelude {
    #[cfg(features = "1DInput")]
    pub use super::update1d::prelude::*;

    #[cfg(features = "2DInput")]
    pub use super::update2d::prelude::*;
}
