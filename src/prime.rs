extern crate rand;

use std::mem;
use rand::Rng;
use crate::{bigi, base::{Bigi, BigiType, BIGI_MAX_DIGITS}};


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


pub fn ferma_test(x: &Bigi, k: usize) -> bool {
    let bits = x.bit_length();
    let mut rng = rand::thread_rng();
    let p = *x - &bigi![1];

    for _i in 0..k {
        let a = Bigi::gen_random(&mut rng, bits, false) % &x;

        if a.is_zero() {
            continue;
        }

        if a.powmod(&p, &x) != bigi![1] {
            return false;
        }
    }

    return true;
}


pub fn miller_rabin(x: &Bigi, k: usize) -> bool {
    /*
    Miller–Rabin primality test: https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test
    */
    let bits = x.bit_length();
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
        let a = Bigi::gen_random(&mut rng, bits, false) % &x;

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
    while !b.is_zero() {
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

    while !b.is_zero() {
        let q = a.divide(&b);

        aa -= &(q * &ba);
        ab -= &(q * &bb);

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
    let mut r = *x + y;
    while r >= *m {
        r -= m;
    }
    r
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
    euclidean_extended(&x, &m).1
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

    let mut r;
    if p.mod_2k(2) == bigi![3] {
        // Case p = 3 (mod 4)
        r = n.powmod(&((*p + &bigi![1]) >> 2), &p);

    } else {
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
        r = n.powmod(&((q + &bigi![1]) >> 1), &p);
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
    }

    // Second root
    let mut rc = *p - &r;
    if rc < r {
        mem::swap(&mut r, &mut rc);
    }

    Ok((r, rc))
}


pub fn legendre_symbol(a: &Bigi, p: &Bigi) -> i32 {
    /*
    The algorithm was taken from  "Algorithmic Number Theory"
    by Bach and Shallit (page 113).
    The alternative (a^((p - 1) / 2) mod p):
        BigiType::from(a.powmod(&((*p - &bigi![1]) >> 1), p)) as i32
    */

    let mut t: i32 = 1;
    let mut ac = a.clone();
    let mut pc = p.clone();

    while !ac.is_zero() {
        let r = BigiType::from(pc.mod_2k(3));
        let i = (r == 3) || (r == 5);
        while ac.is_even() {
            ac >>= 1;
            if i {
                t = -t;
            }
        }
        mem::swap(&mut ac, &mut pc);
        if (r % 4 == 3) && (BigiType::from(pc.mod_2k(2)) == 3) {
            t = -t;
        }
        ac %= &pc;
    }

    t
}


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_miller_rabin() {
        assert_eq!(miller_rabin(&bigi![29], 100), true);
        assert_eq!(miller_rabin(&bigi![1009], 100), true);
        assert_eq!(miller_rabin(&bigi![1001], 100), false);
    }

    #[test]
    fn test_ferma_test() {
        assert_eq!(ferma_test(&bigi![29], 100), true);
        assert_eq!(ferma_test(&bigi![1009], 100), true);
        assert_eq!(ferma_test(&bigi![1001], 100), false);
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

    #[bench]
    fn bench_gen_prime_32(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        b.iter(|| gen_prime(&mut rng, 32));
    }

    #[bench]
    fn bench_gen_prime_64(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        b.iter(|| gen_prime(&mut rng, 64));
    }

    #[bench]
    fn bench_gen_prime_128(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        b.iter(|| gen_prime(&mut rng, 128));
    }

    #[bench]
    fn bench_gen_prime_256(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        b.iter(|| gen_prime(&mut rng, 256));
    }

    #[bench]
    fn bench_quick_prime_check_256(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        b.iter(|| {
            let x = Bigi::gen_random(&mut rng, 256, false);
            quick_prime_check(&x);
        });
    }

    #[bench]
    fn bench_ferma_test_256(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        b.iter(|| {
            let x = Bigi::gen_random(&mut rng, 256, false);
            ferma_test(&x, 1);
        });
    }

    #[bench]
    fn bench_miller_rabin_256(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        b.iter(|| {
            let x = Bigi::gen_random(&mut rng, 256, false);
            miller_rabin(&x, 1);
        });
    }

    #[bench]
    fn bench_euclidean_256(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        b.iter(|| {
            let x = Bigi::gen_random(&mut rng, 256, false);
            let y = Bigi::gen_random(&mut rng, 256, false);
            euclidean(&x, &y);
        });
    }

    #[bench]
    fn bench_euclidean_extended_256(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        b.iter(|| {
            let x = Bigi::gen_random(&mut rng, 256, false);
            let y = Bigi::gen_random(&mut rng, 256, false);
            euclidean_extended(&x, &y);
        });
    }

    #[bench]
    fn bench_add_mod_256(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let m = gen_prime(&mut rng, 256);
        let x = gen_prime(&mut rng, 256) % &m;
        let y = gen_prime(&mut rng, 256) % &m;
        b.iter(|| add_mod(&x, &y, &m));
    }

    #[bench]
    fn bench_sub_mod_256(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let m = gen_prime(&mut rng, 256);
        let x = gen_prime(&mut rng, 256) % &m;
        let y = gen_prime(&mut rng, 256) % &m;
        b.iter(|| sub_mod(&x, &y, &m));
    }

    #[bench]
    fn bench_mul_mod_256(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let m = gen_prime(&mut rng, 256);
        let x = gen_prime(&mut rng, 256) % &m;
        let y = gen_prime(&mut rng, 256) % &m;
        b.iter(|| mul_mod(&x, &y, &m));
    }

    #[bench]
    fn bench_div_mod_256(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let m = gen_prime(&mut rng, 256);
        let x = gen_prime(&mut rng, 256) % &m;
        let y = gen_prime(&mut rng, 256) % &m;
        b.iter(|| div_mod(&x, &y, &m));
    }

    #[bench]
    fn bench_inv_mod_256(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let m = gen_prime(&mut rng, 256);
        let x = gen_prime(&mut rng, 256) % &m;
        b.iter(|| inv_mod(&x, &m));
    }

    #[bench]
    fn bench_legendre_symbol_256(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let p = gen_prime(&mut rng, 256);
        b.iter(|| {
            let x = Bigi::gen_random(&mut rng, 256, false);
            legendre_symbol(&x, &p);
        });
    }

    #[bench]
    fn bench_sqrt_mod_256(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let p = gen_prime(&mut rng, 256);
        b.iter(|| {
            let x = Bigi::gen_random(&mut rng, 256, false);
            let _ = sqrt_mod(&x, &p);
        });
    }
}
