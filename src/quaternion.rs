use alga::general::Ring;
use nalgebra::Vector4;
use num_traits::{One, Zero};
use std::fmt;
use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

/// Quaternion
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Quaternion<T>
where
    T: Ring + fmt::Debug + Copy + 'static,
{
    inner: Vector4<T>,
}

impl<T> Quaternion<T>
where
    T: Ring + fmt::Debug + Copy,
{
    pub fn from_iter(iter: impl Iterator<Item = T>) -> Self {
        Self::from(Vector4::from_iterator(iter))
    }
}

impl<T> fmt::Display for Quaternion<T>
where
    T: Ring + fmt::Debug + Copy + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}+{}i+{}j+{}k", self[0], self[1], self[2], self[3])
    }
}

impl<T> Index<usize> for Quaternion<T>
where
    T: Ring + fmt::Debug + Copy,
{
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl<T> IndexMut<usize> for Quaternion<T>
where
    T: Ring + fmt::Debug + Copy,
{
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}

impl<T> From<Vector4<T>> for Quaternion<T>
where
    T: Ring + fmt::Debug + Copy + 'static,
{
    fn from(inner: Vector4<T>) -> Self {
        Self { inner }
    }
}

impl<T> From<[T; 4]> for Quaternion<T>
where
    T: Ring + fmt::Debug + Copy,
{
    fn from(arr: [T; 4]) -> Self {
        Self::from(Vector4::from(arr))
    }
}

impl<T> Add for Quaternion<T>
where
    T: Ring + fmt::Debug + Copy,
{
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Quaternion::from(self.inner + other.inner)
    }
}

impl<T> AddAssign for Quaternion<T>
where
    T: Ring + fmt::Debug + Copy,
{
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.inner += other.inner
    }
}

impl<T> Neg for Quaternion<T>
where
    T: Ring + fmt::Debug + Copy,
{
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        self.inner.neg().into()
    }
}

impl<T> Zero for Quaternion<T>
where
    T: Ring + fmt::Debug + Copy,
{
    #[inline]
    fn zero() -> Self {
        Self {
            inner: Vector4::zero(),
        }
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.inner.is_zero()
    }
}

impl<T> Mul for Quaternion<T>
where
    T: Ring + fmt::Debug + Copy,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let e = self[0] * other[0] - self[1] * other[1] - self[2] * other[2] - self[3] * other[3];
        let i = self[0] * other[1] + self[1] * other[0] + self[2] * other[3] - self[3] * other[2];
        let j = self[0] * other[2] + self[2] * other[0] + self[3] * other[1] - self[1] * other[3];
        let k = self[0] * other[3] + self[3] * other[0] + self[1] * other[2] - self[2] * other[1];
        Self::from([e, i, j, k])
    }
}

impl<T> MulAssign for Quaternion<T>
where
    T: Ring + fmt::Debug + Copy,
{
    fn mul_assign(&mut self, other: Self) {
        self[0] = self[0] * other[0] - self[1] * other[1] - self[2] * other[2] - self[3] * other[3];
        self[1] = self[0] * other[1] + self[1] * other[0] + self[2] * other[3] - self[3] * other[2];
        self[2] = self[0] * other[2] + self[2] * other[0] + self[3] * other[1] - self[1] * other[3];
        self[3] = self[0] * other[3] + self[3] * other[0] + self[1] * other[2] - self[2] * other[1];
    }
}

impl<T> One for Quaternion<T>
where
    T: Ring + fmt::Debug + Copy,
{
    #[inline]
    fn one() -> Self {
        Self {
            inner: Vector4::from([T::one(), T::zero(), T::zero(), T::zero()]),
        }
    }

    #[inline]
    fn is_one(&self) -> bool {
        let zero = T::zero();
        let one = T::one();
        &self[0] == &one && &self[1] == &zero && &self[2] == &zero && &self[3] == &zero
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, Arbitrary, Gen};
    use rand::Rng;

    impl Arbitrary for Quaternion<i64> {
        fn arbitrary<G: Gen>(g: &mut G) -> Quaternion<i64> {
            Quaternion::from_iter((0..4).into_iter().map(|_| g.gen()))
        }
    }

    quickcheck! {
        fn quaternion_add(x: Quaternion<i64>) -> bool {
            x + Quaternion::zero() == x
        }
    }

    quickcheck! {
        fn quaternion_mul(x: Quaternion<i64>) -> bool {
            x * Quaternion::one() == x
        }
    }

    quickcheck! {
        fn quaternion_neg(x: Quaternion<i64>) -> bool {
            (x + x.neg()).is_zero()
        }
    }
}
