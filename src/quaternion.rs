use crate::vector::Vector4;
use num_traits::{One, Zero};
use std::fmt;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg};

/// Quaternion
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Quaternion<T>
where
    T: Copy + Clone + fmt::Debug + PartialEq,
{
    data: Vector4<T>, // data[0] + data[1]*i + data[2]*j + data[3]*k
}

impl<T> From<Vector4<T>> for Quaternion<T>
where
    T: Copy + Clone + fmt::Debug + PartialEq,
{
    fn from(data: Vector4<T>) -> Self {
        Self { data }
    }
}

impl<T> Add for Quaternion<T>
where
    T: Copy + Clone + fmt::Debug + PartialEq + Add + Zero,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::from(self.data + other.data)
    }
}

impl<T> Zero for Quaternion<T>
where
    T: Copy + Clone + fmt::Debug + PartialEq + Add + Zero,
{
    fn zero() -> Self {
        Self::from(Vector4::zero())
    }

    fn is_zero(&self) -> bool {
        self.data.is_zero()
    }
}
