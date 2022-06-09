use core::cmp::Ordering;
use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Neg, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

use crate::si::Si;
use crate::unit::Unit;

impl<VL: Add<VR, Output = O>, VR, O, const U: Unit> Add<Si<VR, U>> for Si<VL, U> {
    type Output = Si<O, U>;

    fn add(self, other: Si<VR, U>) -> Self::Output {
        Si::<O, U>(self.0 + other.0)
    }
}

impl<V: AddAssign<VR>, VR, const U: Unit> AddAssign<Si<VR, U>> for Si<V, U> {
    fn add_assign(&mut self, other: Si<VR, U>) {
        (*self).0 += other.0;
    }
}

impl<VL: Sub<VR, Output = O>, VR, O, const U: Unit> Sub<Si<VR, U>> for Si<VL, U> {
    type Output = Si<O, U>;

    fn sub(self, other: Si<VR, U>) -> Self::Output {
        Si::<O, U>(self.0 - other.0)
    }
}

impl<V: SubAssign<VR>, VR, const U: Unit> SubAssign<Si<VR, U>> for Si<V, U> {
    fn sub_assign(&mut self, other: Si<VR, U>) {
        (*self).0 -= other.0;
    }
}

impl<V: Neg<Output = O>, O, const U: Unit> Neg for Si<V, U> {
    type Output = Si<O, U>;

