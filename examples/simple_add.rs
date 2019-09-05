use gmorph::*;

fn main() {
    let key_pair = KeyPair::default();
    let enc: Vec<_> = (1..10).map(|x| Enc::encrypt(&key_pair, x)).collect();
    let enc = enc
        .into_iter()
        .fold(Enc::encrypt(&key_pair, 0), |acc, x| acc + x);
    let given = enc.decrypt(&key_pair);
    let expected: u32 = (1..10).sum();

    assert_eq!(expected, given, "the sums should be equal, and equal to 45");
}
