use nalgebra::dimension::U50;
use nalgebra::{ArrayStorage, Matrix};
use std::fs;
use std::io::Write;

type MatrixM<N> = Matrix<N, U50, U50, ArrayStorage<N, U50, U50>>;

fn main() {
    let mut f = fs::File::create("result").unwrap();

    let mut data: MatrixM<u32> = MatrixM::from_iterator((1..).into_iter());

    for _ in 0..1_000 {
        data *= data;
    }

    let res = format!("{}", data);
    f.write_all(res.as_bytes()).unwrap();
}
