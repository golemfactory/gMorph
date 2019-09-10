use gmorph::*;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut keys_file = std::fs::File::open("keys.json").unwrap();
    let mut serialized_keypair = String::new();
    keys_file.read_to_string(&mut serialized_keypair).unwrap();
    let key_pair: KeyPair = serde_json::from_str(&serialized_keypair).unwrap();

    let mut result_file = std::fs::File::open("result.json").unwrap();
    let mut serialized = String::new();
    result_file.read_to_string(&mut serialized).unwrap();
    let enc_result : (Enc, Enc) = serde_json::from_str(&serialized).unwrap();
    let a = enc_result.0.decrypt(&key_pair);
    let b = enc_result.1.decrypt(&key_pair);
    let m = a as f64 / b as f64;
    println!("m = {}", m);
}
