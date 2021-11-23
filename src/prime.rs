//! This module implements some algorithms for working with prime numbers.
//! It includes [Fermat primality test](https://en.wikipedia.org/wiki/Fermat_primality_test),
//! [Miller-Rabin test](https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test),
//! [Euclidean algorithm](https://en.wikipedia.org/wiki/Euclidean_algorithm),
//! [extended Euclidean algorithm](https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm),
//! [Legendre symbol](https://en.wikipedia.org/wiki/Legendre_symbol),
//! [Tonelli–Shanks algorithm](https://en.wikipedia.org/wiki/Tonelli%E2%80%93Shanks_algorithm),
//! functions for modular arithmetics and a function to generate a big random
//! prime number with fixed number of bits.

extern crate rand;

use std::mem;
use rand::Rng;
use crate::base::Bigi;


const QUICK_PRIMES: &[u64] = &[3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41,
                            43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
                            97, 101, 103, 107, 109, 113, 127, 131, 137,
                            139, 149, 151, 157, 163, 167, 173, 179, 181,
                            191, 193, 197, 199, 211, 223, 227, 229, 233];


/// Checks `x` for prime (except for `2`) that returns true if there is no
/// divisor among the fixed set of primes `QUICK_PRIMES` (from `3` to `233`).
/// ```rust
/// use bigi::{Bigi, quick_prime_check};
///
/// assert_eq!(quick_prime_check(&Bigi::<4>::from(11)), true);
/// assert_eq!(quick_prime_check(&Bigi::<4>::from(121)), false);
/// assert_eq!(quick_prime_check(&Bigi::<4>::from(541)), true);
/// assert_eq!(
///     quick_prime_check(&Bigi::<4>::from(282943)), true
/// );  // Though 282943 = 523 * 541
/// ```
pub fn quick_prime_check<const N: usize>(x: &Bigi<N>) -> bool {
    if x.is_even() {
        return false;
    }
    for p in QUICK_PRIMES.iter() {
        let b = Bigi::<N>::from(*p);
        if (*x % &b).is_zero() {
            return *x == b;
        }
    }
    return true;
}


/// Performs [Fermat primality test](https://en.wikipedia.org/wiki/Fermat_primality_test)
/// to check `x` for prime.
/// ```rust
/// use bigi::{Bigi, fermat_test};
///
/// assert_eq!(fermat_test(&Bigi::<4>::from(11), 10), true);
/// assert_eq!(fermat_test(&Bigi::<4>::from(121), 10), false);
/// assert_eq!(fermat_test(&Bigi::<4>::from(541), 10), true);
/// assert_eq!(fermat_test(&Bigi::<4>::from(282943), 10), false);
/// ```
pub fn fermat_test<const N: usize>(x: &Bigi<N>, k: usize) -> bool {
    let one = Bigi::<N>::from(1);
    let bits = x.bit_length();
    let mut rng = rand::thread_rng();
    let p = *x - &one;

    for _i in 0..k {
        let a = Bigi::<N>::gen_random(&mut rng, bits, false) % &x;

        if a.is_zero() {
            continue;
        }

        if a.powmod(&p, &x) != one {
            return false;
        }
    }

    true
}


/// Performs [Miller-Rabin test](https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test)
/// to check `x` for prime.
/// ```rust
/// use bigi::{Bigi, miller_rabin};
///
/// assert_eq!(miller_rabin(&Bigi::<4>::from(11), 10), true);
/// assert_eq!(miller_rabin(&Bigi::<4>::from(121), 10), false);
/// assert_eq!(miller_rabin(&Bigi::<4>::from(541), 10), true);
/// assert_eq!(miller_rabin(&Bigi::<4>::from(282943), 10), false);
/// ```
pub fn miller_rabin<const N: usize>(x: &Bigi<N>, k: usize) -> bool {
    let one = Bigi::<N>::from(1);
    let two = Bigi::<N>::from(2);
    let bits = x.bit_length();
    let mut rng = rand::thread_rng();
    let n = *x - &one;

    // Calculating d and s such that: x = 2^s * d + 1
    let mut d = n.clone();
    let mut s: usize = 0;
    while d.is_even() {
        d >>= 1;
        s += 1;
    }

    // Loop
    for _i in 0..k {
        let a = Bigi::<N>::gen_random(&mut rng, bits, false) % &x;

        if a.is_zero() {
            continue;
        }

        let mut b = a.powmod(&d, &x);

        if b != one {
            let found = {
                let mut found = false;
                for _r in 0..s {
                    if b == n {
                        found = true;
                        break;
                    }
                    b = b.powmod(&two, &x);
                }
                found
            };

            if !found {
                return false;
            }
        }
    }

    true
}


