use gmorph::*;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut data_file = std::fs::File::open("data.json").expect("Failed to open data.json");
    let mut serialized = String::new();
    data_file.read_to_string(&mut serialized).unwrap();
    let data: (Vec<Enc>, Vec<Enc>) = serde_json::from_str(&serialized).unwrap();
    let x = data.0;
    let y = data.1;
    let xy = dot_product_enc(&x, &y);
    let xx = dot_product_enc(&x, &x);
    let serialized_result = serde_json::to_string(&(xy, xx)).unwrap();
    let mut result_file = File::create("result.json").expect("Failed to create result.json");
    result_file.write_all(serialized_result.as_bytes()).unwrap();
}

fn dot_product_enc(v: &Vec<Enc>, w: &Vec<Enc>) -> Enc {
    let length = v.len();
    // We expect both vectors to have the same number of elements
    assert_eq!(length, w.len());
    assert!(length > 0);

    let mut sum = v[0] * w[0];

    for index in 1..length {
        sum = sum + v[index] * w[index];
    }
    sum
}
