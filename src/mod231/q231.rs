use super::m231::Mod231;
use crate::quaternion::QuaternionM;
use nalgebra::{Matrix3, Vector4};
use num_traits::Zero;
use rand::distributions::{Distribution, Standard};
use rand::{thread_rng, Rng};

pub type Q231 = QuaternionM<Mod231>;

impl From<Mod231> for Q231 {
    fn from(w: Mod231) -> Self {
        Self::from_real(w)
    }
}

impl From<Matrix3<Q231>> for Q231 {
    fn from(m: Matrix3<Q231>) -> Self {
        m[0]
    }
}

impl Distribution<Q231> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Q231 {
        Q231::from(Vector4::<Mod231>::zero().map(|_| rng.gen::<Mod231>()))
    }
}

impl Into<Matrix3<Q231>> for Q231 {
    fn into(self) -> Matrix3<Q231> {
        let mut noise = Matrix3::<Q231>::zeros()
            .map(|_| thread_rng().gen::<Q231>())
            .upper_triangle();
        noise[0] = self;
        noise
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mod231::Mod231;
    use nalgebra::Vector3;
    use num_traits::{One, Zero};
    use quickcheck::{quickcheck, Arbitrary, Gen, TestResult};

    impl Arbitrary for Q231 {
        fn arbitrary<G: Gen>(g: &mut G) -> Q231 {
            Q231::new(
                Mod231::arbitrary(g),
                Mod231::arbitrary(g),
                Mod231::arbitrary(g),
                Mod231::arbitrary(g),
            )
        }
    }

    quickcheck! {
      fn add_commutative(a: Q231, b: Q231) -> bool {
          a + b == b + a
      }
    }

    quickcheck! {
        fn prop_conjugate1(a: Q231) -> bool {
            a + a.conjugate() == Q231::from(Mod231::from(2) * a.w)
        }
    }

    quickcheck! {
        fn prop_conjugate2(a: Q231) -> bool {
            (a + a.conjugate()).imag() == Vector3::zero()
        }
    }

    quickcheck! {
        fn prop_recip_right(a: Q231) -> TestResult {
            if a.is_zero() {
                TestResult::discard()
            } else {
                TestResult::from_bool(a * a.recip() == Q231::one())
            }
        }
    }

    quickcheck! {
        fn prop_recip_left(a: Q231) -> TestResult {
            if a.is_zero() {
                TestResult::discard()
            } else {
                TestResult::from_bool(a.recip() * a == Q231::one())
            }
        }
    }

    quickcheck! {
        fn prop_into_matrix_and_back(a: Q231) -> bool {
            let b: Matrix3<Q231> = a.into();
            Q231::from(b) == a
        }
    }

    quickcheck! {
        fn multiply_matrices(a: Q231) -> bool {
            let m1: Matrix3<Q231> = a.into();
            Q231::from(m1 *m1) == a * a
        }
    }
}