/// Generates a prime number sized exactly `bits` bits.
/// ```rust
/// use bigi::gen_prime;
///
/// let mut rng = rand::thread_rng();
/// let p = gen_prime::<_, 4>(&mut rng, 256);
/// ```
pub fn gen_prime<R: Rng + ?Sized, const N: usize>(
            rng: &mut R, bits: usize) -> Bigi<N> {
    loop {
        let x = Bigi::<N>::gen_random(rng, bits, true);
        if !quick_prime_check(&x) {
            continue;
        }
        let is_prime = miller_rabin(&x, 100);
        if is_prime {
            return x;
        }
    }
}


/// Calculates GCD using
/// [Euclidean algorithm](https://en.wikipedia.org/wiki/Euclidean_algorithm).
/// ```rust
/// use bigi::{Bigi, euclidean};
///
/// let a = Bigi::<4>::from(110);
/// let b = Bigi::<4>::from(88);
/// let c = euclidean(&a, &b);
/// assert_eq!(c, Bigi::<4>::from(22));
/// ```
pub fn euclidean<const N: usize>(x: &Bigi<N>, y: &Bigi<N>) -> Bigi<N> {
    let mut a = x.clone();
    let mut b = y.clone();
    while !b.is_zero() {
        a.divide(&b);
        mem::swap(&mut a, &mut b);
    }
    a
}


/// Applies
/// [extended Euclidean algorithm](https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm)
/// over two given numbers.
/// ```rust
/// use bigi::{Bigi, euclidean_extended};
///
/// let a = Bigi::<4>::from(110);
/// let b = Bigi::<4>::from(66);
/// let (c, ra, rb) = euclidean_extended(&a, &b);  // c == a * ra - b * rb
/// assert_eq!(c, Bigi::<4>::from(22));
/// assert_eq!(ra, Bigi::<4>::from(65));
/// assert_eq!(rb, Bigi::<4>::from(108));
/// ```
pub fn euclidean_extended<const N: usize> (
            x: &Bigi<N>, y: &Bigi<N>) -> (Bigi<N>, Bigi<N>, Bigi<N>) {
    let mut a = x.clone();
    let mut b = y.clone();

    let mut aa = Bigi::<N>::from(1);
    let mut ab = Bigi::<N>::from(0);
    let mut ba = Bigi::<N>::from(0);
    let mut bb = Bigi::<N>::from(1);
    let mut inv = false;

    while !b.is_zero() {
        let q = a.divide(&b);

        aa -= &(q * &ba);
        ab -= &(q * &bb);

        mem::swap(&mut a, &mut b);
        mem::swap(&mut aa, &mut ba);
        mem::swap(&mut ab, &mut bb);

        inv = !inv;
    }

    ab = Bigi::<N>::from(0) - &ab;

    if inv {
        aa += y;
        ab += x;
    }

    (a, aa, ab)
}


/// Performs modular addition: `(x + y) % m`.
pub fn add_mod<const N: usize>(
            x: &Bigi<N>, y: &Bigi<N>, m: &Bigi<N>) -> Bigi<N> {
    let yi = *m - y;
    if *x < yi {
        *x + y
    } else {
        *x - &yi
    }
}


/// Performs modular subtraction `(x - y) % m`.
pub fn sub_mod<const N: usize>(
            x: &Bigi<N>, y: &Bigi<N>, m: &Bigi<N>) -> Bigi<N> {
    if x >= y {
        *x - y
    } else {
        *m - y + x
    }
}


