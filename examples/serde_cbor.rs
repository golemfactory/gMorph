use gmorph::*;
use std::fs::File;

fn main() {
    let key_pair = KeyPair::new();
    let enc: Vec<_> = (1..10).map(|x| Enc::encrypt(&key_pair, x)).collect();


    let keys_file = File::create("keys.cbor").unwrap();
    serde_cbor::to_writer(keys_file, &key_pair).unwrap();

    let enc_file = File::create("enc.cbor").unwrap();
    serde_cbor::to_writer(enc_file, &enc).unwrap();


    let enc_file = File::open("enc.cbor").unwrap();
    let deserialized: Vec<Enc> = serde_cbor::from_reader(enc_file).unwrap();

    let enc_sum = deserialized
        .into_iter()
        .fold(Enc::encrypt(&key_pair, 0), |acc, x| acc + x);

    let keys_file = File::open("keys.cbor").unwrap();
    let key_pair: KeyPair = serde_cbor::from_reader(keys_file).unwrap();

    let given = enc_sum.decrypt(&key_pair);
    let expected: u32 = (1..10).sum();

    assert_eq!(expected, given, "the sums should be equal, and equal to 45");

    println!("{}", given);
}
