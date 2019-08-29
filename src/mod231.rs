use modinverse::modinverse;
use num_traits::identities::{One, Zero};
use std::fmt;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};

const MODULUS: i64 = 2147483647;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mod231(pub i64);

impl Mod231 {
    pub fn reciprocal(&self) -> Option<Self> {
        modinverse(self.0, MODULUS).map(Mod231)
    }
}

impl fmt::Display for Mod231 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Zero for Mod231 {
    fn zero() -> Self {
        Self(0)
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl One for Mod231 {
    fn one() -> Self {
        Self(1)
    }

    fn is_one(&self) -> bool {
        self.0.is_one()
    }
}

#[inline]
fn normalize(x: i64) -> i64 {
    ((x % MODULUS) + MODULUS) % MODULUS
}

impl From<i64> for Mod231 {
    fn from(x: i64) -> Self {
        Self(normalize(x))
    }
}

impl Add for Mod231 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(normalize(self.0 + other.0))
    }
}

impl AddAssign for Mod231 {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0
    }
}

impl Mul for Mod231 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self(normalize(self.0 * other.0))
    }
}

impl MulAssign for Mod231 {
    fn mul_assign(&mut self, other: Self) {
        self.0 *= other.0
    }
}

impl Neg for Mod231 {
    type Output = Mod231;

    fn neg(self) -> Self::Output {
        Mod231(MODULUS - normalize(self.0))
    }
}

impl Sub for Mod231 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(normalize(self.0 + other.neg().0))
    }
}

impl SubAssign for Mod231 {
    fn sub_assign(&mut self, other: Self) {
        *self = self.sub(other)
    }
}

impl Div for Mod231 {
    type Output = Option<Self>;

    fn div(self, other: Self) -> Self::Output {
        other.reciprocal().map(|x| Mod231(normalize(self.0 * x.0)))
    }
}

impl PartialEq<i64> for Mod231 {
    fn eq(&self, other: &i64) -> bool {
        self.0 == *other
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, Arbitrary, Gen, TestResult};
    use rand::Rng;

    impl Arbitrary for Mod231 {
        fn arbitrary<G: Gen>(g: &mut G) -> Mod231 {
            let i = g.gen_range(0, MODULUS);
            Mod231(i)
        }
    }

    quickcheck! {
        fn prop_normalize(x: i64) -> bool {
            normalize(x) == ((x % MODULUS) + MODULUS) % MODULUS
        }
    }

    quickcheck! {
      fn double_negate_is_identity(x: Mod231) -> bool {
          x == x.neg().neg()
      }
    }

    quickcheck! {
        fn x_mul_recip_x(x: Mod231) -> TestResult {
            if x == Mod231(0) {
                return TestResult::discard()
            }

            TestResult::from_bool(x * x.reciprocal().unwrap() == Mod231(1) )
        }
    }
}