/// Performs modular multiplication `(x * y) % m`.
pub fn mul_mod<const N: usize>(
            x: &Bigi<N>, y: &Bigi<N>, m: &Bigi<N>) -> Bigi<N> {
    let mut pair = x.multiply_overflowing(y);
    pair.0.divide_overflowing(m, &pair.1);
    pair.0
}


/// Searches for `y` such that `(x * y) % m == 1`.
/// It is called modular inverse.
pub fn inv_mod<const N: usize>(
            x: &Bigi<N>, m: &Bigi<N>) -> Bigi<N> {
    euclidean_extended(&x, &m).1
}


/// Searches for `z` such that `(y * z) % m == x`.
/// It is called modular division.
pub fn div_mod<const N: usize>(
            x: &Bigi<N>, y: &Bigi<N>, m: &Bigi<N>) -> Bigi<N> {
    mul_mod(x, &inv_mod(y, m), m)
}


/// Calculates the
/// [Legendre symbol](https://en.wikipedia.org/wiki/Legendre_symbol)
/// of an integer `a` and prime `p`.
/// The algorithm was taken from  "Algorithmic Number Theory"
/// by Bach and Shallit (page 113).
/// ```rust
/// use bigi::{Bigi, legendre_symbol};
///
/// assert_eq!(legendre_symbol(&Bigi::<4>::from(6), &Bigi::<4>::from(137)), -1);
/// assert_eq!(legendre_symbol(&Bigi::<4>::from(8), &Bigi::<4>::from(137)), 1);
/// ```
pub fn legendre_symbol<const N: usize>(a: &Bigi<N>, p: &Bigi<N>) -> i32 {
    /*
    The algorithm was taken from  "Algorithmic Number Theory"
    by Bach and Shallit (page 113).
    The alternative implementation is (a^((p - 1) / 2) mod p):
        u64::from(&a.powmod(&((*p - &one) >> 1), p)) as i32
    */
    let mut t: i32 = 1;
    let mut ac = a.clone();
    let mut pc = p.clone();

    while !ac.is_zero() {
        let r = u64::from(&pc.mod_2k(3));
        let i = (r == 3) || (r == 5);
        while ac.is_even() {
            ac >>= 1;
            if i {
                t = -t;
            }
        }
        mem::swap(&mut ac, &mut pc);
        if (r % 4 == 3) && (u64::from(&pc.mod_2k(2)) == 3) {
            t = -t;
        }
        ac %= &pc;
    }

    t
}


/// Performs [Tonelli–Shanks algorithm](https://en.wikipedia.org/wiki/Tonelli%E2%80%93Shanks_algorithm)
/// that searches for `x` such that `(x * x) % p == n` where `p` is prime
/// (modular square root). The functions returns a tuple with two roots or error.
/// ```rust
/// use bigi::{Bigi, sqrt_mod};
///
/// assert_eq!(
///     sqrt_mod(&Bigi::<4>::from(8), &Bigi::<4>::from(137)),
///     Ok((Bigi::<4>::from(62), Bigi::<4>::from(75)))
/// );
/// assert_eq!(
///     sqrt_mod(&Bigi::<4>::from(6), &Bigi::<4>::from(137)),
///     Err("Non-quadratic residue")
/// );
/// ```
pub fn sqrt_mod<const N: usize>(n: &Bigi<N>, p: &Bigi<N>
            ) -> Result<(Bigi<N>, Bigi<N>), &'static str> {
    /*
    Tonelli–Shanks algorithm: https://en.wikipedia.org/wiki/Tonelli%E2%80%93Shanks_algorithm
    */
    // If n is not a quadratic residue
    if legendre_symbol(&n, &p) != 1 {
        return Err("Non-quadratic residue");
    }

    let one = Bigi::<N>::from(1);
    let mut r;
    if p.mod_2k(2) == Bigi::<N>::from(3) {
        // Case p = 3 (mod 4)
        r = n.powmod(&((*p + &one) >> 2), &p);

    } else {
        // Defining q and s such that p - 1 = q * 2^s
        let (q, s) = {
            let mut q = *p - &one;
            let mut s: usize = 0;
            while q.is_even() {
                q >>= 1;
                s += 1;
            }
            (q, s)
        };

        // Searching for a non-quadratic residue
        let z = {
            let mut z = Bigi::<N>::from(2);
            loop {
                if legendre_symbol(&z, &p) != 1 {
                    break;
                }
                z += &one;
            }
            z
        };

        let mut c = z.powmod(&q, &p);
        r = n.powmod(&((q + &one) >> 1), &p);
        let mut t = n.powmod(&q, &p);
        let mut m = s;

        // Tonelli–Shanks's loop
        while t != one {
            let i = {
                let mut tp = t.clone();
                let mut i: usize = 0;
                while tp != one {
                    tp = mul_mod(&tp, &tp, &p);
                    i += 1;
                }
                i
            };
            let b = c.powmod(&(one << (m - i - 1)), &p);
            r = mul_mod(&r, &b, &p);
            c = mul_mod(&b, &b, &p);
            t = mul_mod(&t, &c, &p);
            m = i;
        }
    }

    // Second root
    let mut rc = *p - &r;
    if rc < r {
        mem::swap(&mut r, &mut rc);
    }

    Ok((r, rc))
}


