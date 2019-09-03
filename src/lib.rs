pub mod mod231;
pub mod quaternion;

#[macro_use]
extern crate alga_derive;

#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use self::mod231::Q231;
use nalgebra::{Matrix2, Matrix3};

pub trait Invertible {
    type Item;

    fn try_invert(&self) -> Option<Self::Item>;

    /// Will panic if `try_invert` fails
    fn invert(&self) -> Self::Item {
        self.try_invert()
            .unwrap_or_else(|| panic!("item non-invertible"))
    }
}

pub fn invert_2x2(matrix: &Matrix2<Q231>) -> Option<Matrix2<Q231>> {
    let a = matrix[0];
    let b = matrix[2];
    let c = matrix[1];
    let d = matrix[3];
    let a_inv = a.try_invert()?;
    let d_inv = d.try_invert()?;

    let x = a - b * d_inv * c;
    let y = d - c * a_inv * b;
    let x_inv = x.try_invert()?;
    let y_inv = y.try_invert()?;

    Some(Matrix2::new(
        x_inv,
        -a_inv * b * y_inv,
        -y_inv * c * a_inv,
        y_inv,
    ))
}

pub fn invert_3x3(matrix: Matrix3<Q231>) -> Option<Matrix3<Q231>> {
    None
}

pub mod prelude {
    pub use super::invert_2x2;
    pub use super::mod231::{Mod231, Q231};
    pub use super::quaternion::QuaternionM;
    pub use super::Invertible;
}

#[cfg(test)]
mod tests {
    // use super::mod231::Mod231;
    use super::*;
    use quickcheck::TestResult;

    #[quickcheck]
    fn invert_matrix2(a: Q231, b: Q231, c: Q231, d: Q231) -> TestResult {
        let matrix = Matrix2::new(a, b, c, d);
        match invert_2x2(&matrix) {
            None => TestResult::discard(),
            Some(inverted) => {
                let res = matrix * inverted;
                let res2 = inverted * matrix;
                let cmp = res == Matrix2::identity();

                if !cmp {
                    println!(
                        "inverted={}, matrix={}, res={}, res2 = {}",
                        inverted, matrix, res, res2
                    );
                }
                TestResult::from_bool(matrix * inverted == Matrix2::identity())
            }
        }
    }
}