    fn neg(self) -> Self::Output {
        Si::<O, U>(-self.0)
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl<VL: Mul<VR, Output = O>, VR, O, const UL: Unit, const UR: Unit> Mul<Si<VR, UR>> for Si<VL, UL>
where
    Si<O, { UL + UR }>: Sized,
{
    type Output = Si<O, { UL + UR }>;

    fn mul(self, other: Si<VR, UR>) -> Self::Output {
        Si::<O, { UL + UR }>(self.0 * other.0)
    }
}

// Only allow *= with a rhs that has no dimension.
impl<V: MulAssign<VR>, VR, const UL: Unit> MulAssign<Si<VR>> for Si<V, UL> {
    fn mul_assign(&mut self, other: Si<VR>) {
        (*self).0 *= other.0;
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl<VL: Div<VR, Output = O>, VR, O, const UL: Unit, const UR: Unit> Div<Si<VR, UR>> for Si<VL, UL>
where
    Si<O, { UL - UR }>: Sized,
{
    type Output = Si<O, { UL - UR }>;

    fn div(self, other: Si<VR, UR>) -> Self::Output {
        Si::<O, { UL - UR }>(self.0 / other.0)
    }
}

// Only allow /= with a rhs that has no dimension.
impl<V: DivAssign<VR>, VR, const UL: Unit> DivAssign<Si<VR>> for Si<V, UL> {
    fn div_assign(&mut self, other: Si<VR>) {
        (*self).0 /= other.0;
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl<VL: Rem<VR, Output = O>, VR, O, const UL: Unit, const UR: Unit> Rem<Si<VR, UR>> for Si<VL, UL>
where
    Si<O, { UL - UR }>: Sized,
{
    type Output = Si<O, { UL - UR }>;

    fn rem(self, other: Si<VR, UR>) -> Self::Output {
        Si::<O, { UL - UR }>(self.0 % other.0)
    }
}

// Only allow %= with a rhs that has no dimension.
impl<V: RemAssign<VR>, VR, const UL: Unit> RemAssign<Si<VR>> for Si<V, UL> {
    fn rem_assign(&mut self, other: Si<VR>) {
        (*self).0 %= other.0;
    }
}

// Don't use Si value as rhs for shifting.
impl<VL: Shl<VR, Output = O>, VR, O, const U: Unit> Shl<VR> for Si<VL, U> {
    type Output = Si<O, U>;

    fn shl(self, other: VR) -> Self::Output {
        Si::<O, U>(self.0 << other)
    }
}

// Don't use Si value as rhs for shifting.
impl<V: ShlAssign<VR>, VR, const U: Unit> ShlAssign<VR> for Si<V, U> {
    fn shl_assign(&mut self, other: VR) {
        (*self).0 <<= other;
    }
}

// Don't use Si value as rhs for shifting.
impl<VL: Shr<VR, Output = O>, VR, O, const U: Unit> Shr<VR> for Si<VL, U> {
    type Output = Si<O, U>;

    fn shr(self, other: VR) -> Self::Output {
        Si::<O, U>(self.0 >> other)
    }
}

// Don't use Si value as rhs for shifting.
impl<V: ShrAssign<VR>, VR, const U: Unit> ShrAssign<VR> for Si<V, U> {
    fn shr_assign(&mut self, other: VR) {
        (*self).0 >>= other;
    }
}

impl<VL: BitAnd<VR, Output = O>, VR, O, const U: Unit> BitAnd<Si<VR, U>> for Si<VL, U> {
    type Output = Si<O, U>;

    fn bitand(self, other: Si<VR, U>) -> Self::Output {
        Si::<O, U>(self.0 & other.0)
    }
}

impl<V: BitAndAssign<VR>, VR, const U: Unit> BitAndAssign<Si<VR, U>> for Si<V, U> {
    fn bitand_assign(&mut self, other: Si<VR, U>) {
        (*self).0 &= other.0;
    }
}

impl<VL: BitOr<VR, Output = O>, VR, O, const U: Unit> BitOr<Si<VR, U>> for Si<VL, U> {
    type Output = Si<O, U>;

    fn bitor(self, other: Si<VR, U>) -> Self::Output {
        Si::<O, U>(self.0 | other.0)
    }
}

impl<V: BitOrAssign<VR>, VR, const U: Unit> BitOrAssign<Si<VR, U>> for Si<V, U> {
    fn bitor_assign(&mut self, other: Si<VR, U>) {
        (*self).0 |= other.0;
    }
}

impl<VL: BitXor<VR, Output = O>, VR, O, const U: Unit> BitXor<Si<VR, U>> for Si<VL, U> {
    type Output = Si<O, U>;

    fn bitxor(self, other: Si<VR, U>) -> Self::Output {
        Si::<O, U>(self.0 ^ other.0)
    }
}

impl<V: BitXorAssign<VR>, VR, const U: Unit> BitXorAssign<Si<VR, U>> for Si<V, U> {
    fn bitxor_assign(&mut self, other: Si<VR, U>) {
        (*self).0 ^= other.0;
    }
}

impl<VL: PartialEq<VR>, VR, const U: Unit> PartialEq<Si<VR, U>> for Si<VL, U> {
    #[must_use]
    fn eq(&self, other: &Si<VR, U>) -> bool {
        self.0 == other.0
    }
}

impl<V: Eq, const U: Unit> Eq for Si<V, U> {}

impl<VL: PartialOrd<VR>, VR, const U: Unit> PartialOrd<Si<VR, U>> for Si<VL, U> {
    fn partial_cmp(&self, other: &Si<VR, U>) -> Option<Ordering> {
        VL::partial_cmp(&self.0, &other.0)
    }

    fn lt(&self, other: &Si<VR, U>) -> bool {
        VL::lt(&self.0, &other.0)
    }

    fn le(&self, other: &Si<VR, U>) -> bool {
        VL::le(&self.0, &other.0)
    }

    fn gt(&self, other: &Si<VR, U>) -> bool {
        VL::gt(&self.0, &other.0)
    }

    fn ge(&self, other: &Si<VR, U>) -> bool {
        VL::ge(&self.0, &other.0)
    }
}

impl<V: Ord, const U: Unit> Ord for Si<V, U> {
    fn cmp(&self, other: &Self) -> Ordering {
        V::cmp(&self.0, &other.0)
    }

    fn max(self, other: Self) -> Self {
        Self(V::max(self.0, other.0))
    }

    fn min(self, other: Self) -> Self {
        Self(V::min(self.0, other.0))
    }

    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
    {
        Self(V::clamp(self.0, min.0, max.0))
    }
}
