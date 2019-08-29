pub mod matrix;
pub mod mod231;
pub mod quaternion;

use mod231::Mod231;
use quaternion::Quaternion;

pub type Q231 = Quaternion<Mod231>;
