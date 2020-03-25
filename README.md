<div align="center">
  <h1><code>gmorph</code></h1>

  <p>
    <strong>Fully homomorphic encryption library for gWasm.</strong>
  </p>

  <p>
    <a href="https://github.com/golemfactory/gmorph/actions"><img src="https://github.com/golemfactory/gmorph/workflows/Continuous%20Integration/badge.svg" /></a>
    <a href="https://crates.io/crates/gmorph"><img src="https://img.shields.io/crates/v/gmorph.svg?style=flat-square" alt="Crates.io version" /></a>
    <a href="https://crates.io/crates/gmorph"><img src="https://img.shields.io/crates/d/gmorph.svg?style=flat-square" alt="Download" /></a>
    <a href="https://docs.rs/gmorph/"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>
  </p>
</div>

`gMorph` is written entirely in Rust and is meant to be easily cross-compiled to WebAssembly for use in [gWasm].

[gWasm]: https://docs.golem.network/#/Products/Brass-Beta/gWASM
[here]: https://github.com/golemfactory/gmorph/issues

## Disclaimer

`gMorph` is very much experimental in nature so things are expected
to break unexpectedly. Also, please note that we make no claims about security of the encryption scheme.
This work is provided as the Proof of Concept for FHE on gWASM, basically for demonstration purposes.
If you find a bug, please file a bug report [here].

```toml
# Cargo.toml
[dependencies]
gmorph = "0.1"
```

## Example usage:

```rust
use gmorph::*;
use num_traits::Zero;

let key_pair = KeyPair::default();
let enc: Vec<_> = (1..10)
    .map(|x| Encoded::encode(x).encrypt(&key_pair))
    .collect();
let enc = enc.into_iter().fold(Encoded::zero(), |acc, x| acc + x);
let given = enc.decrypt(&key_pair).decode();
let expected: u32 = (1..10).sum();

assert_eq!(expected, given, "the sums should be equal, and equal to 45");
```

## Examples
You can find some more examples in [examples](examples) folder.
For instance, to run `examples/simple_mul.rs`, invoke:

```
cargo run --release --example simple_mul
```

