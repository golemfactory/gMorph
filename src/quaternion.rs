use super::RingM;
use nalgebra::base::coordinates::IJKW;
use nalgebra::{Vector3, Vector4};
use num_traits::{One, Zero};
use std::fmt;
use std::mem;
use std::ops::{Add, AddAssign, Deref, DerefMut, Mul, MulAssign, Neg};

/// Quaternion over a ring mod N
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct QuaternionM<T>
where
    T: RingM + 'static,
{
    inner: Vector4<T>, // [x, y, z, w] or w + xi + yj + zk
}

impl<T> QuaternionM<T>
where
    T: RingM,
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

impl<T> QuaternionM<T>
where
    T: RingM<Item = T>,
{
    #[inline]
    pub fn recip(&self) -> Self {
        self.conjugate().scale(self.norm2().invert())
    }
}

impl<T> Deref for QuaternionM<T>
where
    T: RingM,
{
    type Target = IJKW<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { mem::transmute(self) }
    }
}

impl<T> DerefMut for QuaternionM<T>
where
    T: RingM,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { mem::transmute(self) }
    }
}

impl<T> fmt::Display for QuaternionM<T>
where
    T: RingM + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}+{}i+{}j+{}k", self.w, self.i, self.j, self.k)
    }
}

impl<T> From<Vector4<T>> for QuaternionM<T>
where
    T: RingM + 'static,
{
    fn from(inner: Vector4<T>) -> Self {
        Self { inner }
    }
}

impl<T> From<[T; 4]> for QuaternionM<T>
where
    T: RingM,
{
    fn from(arr: [T; 4]) -> Self {
        Self::from(Vector4::from(arr))
    }
}

impl<T> Add for QuaternionM<T>
where
    T: RingM,
{
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        QuaternionM::from(self.inner + other.inner)
    }
}

impl<T> AddAssign for QuaternionM<T>
where
    T: RingM,
{
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.inner += other.inner
    }
}

impl<T> Neg for QuaternionM<T>
where
    T: RingM,
{
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        self.inner.neg().into()
    }
}

impl<T> Zero for QuaternionM<T>
where
    T: RingM,
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
    T: RingM,
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
    T: RingM,
{
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other
    }
}

impl<T> One for QuaternionM<T>
where
    T: RingM,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mod231::Mod231;
    use quickcheck::{quickcheck, Arbitrary, Gen, TestResult};

    type Q231 = QuaternionM<Mod231>;

    impl Arbitrary for Q231 {
        fn arbitrary<G: Gen>(g: &mut G) -> Q231 {
            Q231::new(
                Mod231::arbitrary(g),
                Mod231::arbitrary(g),
                Mod231::arbitrary(g),
                Mod231::arbitrary(g),
            )
        }
    }

    quickcheck! {
      fn add_commutative(a: Q231, b: Q231) -> bool {
          a + b == b + a
      }
    }

    quickcheck! {
        fn prop_conjugate1(a: Q231) -> bool {
            a + a.conjugate() == Q231::from(Mod231::from(2) * a.w)
        }
    }

    quickcheck! {
        fn prop_conjugate2(a: Q231) -> bool {
            (a + a.conjugate()).imag() == Vector3::zero()
        }
    }

    quickcheck! {
        fn prop_recip_right(a: Q231) -> TestResult {
            if a.is_zero() {
                TestResult::discard()
            } else {
                TestResult::from_bool(a * a.recip() == Q231::one())
            }
        }
    }

    quickcheck! {
        fn prop_recip_left(a: Q231) -> TestResult {
            if a.is_zero() {
                TestResult::discard()
            } else {
                TestResult::from_bool(a.recip() * a == Q231::one())
            }
        }
    }

    // quickcheck! {
    //     fn prop_into_matrix_and_back(a: Q231) -> bool {
    //         let b: Matrix3<Q231> = a.into();
    //         Q231::from(b) == a
    //     }
    // }

    // quickcheck! {
    //     fn multiply_matrices(a: Q231) -> bool {
    //         let m1: Matrix3<Q231> = a.into();
    //         Q231::from(m1 *m1) == a * a
    //     }
    // }
}
