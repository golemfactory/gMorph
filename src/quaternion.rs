use super::Invertible;
use alga::general::Ring;
use nalgebra::base::coordinates::IJKW;
use nalgebra::{Vector3, Vector4};
use num_traits::{One, Zero};
use std::fmt;
use std::mem;
use std::ops::{Add, AddAssign, Deref, DerefMut, Mul, MulAssign, Neg, Sub, SubAssign};

/// Quaternion over a ring mod N
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct QuaternionM<T>
where
    T: Ring + fmt::Debug + Copy + Invertible + 'static,
{
    inner: Vector4<T>, // [x, y, z, w] or w + xi + yj + zk
}

impl<T> QuaternionM<T>
where
    T: Ring + fmt::Debug + Copy + Invertible + 'static,
{
    #[inline]
    pub fn new(w: T, i: T, j: T, k: T) -> Self {
        Self::from(Vector4::new(i, j, k, w))
    }

    #[inline]
    pub fn from_parts(w: T, ijk: Vector3<T>) -> Self {
        Self::new(w, ijk[0], ijk[1], ijk[2])
    }

    #[inline]
    pub fn from_real(w: T) -> Self {
        Self::from_parts(w, Vector3::zero())
    }

    #[inline]
    pub fn from_imag(ijk: Vector3<T>) -> Self {
        Self::from_parts(T::zero(), ijk)
    }

    #[inline]
    pub fn imag(&self) -> Vector3<T> {
        self.inner.xyz()
    }

    #[inline]
    pub fn conjugate(&self) -> Self {
        Self::from_parts(self.w, self.imag().neg())
    }

    #[inline]
    pub fn norm2(&self) -> T {
        (*self * self.conjugate()).w
    }

    #[inline]
    pub fn scale(&self, w: T) -> Self {
        Self::from_real(w) * *self
    }
}

// impl<T> QuaternionM<T>
// where
//     T: Ring + fmt::Debug + Copy + Invertible + 'static,
// {
//     #[inline]
//     pub fn recip(&self) -> Self {
//         self.conjugate().scale(self.norm2().invert())
//     }
// }

impl<T> Deref for QuaternionM<T>
where
    T: Ring + fmt::Debug + Copy + Invertible + 'static,
{
    type Target = IJKW<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { mem::transmute(self) }
    }
}

impl<T> DerefMut for QuaternionM<T>
where
    T: Ring + fmt::Debug + Copy + Invertible + 'static,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { mem::transmute(self) }
    }
}

impl<T> fmt::Display for QuaternionM<T>
where
    T: Ring + fmt::Debug + fmt::Display + Copy + Invertible + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}+{}i+{}j+{}k", self.w, self.i, self.j, self.k)
    }
}

impl<T> From<Vector4<T>> for QuaternionM<T>
where
    T: Ring + fmt::Debug + Copy + Invertible + 'static,
{
    fn from(inner: Vector4<T>) -> Self {
        Self { inner }
    }
}

impl<T> From<[T; 4]> for QuaternionM<T>
where
    T: Ring + fmt::Debug + Copy + Invertible + 'static,
{
    fn from(arr: [T; 4]) -> Self {
        Self::from(Vector4::from(arr))
    }
}

impl<T> Add for QuaternionM<T>
where
    T: Ring + fmt::Debug + Copy + Invertible + 'static,
{
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        QuaternionM::from(self.inner + other.inner)
    }
}

impl<T> AddAssign for QuaternionM<T>
where
    T: Ring + fmt::Debug + Copy + Invertible + 'static,
{
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.inner += other.inner
    }
}

impl<T> Sub for QuaternionM<T>
where
    T: Ring + fmt::Debug + Copy + Invertible + 'static,
{
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        QuaternionM::from(self.inner - other.inner)
    }
}

impl<T> SubAssign for QuaternionM<T>
where
    T: Ring + fmt::Debug + Copy + Invertible + 'static,
{
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.inner -= other.inner
    }
}

impl<T> Neg for QuaternionM<T>
where
    T: Ring + fmt::Debug + Copy + Invertible + 'static,
{
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        self.inner.neg().into()
    }
}

impl<T> Zero for QuaternionM<T>
where
    T: Ring + fmt::Debug + Copy + Invertible + 'static,
{
    #[inline]
    fn zero() -> Self {
        Self::from(Vector4::zero())
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.inner.is_zero()
    }
}

impl<T> Mul for QuaternionM<T>
where
    T: Ring + fmt::Debug + Copy + Invertible + 'static,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let s_ijk = self.imag();
        let o_ijk = other.imag();
        Self::from_parts(
            self.w * other.w - s_ijk.dot(&o_ijk),
            s_ijk.cross(&o_ijk) + (o_ijk * self.w) + s_ijk * other.w,
        )
    }
}

impl<T> MulAssign for QuaternionM<T>
where
    T: Ring + fmt::Debug + Copy + Invertible + 'static,
{
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other
    }
}

impl<T> One for QuaternionM<T>
where
    T: Ring + fmt::Debug + Copy + Invertible + 'static,
{
    #[inline]
    fn one() -> Self {
        Self::from_real(T::one())
    }

    #[inline]
    fn is_one(&self) -> bool {
        self.w.is_one() && self.imag().is_zero()
    }
}
