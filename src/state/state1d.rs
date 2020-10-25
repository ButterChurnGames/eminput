/// Common imports module
pub mod prelude {
    pub use super::InputAxisState1D;
}

/// State for 1D input
/// Games like breakout
#[derive(Default)]
pub struct InputAxisState1D {
    /// Value of the input axis
    n: f32,
}

impl InputAxisState1D {
    /// Returns new instance of Self using given float
    /// * 'n' - Initialization value of n
    pub fn new(n: f32) -> Self {
        Self { n }
    }

    /// Returns value of n
    pub fn get_n(&self) -> f32 {
        self.n
    }

    /// Returns mutable value of n
    pub fn get_n_mut(&mut self) -> f32 {
        self.n
    }

    /// Set n to input value
    /// * 'val' - input value
    pub fn set_n(&mut self, val: f32) {
        self.n = val;
    }
}
