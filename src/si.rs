use crate::unit::Unit;

use core::hash::Hash;

#[repr(transparent)]
#[derive(Debug, Default, Clone, Hash)]
pub struct Si<V = f64, const U: Unit = { Unit::default() }>(pub(crate) V);

impl<V: Copy> Copy for Si<V> {}

impl<V> Si<V> {
    #[must_use]
    pub const fn new<const U: Unit>(value: V) -> Si<V, U> {
        Si::<V, U>(value)
    }
}

impl<V, const U: Unit> Si<V, U> {
    pub fn extract(self) -> V {
        self.0
    }
}

impl<V, const U: Unit> Si<V, U>
where
    V: Copy,
{
    // Allow reference for copy values
    pub fn val(&self) -> V {
        self.0
    }
}

impl<V> From<V> for Si<V> {
    fn from(value: V) -> Self {
        Si::<V>(value)
    }
}
