//! Types and traits which allow for encrypting/decrypting
//! of `Encoded` data type
use super::algebra::Q231;
use super::encode::Encoded;
use nalgebra::Matrix3;

/// Type representing a key pair which can be used for encrypting
/// and decrypting `Encoded` data
#[derive(Debug)]
pub struct KeyPair {
    forwards: Matrix3<Q231>,
    backwards: Matrix3<Q231>,
}

impl Default for KeyPair {
    fn default() -> Self {
        Self {
            forwards: Matrix3::identity(),
            backwards: Matrix3::identity(),
        }
    }
}

/// Helper trait for encrypting `Self`
pub trait Encrypt {
    type Output;

    /// Encrypts `Self` using `key_pair` and outputs `Self::Output`
    fn encrypt(&self, key_pair: &KeyPair) -> Self::Output;
}

/// Helper trait for decrypting `Self`
pub trait Decrypt {
    type Output;

    /// Decrypts `Self` using `key_pair` and outputs `Self::Output`
    fn decrypt(&self, key_pair: &KeyPair) -> Self::Output;
}

impl Encrypt for Encoded {
    type Output = Encoded;

    fn encrypt(&self, key_pair: &KeyPair) -> Self::Output {
        (key_pair.forwards * self.inner * key_pair.backwards).into()
    }
}

impl<'a> Encrypt for &'a Encoded {
    type Output = Encoded;

    fn encrypt(&self, key_pair: &KeyPair) -> Self::Output {
        (key_pair.forwards * self.inner * key_pair.backwards).into()
    }
}

impl Decrypt for Encoded {
    type Output = Encoded;

    fn decrypt(&self, key_pair: &KeyPair) -> Self::Output {
        (key_pair.backwards * self.inner * key_pair.forwards).into()
    }
}

impl<'a> Decrypt for &'a Encoded {
    type Output = Encoded;

    fn decrypt(&self, key_pair: &KeyPair) -> Self::Output {
        (key_pair.backwards * self.inner * key_pair.forwards).into()
    }
}
