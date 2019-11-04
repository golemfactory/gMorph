#![feature(test)]

extern crate test;
use gmorph::*;
use test::Bencher;

#[bench]
fn bench_mul(b: &mut Bencher) {
    let key_pair = KeyPair::new();
    let eone = Enc::encrypt(&key_pair, 1);

    b.iter(|| {
        test::black_box((1..1000).fold(eone, |old, new| old * eone));
    })
}
