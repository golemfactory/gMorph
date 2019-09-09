//! Types and traits which lift `u32` type to FHE compatible
//! `Enc` struct
use super::algebra::{invert_3x3, Mod231, Q231};
use nalgebra::Matrix3;
use num_traits::Zero;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign, Neg};

/// Wrapper type for lifting `u32` type to FHE compatible
/// form
///
/// All FHE operations (currently, addition and multiplication)
/// are defined in terms of this type.
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Enc {
    inner: Matrix3<Q231>,
}

impl Enc {
    #[inline]
    fn enc(key_pair: &KeyPair, value: u32) -> Self {
        let enc: Matrix3<_> = Q231::from(Mod231::from(value)).into();
        let inner = key_pair.forwards * enc * key_pair.backwards;

        Self { inner }
    }

    #[inline]
    fn dec(&self, key_pair: &KeyPair) -> u32 {
        let dec = key_pair.backwards * self.inner * key_pair.forwards;
        dec[0].w.0
    }
}

impl fmt::Display for Enc {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl Add for Enc {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            inner: self.inner + rhs.inner,
        }
    }
}

impl AddAssign for Enc {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Mul for Enc {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            inner: self.inner * rhs.inner,
        }
    }
}

impl MulAssign for Enc {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}

impl Sub for Enc {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            inner: self.inner - rhs.inner,
        }
    }
}

impl SubAssign for Enc {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl Neg for Enc {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self {
            inner: self.inner.neg(),
        }
    }
}

/// Type representing a key pair which can be used for encrypting
/// and decrypting data
#[derive(Debug, Serialize, Deserialize)]
pub struct KeyPair {
    forwards: Matrix3<Q231>,
    backwards: Matrix3<Q231>,
}

impl KeyPair {
    /// Generates new random key pair
    #[inline]
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let mut forwards = Matrix3::<Q231>::zero().map(|_| rng.gen::<Q231>());
        let mut maybe_backwards = invert_3x3(&forwards);

        while maybe_backwards.is_none() {
            forwards = Matrix3::<Q231>::zero().map(|_| rng.gen::<Q231>());
            maybe_backwards = invert_3x3(&forwards);
        }

        Self {
            forwards,
            backwards: maybe_backwards.unwrap(),
        }
    }
}

impl Default for KeyPair {
    /// Creates randomized key pair
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

/// Helper trait for encrypting data
pub trait Encrypt {
    type Output;

    /// Encrypts data using `key_pair` and outputs `Self::Output`
    fn encrypt(key_pair: &KeyPair, value: u32) -> Self::Output;
}

/// Helper trait for decrypting data
pub trait Decrypt {
    type Output;

    /// Decrypts `Self` using `key_pair` and outputs data
    fn decrypt(&self, key_pair: &KeyPair) -> Self::Output;
}

impl Encrypt for Enc {
    type Output = Enc;

    #[inline]
    fn encrypt(key_pair: &KeyPair, value: u32) -> Self::Output {
        Enc::enc(key_pair, value)
    }
}

impl<'a> Encrypt for &'a Enc {
    type Output = Enc;

    #[inline]
    fn encrypt(key_pair: &KeyPair, value: u32) -> Self::Output {
        Enc::enc(key_pair, value)
    }
}

impl Decrypt for Enc {
    type Output = u32;

    #[inline]
    fn decrypt(&self, key_pair: &KeyPair) -> Self::Output {
        self.dec(key_pair)
    }
}

impl<'a> Decrypt for &'a Enc {
    type Output = u32;

    #[inline]
    fn decrypt(&self, key_pair: &KeyPair) -> Self::Output {
        self.dec(key_pair)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity() {
        let key_pair = KeyPair::default();
        assert_eq!(
            key_pair.forwards * key_pair.backwards,
            key_pair.backwards * key_pair.forwards
        )
    }

    #[test]
    fn idempotence() {
        let key_pair = KeyPair::default();
        assert_eq!(1, Enc::encrypt(&key_pair, 1).decrypt(&key_pair));
    }

    /*
     #[quickcheck]
     fn paranoid_enc_mul_homomorphic(x: u32, y: u32) -> TestResult {
         if x < MODULUS && y < MODULUS {
             let key_pair = KeyPair::new();
             let x1 = Enc::encrypt(&key_pair, x);
             let y1 = Enc::encrypt(&key_pair, y);
             let r = x1 * y1;
             return TestResult::from_bool( r.decrypt(&key_pair) == normalize_u64(x as u64  * y as u64));
         }

         TestResult::discard()
     }
    */

    #[quickcheck]
    fn prop_enc_mul_homomorphic(x: u32, y: u32) -> bool {
        let key_pair = KeyPair::new();
        let enc_x = Enc::encrypt(&key_pair, x);
        let enc_y = Enc::encrypt(&key_pair, y);
        (enc_x * enc_y).decrypt(&key_pair) == x * y
    }

    #[quickcheck]
    fn prop_enc_add_homomorphic(x: u32, y: u32) -> bool {
        let key_pair = KeyPair::new();
        let enc_x = Enc::encrypt(&key_pair, x);
        let enc_y = Enc::encrypt(&key_pair, y);
        (enc_x + enc_y).decrypt(&key_pair) == x + y
    }
}
