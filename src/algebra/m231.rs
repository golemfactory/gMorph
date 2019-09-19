use super::Invertible;
use alga::general::{AbstractMagma, Additive, Identity, Multiplicative, TwoSidedInverse};
use num_traits::identities::{One, Zero};
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};

const MODULUS: u32 = 2147483647u32;
const MODULUSI32: i32 = 2147483647i32;
const MODULUSI64: i64 = 2147483647i64;
const MODULUSU64: u64 = 2147483647u64;

#[derive(Clone, Copy, PartialEq, Alga, Serialize, Deserialize)]
#[alga_traits(Ring(Additive, Multiplicative))]
pub struct Mod231(pub u32);

impl Invertible for Mod231 {
    type Item = Mod231;

    fn try_invert(&self) -> Option<Self::Item> {
        modinverse::modinverse(self.0 as i64, MODULUSI64).map(|x| Mod231(x as u32))
    }
}

impl fmt::Debug for Mod231 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for Mod231 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl Distribution<Mod231> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Mod231 {
        Mod231::from(rng.gen::<u32>())
    }
}

impl Zero for Mod231 {
    fn zero() -> Self {
        Mod231(0)
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl One for Mod231 {
    fn one() -> Self {
        Mod231(1)
    }

    fn is_one(&self) -> bool {
        self.0.is_one()
    }
}

#[inline]
fn modulo(mut v: u32) -> u32 {
    if v >= MODULUS {
        v = (v >> 31) + (v & MODULUS);
        while v >= MODULUS {
            v -= MODULUS;
        }
    }
    v
}

#[inline]
fn normalize_u64(mut v: u64) -> u32 {
    if v >= MODULUSU64 {
        v = (v >> 31) + (v & MODULUSU64);
        while v >= MODULUSU64 {
            v -= MODULUSU64;
        }
    }
    v as u32
}

#[inline]
fn normalize(x: u32) -> u32 {
    modulo(x)
}

impl From<u32> for Mod231 {
    fn from(x: u32) -> Self {
        Mod231(normalize(x))
    }
}

impl From<i32> for Mod231 {
    fn from(x: i32) -> Self {
        let y: u32 = if x < 0 {
            let y = x + MODULUSI32;
            if y < 0 {
                panic!("i32 out of range for Mod231")
            } else {
                y as u32
            }
        } else {
            if x < MODULUSI32 {
                x as u32
            } else {
                panic!("i32 out of range for Mod231")
            }
        };

        Mod231(normalize(y))
    }
}

impl Into<i32> for Mod231 {
    fn into(self) -> i32 {
        let y: u32 = self.0;
        let max = MODULUS / 2;

        if y > max {
            (y as i32) - MODULUSI32
        } else {
            y as i32
        }
    }
}

impl Add for Mod231 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(normalize(self.0 + other.0))
    }
}

impl AddAssign for Mod231 {
    fn add_assign(&mut self, other: Self) {
        *self = self.add(other)
    }
}

impl Mul for Mod231 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self(normalize_u64(self.0 as u64 * other.0 as u64))
    }
}

impl MulAssign for Mod231 {
    fn mul_assign(&mut self, other: Self) {
        *self = self.mul(other)
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

    fn sub(self, other: Self) -> Self::Output {
        Self(normalize(self.0 + other.neg().0))
    }
}

impl SubAssign for Mod231 {
    fn sub_assign(&mut self, other: Self) {
        *self = self.sub(other)
    }
}

impl Div for Mod231 {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self(normalize(self.0 * other.invert().0))
    }
}

impl Identity<Additive> for Mod231 {
    fn identity() -> Self {
        Self::zero()
    }
}

impl TwoSidedInverse<Additive> for Mod231 {
    fn two_sided_inverse(&self) -> Self {
        Self::zero() - *self
    }
}

impl Identity<Multiplicative> for Mod231 {
    fn identity() -> Self {
        Self::one()
    }
}

impl TwoSidedInverse<Multiplicative> for Mod231 {
    fn two_sided_inverse(&self) -> Self {
        Self::one() / *self
    }
}

impl AbstractMagma<Additive> for Mod231 {
    fn operate(&self, right: &Self) -> Self {
        *self + *right
    }
}

impl AbstractMagma<Multiplicative> for Mod231 {
    fn operate(&self, right: &Self) -> Self {
        *self * *right
    }
}

impl PartialEq<u32> for Mod231 {
    fn eq(&self, other: &u32) -> bool {
        self.0 == *other
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{Arbitrary, Gen, TestResult};
    use rand::Rng;

    impl Arbitrary for Mod231 {
        fn arbitrary<G: Gen>(g: &mut G) -> Mod231 {
            let i = g.gen_range(0, MODULUS);
            Mod231(i)
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            Box::new(self.0.shrink().map(Mod231::from))
        }
    }

    #[quickcheck]
    fn prop_normalize(x: u32) -> bool {
        normalize(x) == ((x % MODULUS) + MODULUS) % MODULUS
    }

    #[quickcheck]
    fn double_negate_is_identity(x: Mod231) -> bool {
        x == x.neg().neg()
    }

    #[quickcheck]
    fn x_mul_invert_x(x: Mod231) -> TestResult {
        if x == Mod231(0) {
            return TestResult::discard();
        }

        TestResult::from_bool(x * x.invert() == Mod231(1))
    }

    #[quickcheck]
    fn prop_roundtrip_i32(x: Mod231) -> bool {
        let y: i32 = x.into();
        let z: Mod231 = Mod231::from(y);
        x == z
    }

}
