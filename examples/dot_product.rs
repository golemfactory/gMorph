use gmorph::*;
use alga::general::{ClosedAdd,ClosedMul};

fn encrypt_vec(key_pair: &KeyPair, v: &Vec<u32>) -> Vec<Enc> {
    v.into_iter().map(|x| Enc::encrypt(&key_pair, *x)).collect()
}

fn decrypt_vec(key_pair: &KeyPair, v: &Vec<Enc>) -> Vec<u32> {
    v.into_iter().map(|x| x.decrypt(&key_pair)).collect()
}

fn ring_dot_product<T>(v: &Vec<T>, w: &Vec<T>) -> T
  where T: ClosedAdd + ClosedMul + Copy
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

fn dot_product_u32(v: &Vec<u32>, w: &Vec<u32>) -> u32
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


fn main() {
    let key_pair = KeyPair::new();
    let plain: Vec<u32> = (1..10).collect();
    let enc: Vec<_> = encrypt_vec(&key_pair, &plain);
    let given = ring_dot_product(&enc,&enc).decrypt(&key_pair);
    let expected: u32 = dot_product_u32(&plain, &plain);

    assert_eq!(expected, given, "the sums should be equal, and equal");
    println!("{}", given)
}
