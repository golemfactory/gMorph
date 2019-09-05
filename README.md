



# gMorph  - Fully Homomorphic Encryption library
[![build-status]][build-link]

[build-status]: https://dev.azure.com/kubkon/gmorph/_apis/build/status/golemfactory.gmorph?branchName=master
[build-link]: https://dev.azure.com/kubkon/gmorph/_build?definitionId=4

`gMorph` is written entirely in Rust and is meant to be easily
cross-compiled to WebAssembly for use in [gWasm].

`gMorph` is very much experimental in nature so things are expected
to break unexpectedly. If you find a bug, please file a bug report [here].

[gWasm]: https://docs.golem.network/#/Products/Brass-Beta/gWASM
[here]: https://github.com/golemfactory/gmorph/issues

## Example usage:

```
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