#[cfg(test)]
mod tests {
    use crate::bigi;
    use super::*;
    use test::Bencher;

    #[test]
    fn test_fermat_test() {
        assert_eq!(fermat_test(&bigi![8; 29], 100), true);
        assert_eq!(fermat_test(&bigi![8; 1009], 100), true);
        assert_eq!(fermat_test(&bigi![8; 1001], 100), false);
    }

    #[test]
    fn test_miller_rabin() {
        assert_eq!(miller_rabin(&bigi![8; 29], 100), true);
        assert_eq!(miller_rabin(&bigi![8; 1009], 100), true);
        assert_eq!(miller_rabin(&bigi![8; 1001], 100), false);
    }

    #[test]
    fn test_gen_prime() {
        let mut rng = rand::thread_rng();

        assert_eq!(gen_prime::<_, 4>(&mut rng, 128).bit_length(), 128);
        assert_eq!(gen_prime::<_, 4>(&mut rng, 65).bit_length(), 65);
        assert_eq!(gen_prime::<_, 4>(&mut rng, 96).bit_length(), 96);
        assert_eq!(gen_prime::<_, 4>(&mut rng, 33).bit_length(), 33);
        assert_eq!(gen_prime::<_, 4>(&mut rng, 15).bit_length(), 15);
        assert_eq!(gen_prime::<_, 4>(&mut rng, 3).bit_length(), 3);
    }

    #[test]
    fn test_sqrt_mod() {
        assert_eq!(sqrt_mod(&bigi![8; 10], &bigi![8; 13]), Ok((bigi![8; 6], bigi![8; 7])));
        assert_eq!(sqrt_mod(&bigi![8; 5], &bigi![8; 29]), Ok((bigi![8; 11], bigi![8; 18])));
        assert_eq!(sqrt_mod(&bigi![8; 8], &bigi![8; 29]), Err("Non-quadratic residue"));
        assert_eq!(sqrt_mod(&bigi![8; 75], &bigi![8; 97]), Ok((bigi![8; 47], bigi![8; 50])));
    }

    #[bench]
    fn bench_quick_prime_check_256(bencher: &mut Bencher) {
        let mut rng = rand::thread_rng();
        bencher.iter(|| {
            let x = Bigi::<4>::gen_random(&mut rng, 256, false);
            quick_prime_check(&x);
        });
    }

    #[bench]
    fn bench_fermat_test_256(bencher: &mut Bencher) {
        let mut rng = rand::thread_rng();
        bencher.iter(|| {
            let x = Bigi::<4>::gen_random(&mut rng, 256, false);
            fermat_test(&x, 1);
        });
    }

