pub mod mod231;
pub mod quaternion;

#[macro_use]
extern crate alga_derive;

use alga::general::Ring;
use std::fmt::Debug;

pub trait RingM: Ring + Debug + Copy {
    type Item;

    fn try_invert(&self) -> Option<Self::Item>;

    /// Will panic if `try_invert` fails
    fn invert(&self) -> Self::Item {
        self.try_invert()
            .unwrap_or_else(|| panic!("item non-invertible"))
    }
}
