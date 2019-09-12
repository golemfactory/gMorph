use gmorph::*;

use std::fs::File;
use std::io::prelude::*;

fn encrypt_vec(key_pair: &KeyPair, v: Vec<u32>) -> Vec<Enc> {
    v.into_iter().map(|x| Enc::encrypt(&key_pair, x)).collect()
}

// input.json of the form
// [[1,2,3],[2,4,6]]
fn main() {
    let key_pair = KeyPair::new();
    //    let x = vec!(1,2,3);
    //    let y = vec!(2,4,6);

    let mut vectors_file = File::open("input.json").expect("Failed to open input.json");

    let mut serialized_vectors = String::new();
    vectors_file
        .read_to_string(&mut serialized_vectors)
        .expect("Failed to read input.json");

    let (x, y): (Vec<u32>, Vec<u32>) =
        serde_json::from_str(&serialized_vectors).expect("Failed to deserialize input vectors");

    let enc_x = encrypt_vec(&key_pair, x);
    let enc_y = encrypt_vec(&key_pair, y);

    let data = serde_json::to_string(&(enc_x, enc_y)).unwrap();
    let serialized_keypair = serde_json::to_string(&key_pair).unwrap();

    let mut data_file = File::create("data.json").unwrap();
    data_file
        .write_all(data.as_bytes())
        .expect("Failed to write data.json");

    let mut keys_file = File::create("keys.json").unwrap();
    keys_file
        .write_all(serialized_keypair.as_bytes())
        .expect("Failed to write keys.json")
}
