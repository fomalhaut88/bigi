extern crate rand;

use std::mem;
use rand::Rng;
use crate::{bigi, base::{Bigi, BigiType, BIGI_MAX_DIGITS, BIGI_TYPE_BITS}};


pub fn quick_prime_check(x: &Bigi) -> bool {
    if x.is_even() {
        return false;
    }
    let primes: &[BigiType] = &[3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41,
                                43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
                                97, 101, 103, 107, 109, 113, 127, 131, 137,
                                139, 149, 151, 157, 163, 167, 173, 179, 181,
                                191, 193, 197, 199, 211, 223, 227, 229, 233,
                                239, 241, 251, 257, 263, 269, 271, 277, 281,
                                283, 293, 307, 311, 313, 317, 331, 337, 347,
                                349, 353, 359, 367, 373, 379, 383, 389, 397,
                                401, 409, 419, 421, 431, 433, 439, 443, 449,
                                457, 461, 463, 467, 479, 487, 491, 499, 503,
                                509, 521, 523, 541];

    for p in primes.iter() {
        let b = bigi![*p];
        if (*x % &b).is_zero() {
            return *x == b;
        }
    }

    return true;
}


pub fn miller_rabin(x: &Bigi, k: usize) -> bool {
    /*
    Miller–Rabin primality test: https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test
    */
    let bits = x.order * BIGI_TYPE_BITS;
    let mut rng = rand::thread_rng();
    let n = *x - &bigi![1];

    // Calculating d and s such that: x = 2^s * d + 1
    let mut d = n.clone();
    let mut s: usize = 0;
    while d.is_even() {
        d >>= 1;
        s += 1;
    }

    // Loop
    for _i in 0..k {
        let mut a = Bigi::gen_random(&mut rng, bits, false);
        a.divide(&x);

        if a.is_zero() {
            continue;
        }

        let mut b = a.powmod(&d, &x);

        if b != bigi![1] {
            let found = {
                let mut found = false;
                for _r in 0..s {
                    if b == n {
                        found = true;
                        break;
                    }
                    b = b.powmod(&bigi![2], &x);
                }
                found
            };

            if !found {
                return false;
            }
        }
    }

    return true;
}


pub fn gen_prime<R: Rng + ?Sized>(rng: &mut R, bits: usize) -> Bigi {
    loop {
        let x = Bigi::gen_random(rng, bits, true);
        if !quick_prime_check(&x) {
            continue;
        }
        let is_prime = miller_rabin(&x, 100);
        if is_prime {
            return x;
        }
    }
}


pub fn euclidean(x: &Bigi, y: &Bigi) -> Bigi {
    let mut a = x.clone();
    let mut b = y.clone();
    while b != bigi![0] {
        a.divide(&b);
        mem::swap(&mut a, &mut b);
    }
    a
}


pub fn euclidean_extended(x: &Bigi, y: &Bigi) -> (Bigi, Bigi, Bigi) {
    let mut a = x.clone();
    let mut b = y.clone();

    let mut aa = bigi![1];
    let mut ab = bigi![0];
    let mut ba = bigi![0];
    let mut bb = bigi![1];

    while b != bigi![0] {
        let q = a.divide(&b);

        aa = aa - &(q * &ba);
        ab = ab - &(q * &bb);

        mem::swap(&mut a, &mut b);
        mem::swap(&mut aa, &mut ba);
        mem::swap(&mut ab, &mut bb);
    }

    ab = bigi![0] - &ab;

    if aa > *y {
        aa = *y + &aa;
    }
    if ab > *x {
        ab = *x + &ab;
    }

    (a, aa, ab)
}


pub fn add_mod(x: &Bigi, y: &Bigi, m: &Bigi) -> Bigi {
    (*x + y) % m
}


pub fn sub_mod(x: &Bigi, y: &Bigi, m: &Bigi) -> Bigi {
    if x >= y {
        *x - y
    } else {
        *m - y + x
    }
}


pub fn mul_mod(x: &Bigi, y: &Bigi, m: &Bigi) -> Bigi {
    (*x * y) % m
}


pub fn inv_mod(x: &Bigi, m: &Bigi) -> Bigi {
    x.powmod(&(*m - &bigi![2]), &m)
}


pub fn div_mod(x: &Bigi, y: &Bigi, m: &Bigi) -> Bigi {
    mul_mod(x, &inv_mod(y, m), m)
}


