# bigi

Bigi is a free library written in pure Rust for multi precision arithmetic over unsigned integers. It includes efficient algorithms to perform the standard arithmetic operations, modular arithmetic, some algorithms for prime numbers (Miller-Rabin test, Ferma test, Euclidean algorithm, Tonelli–Shanks algorithm), Montgomery modular multiplication. Mostly Bigi is designed for cryptography tasks, but it also can be applied anywhere else. The library is developed for Rust Nightly strictly.

Bigi uses static data allocation for numbers, so it is necessary to build the library with specified BIGI_BITS environment variable - the size of integers in bits.

Test:

    BIGI_BITS=512 cargo test

Benchmark:

    BIGI_BITS=512 cargo bench

## Installation

Add this line to the dependencies in your Cargo.toml:

    ...
    [dependencies]
    bigi = { git = "https://github.com/fomalhaut88/bigi.git", tag = "v0.4.0" }

## Use cases

#### Basic example

```rust
extern crate bigi;
use bigi::{bigi, Bigi, BIGI_MAX_DIGITS};
...
let a = bigi![25];
let b = bigi![40];
let c = a * &b;
println!("{:?}", c);  // 1000
```

#### Format

```rust
let a = Bigi::from_decimal("9238475695037419187591267512");
println!("{:?}", a.to_decimal());  // "9238475695037419187591267512"
println!("{:?}", a.to_hex());  // "0x1DD9E352F361677CE18544B8"
println!("{:?}", a.to_bytes());  // [184, 68, 133, 225, 124, 103, 97, 243, 82, 227, 217, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
```

#### Random

Generating 256-bits random unsigned integer:

```rust
let mut rng = rand::thread_rng();
let a = Bigi::gen_random(&mut rng, 256, false);
```

#### Modular arithmetic

Modulo must be prime.

```rust
use bigi::Modulo;
...
let m = Modulo::new(&bigi![43]);

println!("{:?}", m.add(&bigi![25], &bigi![40]));  // 22
println!("{:?}", m.sub(&bigi![25], &bigi![40]));  // 28
println!("{:?}", m.mul(&bigi![25], &bigi![40]));  // 11
println!("{:?}", m.div(&bigi![25], &bigi![40]));  // 6
println!("{:?}", m.pow(&bigi![25], &bigi![40]));  // 15
println!("{:?}", m.inv(&bigi![40]));  // 14
println!("{:?}", m.sqrt(&bigi![40]));  // Ok((13, 30))
```

#### Prime numbers

```rust
use bigi::prime::{gen_prime, miller_rabin};
...
let mut rng = rand::thread_rng();
let x = gen_prime(&mut rng, 256);  // 256-bits prime number
let is_prime = miller_rabin(&x, 100);  // true
```

#### Euclidean algorithm

```rust
use bigi::prime::{euclidean, euclidean_extended};
...
// GCD calculation
let gcd = euclidean(&bigi![15], &bigi![9]);  // 3

// GCD with coefficients
let (gcd, c1, c2) = euclidean_extended(&bigi![15], &bigi![9]);  // (3, 8, 13): 3 = 8 * 15 - 13 * 9
```

#### Montgomery modular multiplication

```rust
use bigi::montgomery::MontgomeryAlg;
...
// Montgomery arithmetic for modulo 23 and r = 2^5 = 32
let mgr = MontgomeryAlg::new(5, &bigi![23]);

// Multiplication example
let a = bigi![6];
let b = bigi![2];

let a_repr = mgr.to_repr(&a);  // 8
let b_repr = mgr.to_repr(&b);  // 18

let c_repr = mgr.mul(&a_repr, &b_repr);  // 16

let c = mgr.from_repr(&c_repr);  // 12 that is a * b (mod 23)

// Power example
let d = mgr.powmod(&a, &b);  // 13 = a^b (mod 23)
```
