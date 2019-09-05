use gmorph::*;
use nalgebra::dimension::U5;
use nalgebra::{ArrayStorage, Matrix};

type MatrixM<N> = Matrix<N, U5, U5, ArrayStorage<N, U5, U5>>;

fn main() {
    let key_pair = KeyPair::default();
    let mut enc = MatrixM::from_iterator((1..).map(|x| Encoded::encode(x).encrypt(&key_pair)));
    for _ in 0..2 {
        enc *= enc;
    }
    let given = MatrixM::from_iterator(enc.iter().map(|x| x.decrypt(&key_pair).decode()));

    let mut expected = MatrixM::from_iterator((1u32..).into_iter());
    for _ in 0..2 {
        expected *= expected;
    }

    assert_eq!(expected, given, "the outputs should match");
}
