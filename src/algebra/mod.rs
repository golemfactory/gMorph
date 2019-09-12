mod m231;
mod q231;
mod quaternion;

use nalgebra::{dimension::U2, storage::Storage, Matrix, Matrix2, Matrix3};

pub(crate) use self::m231::Mod231;
pub(crate) use self::q231::Q231;

pub trait Invertible {
    type Item;

    fn try_invert(&self) -> Option<Self::Item>;

    /// Will panic if `try_invert` fails
    fn invert(&self) -> Self::Item {
        self.try_invert()
            .unwrap_or_else(|| panic!("item non-invertible"))
    }
}

pub(crate) fn invert_2x2<S: Storage<Q231, U2, U2>>(
    matrix: &Matrix<Q231, U2, U2, S>,
) -> Option<Matrix2<Q231>> {
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
        -x_inv * b * d_inv,
        -d_inv * c * x_inv,
        y_inv,
    ))
}

pub(crate) fn invert_3x3(matrix: &Matrix3<Q231>) -> Option<Matrix3<Q231>> {
    use nalgebra::dimension::U1;

    let a = matrix[0];
    let b = matrix.fixed_slice::<U1, U2>(0, 1);
    let c = matrix.fixed_slice::<U2, U1>(1, 0);
    let d = matrix.fixed_slice::<U2, U2>(1, 1);

    let a_inv = a.try_invert()?;
    let d_inv = invert_2x2(&d)?;

    let x = a - (b * d_inv * c)[0];
    let y = d - c * a_inv * b;

    let x_inv = x.try_invert()?;
    let y_inv = invert_2x2(&y)?;

    let r1 = (b * d_inv).map(|x| -x_inv * x);
    let r2 = (-d_inv * c).map(|x| x * x_inv);

    let m = Matrix3::new(
        x_inv, r1[0], r1[1], r2[0], y_inv[0], y_inv[2], r2[1], y_inv[1], y_inv[3],
    );

    Some(m)
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{Arbitrary, Gen, TestResult};

    #[derive(Debug, Clone)]
    struct M2Q231(Matrix2<Q231>);

    impl Arbitrary for M2Q231 {
        fn arbitrary<G: Gen>(g: &mut G) -> M2Q231 {
            M2Q231(Matrix2::new(
                Q231::arbitrary(g),
                Q231::arbitrary(g),
                Q231::arbitrary(g),
                Q231::arbitrary(g),
            ))
        }
    }

    #[derive(Debug, Clone)]
    struct M3Q231(Matrix3<Q231>);

    impl Arbitrary for M3Q231 {
        fn arbitrary<G: Gen>(g: &mut G) -> M3Q231 {
            M3Q231(Matrix3::new(
                Q231::arbitrary(g),
                Q231::arbitrary(g),
                Q231::arbitrary(g),
                Q231::arbitrary(g),
                Q231::arbitrary(g),
                Q231::arbitrary(g),
                Q231::arbitrary(g),
                Q231::arbitrary(g),
                Q231::arbitrary(g),
            ))
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            let iter = std::iter::empty();
            let cloned = self.clone();
            let iter = iter.chain(self.0[0].shrink().map(move |w| {
                let mut q = cloned.clone();
                q.0[0] = w;
                q
            }));
            let cloned = self.clone();
            let iter = iter.chain(self.0[1].shrink().map(move |w| {
                let mut q = cloned.clone();
                q.0[1] = w;
                q
            }));
            let cloned = self.clone();
            let iter = iter.chain(self.0[2].shrink().map(move |w| {
                let mut q = cloned.clone();
                q.0[2] = w;
                q
            }));
            let cloned = self.clone();
            let iter = iter.chain(self.0[3].shrink().map(move |w| {
                let mut q = cloned.clone();
                q.0[3] = w;
                q
            }));
            let cloned = self.clone();
            let iter = iter.chain(self.0[4].shrink().map(move |w| {
                let mut q = cloned.clone();
                q.0[4] = w;
                q
            }));
            let cloned = self.clone();
            let iter = iter.chain(self.0[5].shrink().map(move |w| {
                let mut q = cloned.clone();
                q.0[5] = w;
                q
            }));
            let cloned = self.clone();
            let iter = iter.chain(self.0[6].shrink().map(move |w| {
                let mut q = cloned.clone();
                q.0[6] = w;
                q
            }));
            let cloned = self.clone();
            let iter = iter.chain(self.0[7].shrink().map(move |w| {
                let mut q = cloned.clone();
                q.0[7] = w;
                q
            }));
            let cloned = self.clone();
            let iter = iter.chain(self.0[8].shrink().map(move |w| {
                let mut q = cloned.clone();
                q.0[8] = w;
                q
            }));
            Box::new(iter)
        }
    }

    #[quickcheck]
    fn invert_matrix2(matrix: M2Q231) -> TestResult {
        match invert_2x2(&matrix.0) {
            None => TestResult::discard(),
            Some(inverted) => TestResult::from_bool(matrix.0 * inverted == Matrix2::identity()),
        }
    }

    #[quickcheck]
    fn invert_matrix3(m: M3Q231) -> TestResult {
        match invert_3x3(&m.0) {
            None => TestResult::discard(),
            Some(inverted) => TestResult::from_bool(m.0 * inverted == Matrix3::identity()),
        }
    }
}
