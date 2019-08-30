use gmorph::prelude::*;
use nalgebra::dimension::U100;
use nalgebra::{ArrayStorage, Matrix, Matrix3};
use std::fs;
use std::io::Write;

type MatrixM<N> = Matrix<N, U100, U100, ArrayStorage<N, U100, U100>>;

fn main() {
    let mut f = fs::File::create("result").unwrap();

    let data: MatrixM<u32> = MatrixM::from_iterator((1..).into_iter());
    let mut encode: MatrixM<Matrix3<Q231>> = data.map(|x| {
        let x_mod231: Mod231 = x.into();
        let x_q231: Q231 = x_mod231.into();
        x_q231.into()
    });

    for _ in 0..10 {
        encode *= encode;
    }

    let decode: MatrixM<u32> = encode.map(|x| {
        let x_q231 = Q231::from(x);
        let x_mod231 = x_q231.w;
        x_mod231.0
    });

    let res = format!("{}", decode);
    f.write_all(res.as_bytes()).unwrap();
}
