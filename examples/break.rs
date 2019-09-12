use gmorph::*;
use num_traits::{Zero,One};

const MODULUS: u32 = 2147483647u32;

fn trybreak(x: Enc) -> Option<u32> {
    let mut g = 0;
    let mut y = x;

    while g < MODULUS {
        if y.invertible() {
            g = g+1;
            y = y - Enc::one();
        }
        else{
            return Some(g);
        };
    }
    None

}

fn main() {
    let key_pair = KeyPair::new();
    let start = 100_000;
    let len = 100;
    for i in start..start+len {
        let x = Enc::encrypt(&key_pair, i);
        let r = trybreak(x);
        match r {
            Some(g) => println!("Si! i={} g={}", i, g),
            None => println!("No!, i={}", i)
        }
    }
//    assert_eq!(expected, given, "the sums should be equal, and equal to 45");
}
