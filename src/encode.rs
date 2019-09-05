//! Types and traits which lift `u32` type to FHE compatible
//! `Encoded` struct
use super::algebra::{Mod231, Q231};
use nalgebra::Matrix3;
use num_traits::{One, Zero};
use std::ops::{Add, AddAssign, Mul, MulAssign};

/// Helper trait for encoding `T` into some value `Self`
pub trait Encode<T> {
    /// Encodes `T` into `Self`
    fn encode(value: T) -> Self;
}

/// Helper trait for decoding `Self` into `T`
pub trait Decode<T> {
    /// Decodes `Self` into `T`
    fn decode(&self) -> T;
}

/// Wrapper type for lifting `u32` type to FHE compatible
/// form
///
/// All FHE operations (currently, addition and multiplication)
/// are defined in terms of this type.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Encoded {
    pub(crate) inner: Matrix3<Q231>,
}

impl Encoded {
    #[inline]
    fn enc(value: u32) -> Self {
        Self {
            inner: Q231::from(Mod231::from(value)).into(),
        }
    }

    #[inline]
    fn dec(&self) -> u32 {
        self.inner[0].w.0
    }
}

impl From<Matrix3<Q231>> for Encoded {
    fn from(inner: Matrix3<Q231>) -> Self {
        Self { inner }
    }
}

impl<'a> From<&'a Matrix3<Q231>> for Encoded {
    fn from(inner: &'a Matrix3<Q231>) -> Self {
        Self { inner: *inner }
    }
}

impl Encode<u32> for Encoded {
    #[inline]
    fn encode(value: u32) -> Self {
        Self::enc(value)
    }
}

impl<'a> Encode<&'a u32> for Encoded {
    #[inline]
    fn encode(value: &'a u32) -> Self {
        Self::enc(*value)
    }
}

impl Decode<u32> for Encoded {
    #[inline]
    fn decode(&self) -> u32 {
        self.dec()
    }
}

impl<'a> Decode<u32> for &'a Encoded {
    #[inline]
    fn decode(&self) -> u32 {
        self.dec()
    }
}

impl Zero for Encoded {
    #[inline]
    fn zero() -> Self {
        Self::enc(0)
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.inner[0].is_zero()
    }
}

impl One for Encoded {
    #[inline]
    fn one() -> Self {
        Self::enc(1)
    }

    #[inline]
    fn is_one(&self) -> bool {
        self.inner[0].is_one()
    }
}

impl Add for Encoded {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            inner: self.inner + rhs.inner,
        }
    }
}

impl AddAssign for Encoded {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Mul for Encoded {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            inner: self.inner * rhs.inner,
        }
    }
}

impl MulAssign for Encoded {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}
