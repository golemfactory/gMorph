pub mod matrix;
pub mod mod231;
pub mod quaternion;

#[macro_use]
extern crate alga_derive;

use mod231::Mod231;
use quaternion::Quaternion;

pub trait Invertible {
    type Item;

    fn try_invert(&self) -> Option<Self::Item>;
}
