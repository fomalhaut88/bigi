# bigi

Bigi is a free library written in pure Rust for multi precision arithmetic
over unsigned integers. It includes efficient algorithms to perform
the standard arithmetic operations, modular arithmetic, some algorithms
for prime numbers (Miller-Rabin primality test, Fermat primality test,
Euclidean algorithm, Tonelli–Shanks algorithm),
Montgomery modular multiplication. Mostly Bigi is designed for cryptography
issues, but it also can be applied anywhere else. The library is developed
for Rust Nightly strictly.

For high performance bigi uses static data allocation for integers.

Test:

    cargo test

Benchmark:

    cargo bench

## Installation

Add this line to the dependencies in your Cargo.toml:

    ...
    [dependencies]
    bigi = { git = "https://github.com/fomalhaut88/bigi.git", tag = "v1.0.0" }

## Use cases

#### Basic example

```rust
use bigi::Bigi;

let a = Bigi::<4>::from(25);
let b = Bigi::<4>::from(40);
let c = a * &b;
println!("{}", c.to_decimal());  // 1000
```

In the type `Bigi::<4>`, `4` is the number of digits in the array of `u64` for
the inner integer representation, so that in `Bigi::<4>` you can store integers
from `0` to `115792089237316195423570985008687907853269984665640564039457584007913129639935`
(that is `2` power `256 = 64 * 4` minus `1`).

#### Format

```rust
use bigi::Bigi;

let a = Bigi::<4>::from_decimal("9238475695037419187591267512");
println!("{:?}", a.to_decimal());  // "9238475695037419187591267512"
println!("{:?}", a.to_hex());  // "0x1DD9E352F361677CE18544B8"
println!("{:?}", a.to_bytes());  // [184, 68, 133, 225, 124, 103, 97, 243, 82, 227, 217, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
```

#### Random

Generating 256-bits random unsigned integer:

```rust
use bigi::Bigi;

let mut rng = rand::thread_rng();
let a = Bigi::<4>::gen_random(&mut rng, 256, false);
```

#### Modular arithmetic

Modulo must be prime.

```rust
use bigi::{Bigi, Modulo};

let m = Modulo::new(&Bigi::<4>::from(43));

println!("{:?}", m.add(&Bigi::<4>::from(25), &Bigi::<4>::from(40)));  // 22
println!("{:?}", m.sub(&Bigi::<4>::from(25), &Bigi::<4>::from(40)));  // 28
println!("{:?}", m.mul(&Bigi::<4>::from(25), &Bigi::<4>::from(40)));  // 11
println!("{:?}", m.div(&Bigi::<4>::from(25), &Bigi::<4>::from(40)));  // 6
println!("{:?}", m.pow(&Bigi::<4>::from(25), &Bigi::<4>::from(40)));  // 15
println!("{:?}", m.inv(&Bigi::<4>::from(40)));  // 14
println!("{:?}", m.sqrt(&Bigi::<4>::from(40)));  // Ok((13, 30))
```

#### Prime numbers

Prime number generation and Miller-Rabin primality test example:

```rust
use bigi::prime::{gen_prime, miller_rabin};

let mut rng = rand::thread_rng();
let x = gen_prime::<8>(&mut rng, 256);  // 256-bits prime number
let is_prime = miller_rabin(&x, 100);  // true
```

#### Euclidean algorithm

```rust
use bigi::prime::{euclidean, euclidean_extended};

// GCD calculation
let gcd = euclidean(&Bigi::<4>::from(15), &Bigi::<4>::from(9));  // 3

// GCD with coefficients
let (gcd, c1, c2) = euclidean_extended(&Bigi::<4>::from(15), &Bigi::<4>::from(9));  // (3, 8, 13): 3 = 8 * 15 - 13 * 9
```

#### Montgomery modular multiplication

```rust
use bigi::montgomery::MontgomeryAlg;

// Montgomery arithmetic for modulo 23 and r = 2^5 = 32
let mgr = MontgomeryAlg::new(5, &Bigi::<4>::from(23));

// Multiplication example
let a = Bigi::<4>::from(6);
let b = Bigi::<4>::from(2);

let a_repr = mgr.to_repr(&a);  // 8
let b_repr = mgr.to_repr(&b);  // 18

let c_repr = mgr.mul(&a_repr, &b_repr);  // 16

let c = mgr.from_repr(&c_repr);  // 12 that is a * b (mod 23)

// Power example
let d = mgr.powmod(&a, &b);  // 13 = a^b (mod 23)
```
