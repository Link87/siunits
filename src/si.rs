use std::ops::{Add, Mul};

use crate::unit::Unit;

#[derive(Debug)]
pub struct Si<V, const U: Unit>(pub V);

impl<V, const U: Unit> Si<V, U> {
    pub fn num(self) -> V {
        self.0
    }
}

impl<V: Add<Output = V>, const U: Unit> Add for Si<V, U> {
    type Output = Si<V, U>;

    fn add(self, other: Self) -> Self::Output {
        Si::<V, U>(self.0 + other.0)
    }
}

impl<V: Mul<Output = V>, const UL: Unit, const UR: Unit> Mul<Si<V, UR>> for Si<V, UL>
where
    Si<V, { UL + UR }>: Sized,
{
    type Output = Si<V, { UL + UR }>;

    fn mul(self, other: Si<V, UR>) -> Self::Output {
        Si::<V, { UL + UR }>(self.0 * other.0)
    }
}
