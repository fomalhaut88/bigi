//! Bigi is a free library written in pure Rust for multi precision arithmetic
//! over unsigned integers. It includes efficient algorithms to perform
//! the standard arithmetic operations, modular arithmetic, some algorithms
//! for prime numbers (Miller-Rabin primality test, Fermat primality test,
//! Euclidean algorithm, Tonelli–Shanks algorithm),
//! Montgomery modular multiplication. Mostly Bigi is designed for cryptography
//! issues, but it also can be applied anywhere else. The library is developed
//! for Rust Nightly strictly.
//!
//! To achieve high performance static data allocation for the numbers is used,
//! the number type is implemented as a generic structure with a fixed-size
//! array of *u64*.

#![feature(test)]
extern crate test;

pub mod base;
pub mod convert;
pub mod format;
pub mod random;
pub mod operations;
pub mod prime;
pub mod modulo;
pub mod montgomery;

pub use base::*;
pub use convert::*;
pub use format::*;
pub use random::*;
pub use operations::*;
pub use prime::*;
pub use modulo::*;
pub use montgomery::*;
