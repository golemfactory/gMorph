use gmorph::*;

use gwasm_api::dispatcher;
use gwasm_api::{Blob, Output, TaskResult};
use gwasm_api::{SplitContext};

use std::fs::File;
use std::io::prelude::*;

fn split(context : &mut SplitContext) -> Vec<(Vec<Enc>, Vec<Enc>)> {
    let mut data_file = std::fs::File::open("data.json")
        .expect("Failed to open data.json");
    let mut serialized = String::new();
    data_file.read_to_string(&mut serialized).unwrap();
    let data : (Vec<Enc>, Vec<Enc>) = serde_json::from_str(&serialized).unwrap();
    vec![data]
}

fn execute(x: Vec<Enc>, y: Vec<Enc>) -> (Enc, Enc) {
    let xy = dot_product_enc(&x, &y);
    let xx = dot_product_enc(&x, &x);
    let serialized_result = serde_json::to_string(&(xy, xx)).unwrap();
    (xy, xx)
}

// both :: (a->b) -> (a,a) -> (b,b)
// both f (a,b) = (f a, f b)
fn both<A,B>(f: impl Fn(A) -> B, pair: (A,A) ) -> (B,B) {
  (f(pair.0), f(pair.1))
}

fn merge(args : &Vec<String>, results: Vec<( (Vec<Enc>, Vec<Enc>), (Enc, Enc))>) {
    let mut keys_file = std::fs::File::open("keys.json").unwrap();
    let mut serialized_keypair = String::new();
    keys_file.read_to_string(&mut serialized_keypair).unwrap();
    let key_pair: KeyPair = serde_json::from_str(&serialized_keypair).unwrap();

    let (a,b)
    	= results.into_iter()
	  .map(|p| both(|x| x.decrypt(&key_pair), p.1))
 	  .fold((0,0), |acc, x| (acc.0 + x.0, acc.1 + x.1));

    let m = a as f64 / b as f64;
    println!("m = {}", m);
}


fn main() {
   dispatcher::run(&split, &execute, &merge).unwrap();
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
