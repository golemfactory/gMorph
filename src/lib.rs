mod algebra;
pub mod encode;
pub mod encrypt;

#[macro_use]
extern crate alga_derive;

#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

pub mod prelude {
    pub use super::encode::{Decode, Encode, Encoded};
    pub use super::encrypt::{Decrypt, Encrypt, KeyPair};
}
