use num_traits::{One, Zero};
use std::fmt;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

/// Statically-sized Vector of 4 elements
const DIM4: usize = 4;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vector4<T>
where
    T: fmt::Debug + PartialEq + Copy + Clone,
{
    data: [T; DIM4],
}

impl<T> Vector4<T>
where
    T: Copy + fmt::Debug + PartialEq + Copy + Clone + Zero,
{
    pub fn from_iter(it: impl Iterator<Item = T>) -> Self {
        let mut data = [T::zero(); DIM4];
        for (x, i) in it.zip(0..DIM4) {
            data[i] = x;
        }
        Self { data }
    }
}

impl<T> Add for Vector4<T>
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

impl<T> AddAssign for Vector4<T>
where
    T: Copy + fmt::Debug + Zero + AddAssign + PartialEq,
{
    #[inline]
    fn add_assign(&mut self, other: Self) {
        for i in 0..DIM4 {
            self.data[i] += other.data[i];
        }
    }
}

impl<T> Zero for Vector4<T>
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
        &self.data[0] == &z && &self.data[1] == &z && &self.data[2] == &z
    }
}

/// Dynamically-sized Vector of N elements
pub struct Vector<T> {
    data: Vec<T>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, Arbitrary, Gen};
    use rand::Rng;

    impl Arbitrary for Vector4<i64> {
        fn arbitrary<G: Gen>(g: &mut G) -> Vector4<i64> {
            Vector4::from_iter((0..DIM4).into_iter().map(|_| g.gen()))
        }
    }

    quickcheck! {
        fn vector4_add(x: Vector4<i64>) -> bool {
            x + Vector4::zero() == x
        }
    }
}