pub fn sqrt_mod(n: &Bigi, p: &Bigi) -> Result<(Bigi, Bigi), &'static str> {
    /*
    Tonelli–Shanks algorithm: https://en.wikipedia.org/wiki/Tonelli%E2%80%93Shanks_algorithm
    */
    // If n is not a quadratic residue
    if legendre_symbol(&n, &p) != 1 {
        return Err("Non-quadratic residue");
    }

    // Defining q and s such that p - 1 = q * 2^s
    let (q, s) = {
        let mut q = *p - &bigi![1];
        let mut s: usize = 0;
        while q.is_even() {
            q >>= 1;
            s += 1;
        }
        (q, s)
    };

    // Searching for a non-quadratic residue
    let z: Bigi = {
        let mut z = bigi![2];
        loop {
            if legendre_symbol(&z, &p) != 1 {
                break;
            }
            z += &bigi![1];
        }
        z
    };

    let mut c = z.powmod(&q, &p);
    let mut r = n.powmod(&((q + &bigi![1]) >> 1), &p);
    let mut t = n.powmod(&q, &p);
    let mut m = s;

    // Tonelli–Shanks's loop
    while t != bigi![1] {
        let i = {
            let mut tp = t.clone();
            let mut i: usize = 0;
            while tp != bigi![1] {
                tp = mul_mod(&tp, &tp, &p);
                i += 1;
            }
            i
        };
        let b = c.powmod(&(bigi![1] << (m - i - 1)), &p);
        r = mul_mod(&r, &b, &p);
        c = mul_mod(&b, &b, &p);
        t = mul_mod(&t, &c, &p);
        m = i;
    }

    // Second root
    let mut rc = *p - &r;
    if rc < r {
        mem::swap(&mut r, &mut rc);
    }

    Ok((r, rc))
}


pub fn legendre_symbol(a: &Bigi, p: &Bigi) -> BigiType {
    BigiType::from(a.powmod(&((*p - &bigi![1]) >> 1), p))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_miller_rabin() {
        assert_eq!(miller_rabin(&bigi![29], 100), true);
        assert_eq!(miller_rabin(&bigi![1009], 100), true);
        assert_eq!(miller_rabin(&bigi![1001], 100), false);
    }

    #[test]
    fn test_euclidean() {
        assert_eq!(euclidean(&bigi![5], &bigi![7]), bigi![1]);
        assert_eq!(euclidean(&bigi![15], &bigi![6]), bigi![3]);
        assert_eq!(euclidean(&bigi![15], &bigi![8]), bigi![1]);
        assert_eq!(euclidean(&bigi![34000000], &bigi![17800]), bigi![200]);
    }

    #[test]
    fn test_euclidean_extended() {
        assert_eq!(euclidean_extended(&bigi![5], &bigi![7]), (bigi![1], bigi![3], bigi![2]));
        assert_eq!(euclidean_extended(&bigi![8], &bigi![15]), (bigi![1], bigi![2], bigi![1]));
        assert_eq!(euclidean_extended(&bigi![15], &bigi![8]), (bigi![1], bigi![7], bigi![13]));
        assert_eq!(euclidean_extended(&bigi![34000000], &bigi![17800]), (bigi![200], bigi![9], bigi![17191]));
    }

    #[test]
    fn test_sqrt_mod() {
        assert_eq!(sqrt_mod(&bigi![10], &bigi![13]), Ok((bigi![6], bigi![7])));
        assert_eq!(sqrt_mod(&bigi![5], &bigi![29]), Ok((bigi![11], bigi![18])));
        assert_eq!(sqrt_mod(&bigi![8], &bigi![29]), Err("Non-quadratic residue"));
        assert_eq!(sqrt_mod(&bigi![75], &bigi![97]), Ok((bigi![47], bigi![50])));
    }

    #[test]
    fn test_gen_prime() {
        let mut rng = rand::thread_rng();

        assert_eq!(gen_prime(&mut rng, 128).bit_length(), 128);
        assert_eq!(gen_prime(&mut rng, 65).bit_length(), 65);
        assert_eq!(gen_prime(&mut rng, 96).bit_length(), 96);
        assert_eq!(gen_prime(&mut rng, 33).bit_length(), 33);
        assert_eq!(gen_prime(&mut rng, 15).bit_length(), 15);
        assert_eq!(gen_prime(&mut rng, 3).bit_length(), 3);
    }
}