    #[bench]
    fn bench_miller_rabin_256(bencher: &mut Bencher) {
        let mut rng = rand::thread_rng();
        bencher.iter(|| {
            let x = Bigi::<4>::gen_random(&mut rng, 256, false);
            miller_rabin(&x, 1);
        });
    }

    #[bench]
    fn bench_gen_prime_32(bencher: &mut Bencher) {
        let mut rng = rand::thread_rng();
        bencher.iter(|| gen_prime::<_, 4>(&mut rng, 32));
    }

    #[bench]
    fn bench_gen_prime_64(bencher: &mut Bencher) {
        let mut rng = rand::thread_rng();
        bencher.iter(|| gen_prime::<_, 4>(&mut rng, 64));
    }

    #[bench]
    fn bench_gen_prime_128(bencher: &mut Bencher) {
        let mut rng = rand::thread_rng();
        bencher.iter(|| gen_prime::<_, 4>(&mut rng, 128));
    }

    #[bench]
    fn bench_gen_prime_256(bencher: &mut Bencher) {
        let mut rng = rand::thread_rng();
        bencher.iter(|| gen_prime::<_, 4>(&mut rng, 256));
    }

    #[bench]
    fn bench_euclidean_256(bencher: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let x = Bigi::<4>::gen_random(&mut rng, 256, false);
        let y = Bigi::<4>::gen_random(&mut rng, 256, false);
        bencher.iter(|| euclidean(&x, &y));
    }

    #[bench]
    fn bench_euclidean_extended_256(bencher: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let x = Bigi::<4>::gen_random(&mut rng, 256, false);
        let y = Bigi::<4>::gen_random(&mut rng, 256, false);
        bencher.iter(|| euclidean_extended(&x, &y));
    }

    #[bench]
    fn bench_add_mod_256(bencher: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let m = gen_prime::<_, 4>(&mut rng, 256);
        let x = gen_prime::<_, 4>(&mut rng, 256) % &m;
        let y = gen_prime::<_, 4>(&mut rng, 256) % &m;
        bencher.iter(|| add_mod(&x, &y, &m));
    }

    #[bench]
    fn bench_sub_mod_256(bencher: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let m = gen_prime::<_, 4>(&mut rng, 256);
        let x = gen_prime::<_, 4>(&mut rng, 256) % &m;
        let y = gen_prime::<_, 4>(&mut rng, 256) % &m;
        bencher.iter(|| sub_mod(&x, &y, &m));
    }

    #[bench]
    fn bench_mul_mod_256(bencher: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let m = gen_prime::<_, 4>(&mut rng, 256);
        let x = gen_prime::<_, 4>(&mut rng, 256) % &m;
        let y = gen_prime::<_, 4>(&mut rng, 256) % &m;
        bencher.iter(|| mul_mod(&x, &y, &m));
    }

    #[bench]
    fn bench_inv_mod_256(bencher: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let m = gen_prime::<_, 4>(&mut rng, 256);
        let x = gen_prime::<_, 4>(&mut rng, 256) % &m;
        bencher.iter(|| inv_mod(&x, &m));
    }

    #[bench]
    fn bench_div_mod_256(bencher: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let m = gen_prime::<_, 4>(&mut rng, 256);
        let x = gen_prime::<_, 4>(&mut rng, 256) % &m;
        let y = gen_prime::<_, 4>(&mut rng, 256) % &m;
        bencher.iter(|| div_mod(&x, &y, &m));
    }

    #[bench]
    fn bench_legendre_symbol_256(bencher: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let p = gen_prime::<_, 4>(&mut rng, 256);
        bencher.iter(|| {
            let x = Bigi::<4>::gen_random(&mut rng, 255, false);
            legendre_symbol(&x, &p);
        });
    }

    #[bench]
    fn bench_sqrt_mod_256(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let p = gen_prime::<_, 4>(&mut rng, 256);
        b.iter(|| {
            let x = Bigi::<4>::gen_random(&mut rng, 255, false);
            let _ = sqrt_mod(&x, &p);
        });
    }
}
