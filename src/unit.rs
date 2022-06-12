use crate::dim::Dim;

use core::ops::{Add, Sub};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Unit {
    // TODO scale: f64,
    dim: Dim,
}

impl Unit {}

// Clippy does not recognise const impls.
impl const Default for Unit {
    fn default() -> Self {
        Unit {
            dim: Dim::default(),
        }
    }
}

impl const Add for Unit {
    type Output = Unit;
    fn add(self, rhs: Self) -> Self::Output {
        Unit {
            dim: self.dim + rhs.dim,
        }
    }
}

impl const Sub for Unit {
    type Output = Unit;
    fn sub(self, rhs: Self) -> Self::Output {
        Unit {
            dim: self.dim - rhs.dim,
        }
    }
}

pub const METER: Unit = Unit {
    dim: Dim::new(0, 1, 0, 0, 0, 0, 0),
};
pub const NEWTON: Unit = Unit {
    dim: Dim::new(-2, 1, 1, 0, 0, 0, 0),
};
pub const JOULE: Unit = Unit {
    dim: Dim::new(-2, 2, 1, 0, 0, 0, 0),
};
