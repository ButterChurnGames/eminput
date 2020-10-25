/// Module implementing state for 2D movement
use bevy::prelude::*;

#[cfg(feature = "InputSerialization")]
use serde::Serialize;

/// Common imports module
pub mod prelude {
    pub use super::InputAxisState2D;
}

/// State for 2D input
/// Intended for platformers and similar 2d games
/// Also works for fps/tps input
#[cfg(not(feature = "InputSerialization"))]
#[derive(Debug, Default, PartialEq)]
pub struct InputAxisState2D {
    /// Direction the player is inputting
    dir: Vec2,
}

/// State for 2D input
/// Intended for platformers and similar 2d games
/// Also works for fps/tps input
#[cfg(feature = "InputSerialization")]
#[derive(Debug, Default, PartialEq, Serialize)]
pub struct InputAxisState2D {
    /// Direction the player is inputting
    dir: Vec2,
}

impl InputAxisState2D {
    /// Return instance of Self from given Vec2
    pub fn new(dir: Vec2) -> Self {
        Self { dir }
    }

    /// Returns the direction of the input axis
    pub fn get_dir(&self) -> Vec2 {
        self.dir
    }

    /// Returns the direction of the input axis as mutable
    pub fn get_dir_mut(&mut self) -> &mut Vec2 {
        &mut self.dir
    }

    /// Set value of dir
    pub fn set_dir(&mut self, val: Vec2) {
        self.dir = val;
    }

    /// Returns the horizontal component of the input axis
    pub fn x(&self) -> f32 {
        self.dir.x()
    }

    /// Returns the vertical component of the input axis
    pub fn y(&self) -> f32 {
        self.dir.y()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    mod new_tests {
        use super::*;

        #[test]
        fn new_passes() {
            let _axis = InputAxisState2D::new((0.0, 0.0).into());
        }

        #[test]
        fn new_return_eqto_manual_construction() {
            let dir = Vec2::new(0.0, 0.0);
            assert_eq!(InputAxisState2D::new(dir), InputAxisState2D { dir });
        }
    }

    /// InputAxis2D::new() benchmarking module
    mod new_benches {
        use super::*;

        /// Global iteration count that won't leak
        fn get_bench_iter_count() -> u32 {
            10
        }

        #[bench]
        /// Benchmark construction using existing Vec2
        fn new_vec2_cached(b: &mut Bencher) {
            let dir: Vec2 = (0.0, 0.0).into();
            b.iter(|| {
                for _i in 1..get_bench_iter_count() {
                    black_box(InputAxisState2D::new(dir));
                }
            });
        }

        #[bench]
        /// Benchmark construction by calling Vec2's new function
        fn new_vec2new(b: &mut Bencher) {
            b.iter(|| {
                for _i in 1..get_bench_iter_count() {
                    black_box(InputAxisState2D::new(Vec2::new(0.0, 0.0)));
                }
            });
        }

        #[bench]
        /// Benchmark construction by calling .into on a tuple
        fn new_into(b: &mut Bencher) {
            b.iter(|| {
                for _i in 1..get_bench_iter_count() {
                    black_box(InputAxisState2D::new((0.0, 0.0).into()));
                }
            });
        }
    }
}
