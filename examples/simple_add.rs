use gmorph::prelude::*;
use num_traits::Zero;

fn main() {
    let key_pair = KeyPair::default();
    let enc: Vec<_> = (1..10)
        .map(|x| Encoded::encode(x).encrypt(&key_pair))
        .collect();
    let enc = enc.into_iter().fold(Encoded::zero(), |acc, x| acc + x);
    let given = enc.decrypt(&key_pair).decode();
    let expected: u32 = (1..10).sum();

    assert_eq!(expected, given, "the sums should be equal, and equal to 45");
}
