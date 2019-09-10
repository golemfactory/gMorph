use gmorph::*;

fn main() {
    let key_pair = KeyPair::new();
    let enc: Vec<_> = (1..10).map(|x| Enc::encrypt(&key_pair, x)).collect();

    let serialized = serde_json::to_string(&enc).unwrap();
    let serialized_keypair = serde_json::to_string(&key_pair).unwrap();

    let deserialized: Vec<Enc> = serde_json::from_str(&serialized).unwrap();
    let enc_sum = deserialized
        .into_iter()
        .fold(Enc::encrypt(&key_pair, 0), |acc, x| acc + x);

    let key_pair: KeyPair = serde_json::from_str(&serialized_keypair).unwrap();
    let given = enc_sum.decrypt(&key_pair);
    let expected: u32 = (1..10).sum();

    assert_eq!(expected, given, "the sums should be equal, and equal to 45");

    println!("{}", given);
}
