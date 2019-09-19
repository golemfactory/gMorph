use super::{invert_3x3, m231::try_sqrt, m231::Mod231, quaternion::QuaternionM, Invertible};
use nalgebra::{Matrix3, Vector4};
use num_traits::Zero;
use rand::distributions::{Distribution, Standard};
use rand::{thread_rng, Rng};
use std::ops::Neg;

pub(crate) type Q231 = QuaternionM<Mod231>;

impl From<Mod231> for Q231 {
    fn from(w: Mod231) -> Self {
        Self::from_real(w)
    }
}

impl Invertible for Q231 {
    type Item = Q231;

    fn try_invert(&self) -> Option<Self::Item> {
        let norm = self.norm2();
        let renorm = norm.try_invert()?;
        Some(self.conjugate().scale(renorm))
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
        // Make sure the middle element (and hence the matrix) is noninvertible
        // The loop should not run many times - statistically every 2nd element has a square root
        loop {
            let c: Q231 = thread_rng().gen::<Q231>();
            let y: Mod231 = c.i * c.i + c.j * c.j + c.k * c.k;
            if let Some(x) = try_sqrt(y.neg()) {
                noise[4] = Q231::new(x, c.i, c.j, c.k);
                assert_eq!(noise[4].norm2(), Mod231(0));
                break;
            }
        }
        noise[0] = self;
        noise
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Vector3;
    use num_traits::{One, Zero};
    use quickcheck::{Arbitrary, Gen, TestResult};

    impl Arbitrary for Q231 {
        fn arbitrary<G: Gen>(g: &mut G) -> Q231 {
            Q231::new(
                Mod231::arbitrary(g),
                Mod231::arbitrary(g),
                Mod231::arbitrary(g),
                Mod231::arbitrary(g),
            )
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            let iter = std::iter::empty();
            let cloned = self.clone();
            let iter = iter.chain(self.w.shrink().map(move |w| {
                let mut q = cloned.clone();
                q.w = w;
                q
            }));
            let cloned = self.clone();
            let iter = iter.chain(self.i.shrink().map(move |i| {
                let mut q = cloned.clone();
                q.i = i;
                q
            }));
            let cloned = self.clone();
            let iter = iter.chain(self.j.shrink().map(move |j| {
                let mut q = cloned.clone();
                q.j = j;
                q
            }));
            let cloned = self.clone();
            let iter = iter.chain(self.k.shrink().map(move |k| {
                let mut q = cloned.clone();
                q.k = k;
                q
            }));
            Box::new(iter)
        }
    }

    #[quickcheck]
    fn add_commutative(a: Q231, b: Q231) -> bool {
        a + b == b + a
    }

    #[quickcheck]
    fn prop_conjugate1(a: Q231) -> bool {
        a + a.conjugate() == Q231::from(Mod231::from(2) * a.w)
    }

    #[quickcheck]
    fn prop_conjugate2(a: Q231) -> bool {
        (a + a.conjugate()).imag() == Vector3::zero()
    }

    #[quickcheck]
    fn prop_recip_right(a: Q231) -> TestResult {
        match a.try_invert() {
            None => TestResult::discard(),
            Some(a_inv) => TestResult::from_bool(a * a_inv == Q231::one()),
        }
    }

    #[quickcheck]
    fn prop_recip_left(a: Q231) -> TestResult {
        if a.is_zero() {
            TestResult::discard()
        } else {
            TestResult::from_bool(a.invert() * a == Q231::one())
        }
    }

    #[quickcheck]
    fn prop_into_matrix_and_back(a: Q231) -> bool {
        let b: Matrix3<Q231> = a.into();
        Q231::from(b) == a
    }

    #[quickcheck]
    fn multiply_matrices(a: Q231) -> bool {
        let m1: Matrix3<Q231> = a.into();
        Q231::from(m1 * m1) == a * a
    }

    #[quickcheck]
    fn prop_noninvertible(a: Q231) -> bool {
        let b: Matrix3<Q231> = a.into();
        invert_3x3(&b) == None
    }
}
