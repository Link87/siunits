use crate::unit::Unit;

use core::hash::Hash;

#[derive(Debug, Default, Hash)]
pub struct Si<V = f64, const U: Unit = { Unit::default() }>(pub V);

impl<V, const U: Unit> Si<V, U> {
    // Allow reference for copy values
    pub fn num(self) -> V {
        self.0
    }
}

impl<V> From<V> for Si<V> {
    fn from(value: V) -> Self {
        Si::<V>(value)
    }
}
