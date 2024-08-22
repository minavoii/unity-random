# Unity-Random

[<img alt="github" src="https://img.shields.io/badge/github-minavoii/unity--random-8da0cb?labelColor=555555&logo=github" height="20">](https://github.com/minavoii/unity-random) [<img alt="crates.io" src="https://img.shields.io/crates/v/unity-random.svg?color=fc8d62&logo=rust" height="20">](https://crates.io/crates/unity-random) [<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-unity--random-66c2a5?labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/unity-random)

A full reimplementation of Unity's PRNG in Rust, which is based on the Xorshift128 algorithm.

Results should be near 1-to-1 with Unity, with the exception of `color()` which may produce slightly inaccurate results due to an issue with .NET versions before 5.x.

Unlike Unity, it does not offer a static class.

Note: you should not use this if you expect a cryptographically secure PRNG. If you just want to generate random numbers, you should be looking into the [`Rand`](https://crates.io/crates/rand) crate.

This project is not affiliated with Unity Technologies.

## Usage

```rust
use unity_random::Random;
let mut random = Random::new();
random.init_state(220824); // set the PRNG seed

let integer = random.range_int(0, 100);
let float = random.range_float(0., 1.);
let rotation = random.rotation_uniform();
```

You can also save/load the current state:

```rust
use unity_random::Random;
let mut random = Random::new();
random.init_state(220824);

// You can save the current state...
let saved_state = random.state;

// then generate random numbers...
let i1 = random.range_int(0, 100);

// and load the state again...
random.state = saved_state;

// ... to generate the same sequence
let i2 = random.range_int(0, 100);
assert_eq!(i1, i2);
```

## Feature flags

- `serde`: Enables serialization and deserialization of the PRNG's `State`.
