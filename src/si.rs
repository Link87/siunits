use std::ops::{Add, Mul};

use crate::dim::Dim;

#[derive(Debug)]
pub struct Si<V, const D: Dim>(pub V);

impl<V: Add<Output = V>, const D: Dim> Add for Si<V, D> {
    type Output = Si<V, D>;

    fn add(self, other: Self) -> Self::Output {
        Si::<V, D>(self.0 + other.0)
    }
}

impl<V: Mul<Output = V>, const DL: Dim, const DR: Dim> Mul<Si<V, DR>> for Si<V, DL>
where
    Si<V, { DL + DR }>: Sized,
{
    type Output = Si<V, { DL + DR }>;

    fn mul(self, other: Si<V, DR>) -> Self::Output {
        Si::<V, { DL + DR }>(self.0 * other.0)
    }
}
