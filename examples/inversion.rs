use gmorph::prelude::*;
use nalgebra::{Matrix2, Matrix3};
use num_traits::{One, Zero};

fn main() {
    let a = Q231::new(
        Mod231::zero(),
        Mod231::zero(),
        Mod231::zero(),
        Mod231::one(),
    );
    let b = Q231::zero();
    let c = Q231::new(
        Mod231::zero(),
        Mod231::zero(),
        Mod231::one(),
        Mod231::zero(),
    );
    let d = Q231::new(
        Mod231::zero(),
        Mod231::one(),
        Mod231::zero(),
        Mod231::zero(),
    );
    let matrix = Matrix2::new(a, b, c, d);
    let inv_matrix = invert_2x2(&matrix).unwrap();

    println!("matrix = {}", matrix);
    println!("inverted = {}", inv_matrix);
    println!("left mul = {}", matrix * inv_matrix);
    println!("right mul = {}", inv_matrix * matrix);

    let mut m: Matrix3<Q231> = Matrix3::identity();
    m[0] = a;
    m[1] = c;
    m[4] = d;
    let m_inv = invert_3x3(&m).unwrap();
    println!("m = {}", m);
    println!("m_inv = {}", m_inv);
    println!("left mul = {}", m * m_inv);
    println!("right mul = {}", m_inv * m);
}
