use gmorph::prelude::*;
use nalgebra::Matrix2;
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
}
