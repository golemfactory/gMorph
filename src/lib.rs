//! # gMorph - Fully Homomorphic Encryption library
//! `gMorph` is written entirely in Rust and is meant to be easily
//! cross-compiled to WebAssembly for use in [gWasm].
//!
//! `gMorph` is very much experimental in nature so things are expected
//! to break unexpectedly. If you find a bug, please file a bug report [here].
//!
//! [gWasm]: https://docs.golem.network/#/Products/Brass-Beta/gWASM
//! [here]: https://github.com/golemfactory/gmorph/issues
//!
//! ## Example:
//!
//! ```
//! use gmorph::*;
//!
//! let key_pair = KeyPair::default();
//! let enc: Vec<_> = (1..10).map(|x| Enc::encrypt(&key_pair, x)).collect();
//! let enc = enc
//!     .into_iter()
//!     .fold(Enc::encrypt(&key_pair, 0), |acc, x| acc + x);
//! let given = enc.decrypt(&key_pair);
//! let expected: u32 = (1..10).sum();
//!
//! assert_eq!(expected, given, "the sums should be equal, and equal to 45");
//! ```
//!
//! ## More examples:
//! You can find some more examples in [examples] folder.
//!
//! [examples]: https://github.com/golemfactory/gMorph/tree/master/examples
mod algebra;
pub mod enc;

#[macro_use]
extern crate alga_derive;

#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

pub use self::enc::{Decrypt, Enc, Encrypt, KeyPair};
