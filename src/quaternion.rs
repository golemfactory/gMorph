use num_traits::{One, Zero};
use std::fmt;
use std::ops::{Add, AddAssign, Index, Mul, MulAssign, Neg, Sub};

const DIM4: usize = 4;

/// Quaternion
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Quaternion<T>
where
    T: Copy + Clone + fmt::Debug + PartialEq,
{
    data: [T; DIM4],
}

impl<T> From<[T; DIM4]> for Quaternion<T>
where
    T: fmt::Debug + Copy + PartialEq,
{
    fn from(data: [T; DIM4]) -> Self {
        Self { data }
    }
}

impl<T> Quaternion<T>
where
    T: Copy + fmt::Debug + PartialEq + Clone + Zero,
{
    pub fn from_iter(it: impl Iterator<Item = T>) -> Self {
        let mut data = [T::zero(); DIM4];
        for (x, i) in it.zip(0..DIM4) {
            data[i] = x;
        }
        Self { data }
    }
}

impl<T> Add for Quaternion<T>
where
    T: Copy + fmt::Debug + Zero + Add + PartialEq,
{
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        let mut data = [T::zero(); DIM4];
        for i in 0..DIM4 {
            data[i] = self.data[i] + other.data[i];
        }
        Self { data }
    }
}

impl<T> AddAssign for Quaternion<T>
where
    T: Copy + fmt::Debug + Zero + AddAssign + PartialEq,
{
    #[inline]
    fn add_assign(&mut self, other: Self) {
        *self = self.add(other)
    }
}

impl<T> Neg for Quaternion<T>
where
    T: Copy + fmt::Debug + PartialEq + Neg<Output = T>,
{
    type Output = Self;

    #[inline]
    fn neg(mut self) -> Self {
        for i in 0..DIM4 {
            self.data[i] = self.data[i].neg();
        }
        self
    }
}

impl<T> Zero for Quaternion<T>
where
    T: Copy + fmt::Debug + Zero + PartialEq,
{
    #[inline]
    fn zero() -> Self {
        Self {
            data: [T::zero(); DIM4],
        }
    }

    #[inline]
    fn is_zero(&self) -> bool {
        let z = T::zero();
        &self[0] == &z && &self[1] == &z && &self[2] == &z && &self[3] == &z
    }
}

impl<T> Mul for Quaternion<T>
where
    T: Copy + Clone + fmt::Debug + PartialEq + Sub<Output = T> + Mul<Output = T> + Add<Output = T>,
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
    T: Copy + fmt::Debug + PartialEq + Mul<Output = T> + Add<Output = T> + Sub<Output = T>,
{
    fn mul_assign(&mut self, other: Self) {
        *self = self.mul(other)
    }
}

impl<T> One for Quaternion<T>
where
    T: Copy + fmt::Debug + PartialEq + Add<Output = T> + Sub<Output = T> + Mul + One + Zero,
{
    #[inline]
    fn one() -> Self {
        Self {
            data: [T::one(), T::zero(), T::zero(), T::zero()],
        }
    }

    #[inline]
    fn is_one(&self) -> bool {
        let zero = T::zero();
        let one = T::one();
        &self[0] == &one && &self[1] == &zero && &self[2] == &zero && &self[3] == &zero
    }
}

impl<T> Index<usize> for Quaternion<T>
where
    T: Copy + fmt::Debug + PartialEq,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, Arbitrary, Gen};
    use rand::Rng;

    impl Arbitrary for Quaternion<i64> {
        fn arbitrary<G: Gen>(g: &mut G) -> Quaternion<i64> {
            Quaternion::from_iter((0..DIM4).into_iter().map(|_| g.gen()))
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
