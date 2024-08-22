//! A reimplementation of Unity's PRNG in Rust, which is based on the Xorshift128 algorithm.
//!
//!
//! Results should be near 1-to-1 with Unity,
//! with the exception of `color()` which may produce
//! slightly inaccurate results due to an issue with .NET versions before 5.x.
//! 
//! Unlike Unity, it does not offer a static class.
//!
//! This project is not affiliated with Unity Technologies.
mod crypto;
mod random;
pub use crate::random::{Random, State};
