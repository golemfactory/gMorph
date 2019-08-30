use gmorph::prelude::*;
use nalgebra::{dimension, Matrix3, MatrixN};
use std::fs;
use std::io::Write;

type Matrix10<T> = MatrixN<T, dimension::U10>;

fn main() {
    let mut f = fs::File::create("result").unwrap();

    let data: Matrix10<u32> = Matrix10::from_iterator((1..).into_iter());
    let mut encode: Matrix10<Matrix3<Q231>> = data.map(|x| {
        let x_mod231: Mod231 = x.into();
        let x_q231: Q231 = x_mod231.into();
        x_q231.into()
    });

    for _ in 0..10 {
        encode *= encode;
    }

    let decode: Matrix10<u32> = encode.map(|x| {
        let x_q231 = Q231::from(x);
        let x_mod231 = x_q231.w;
        x_mod231.0
    });

    let res = format!("{}", decode);
    f.write_all(res.as_bytes()).unwrap();
}
