use gmorph::*;

use std::fs::File;
use std::io::prelude::*;

fn encrypt_vec(key_pair: &KeyPair, v: Vec<u32>) -> Vec<Enc> {
    v.into_iter().map(|x| Enc::encrypt(&key_pair, x)).collect()
}

fn main() {
    let key_pair = KeyPair::new();
    let x = vec!(1,2,3);
    let y = vec!(2,4,6);

    let enc_x = encrypt_vec(&key_pair, x);
    let enc_y = encrypt_vec(&key_pair, y);

    let data = serde_json::to_string(&(enc_x, enc_y)).unwrap();
    let serialized_keypair = serde_json::to_string(&key_pair).unwrap();


    let mut data_file = File::create("data.json").unwrap();
    data_file.write_all(data.as_bytes());

    let mut keys_file = File::create("keys.json").unwrap();
    keys_file.write_all(serialized_keypair.as_bytes());

}
