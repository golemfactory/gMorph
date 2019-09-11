use gmorph::*;

use std::fs::File;
use std::io::prelude::*;

/// args - commandline arguments
fn split(args : Vec<String>) -> Vec<(Vec<Enc>, Vec<Enc>)> {
    let mut data_file = std::fs::File::open("data.json")
        .expect("Failed to open data.json");
    let mut serialized = String::new();
    data_file.read_to_string(&mut serialized).unwrap();
    let data : (Vec<Enc>, Vec<Enc>) = serde_json::from_str(&serialized).unwrap();
    vec![data]
}

fn execute(data : (Vec<Enc>, Vec<Enc>)) -> (Enc, Enc) {

    let x = data.0;
    let y = data.1;
    let xy = dot_product_enc(&x, &y);
    let xx = dot_product_enc(&x, &x);
    let serialized_result = serde_json::to_string(&(xy, xx)).unwrap();
    (xy, xx)
}


fn merge(args : Vec<String>, data: Vec<(Vec<Enc>, Vec<Enc>)>, results: Vec<(Enc,Enc)>) {
    let mut keys_file = std::fs::File::open("keys.json").unwrap();
    let mut serialized_keypair = String::new();
    keys_file.read_to_string(&mut serialized_keypair).unwrap();
    let key_pair: KeyPair = serde_json::from_str(&serialized_keypair).unwrap();

    let a: u32 = (&results).into_iter().map(|p| p.0.decrypt(&key_pair)).sum();
    let b: u32 = results.into_iter().map(|p| p.1.decrypt(&key_pair)).sum();
    let m = a as f64 / b as f64;
    println!("m = {}", m);
}


fn main() {

}
fn dot_product_enc(v: &Vec<Enc>, w: &Vec<Enc>) -> Enc
{
    let length = v.len();
    // We expect both vectors to have the same number of elements
    assert_eq!(length, w.len());
    assert!(length > 0);

    let mut sum = v[0]*w[0];

    for index in 0..length {
        sum = sum + v[index] * w[index];
    }
    sum
}
