//! Input systems module

#[cfg(feature = "1DInput")]
pub mod update1d;
#[cfg(feature = "2DInput")]
pub mod update2d;

use bevy::prelude::*;

pub mod prelude {
    #[cfg(feature = "1DInput")]
    pub use super::update1d::prelude::*;

    #[cfg(feature = "2DInput")]
    pub use super::update2d::prelude::*;
}
