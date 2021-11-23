//! This module implements
//! [Montgomery arithmetics](https://en.wikipedia.org/wiki/Montgomery_modular_multiplication).
//!
//! Multiplication example:
//! ```rust
//! use bigi::{Bigi, MontgomeryAlg};
//!
//! let n = Bigi::<8>::from(23);
//! let mgr = MontgomeryAlg::new(5, &n);
//!
//! let a = Bigi::<8>::from(6);
//! let b = Bigi::<8>::from(2);
//!
//! let ai = mgr.to_repr(&a);
//! let bi = mgr.to_repr(&b);
//!
//! let ci = mgr.mul(&ai, &bi);
//!
//! let c = mgr.from_repr(&ci);
//!
//! assert_eq!(c, Bigi::<8>::from(12));  // 12 = 6 * 2
//! ```
//!
//! Exponentiation example:
//! ```rust
//! use bigi::{Bigi, MontgomeryAlg};
//!
//! let n = Bigi::<8>::from(23);
//! let mgr = MontgomeryAlg::new(5, &n);
//!
//! let a = Bigi::<8>::from(3);
//! let k = Bigi::<8>::from(4);
//!
//! let ai = mgr.to_repr(&a);
//!
//! let ci = mgr.powmod(&ai, &k);
//!
//! let c = mgr.from_repr(&ci);
//!
//! assert_eq!(c, Bigi::<8>::from(12));  // 12 = 3**4 % 23
//! ```

use crate::base::Bigi;
use crate::prime::euclidean_extended;


pub struct MontgomeryAlg<const N: usize> {
    k: usize,
    n: Bigi<N>,
    ni: Bigi<N>
}


impl<const N: usize> MontgomeryAlg<N> {
    /// Creates a Montgomery arithmetics algoruthm instance.
    pub fn new(k: usize, n: &Bigi<N>) -> Self {
        assert!(k >= n.bit_length());
        let ni = euclidean_extended(&(Bigi::<N>::from(1) << k), n).2;
        Self { k: k, n: *n, ni: ni }
    }

    /// Converts integer to its Montgomery image.
    pub fn to_repr(&self, a: &Bigi<N>) -> Bigi<N> {
        (*a << self.k) % &self.n
    }

    /// Converts Montgomery image to its original integer.
    pub fn from_repr(&self, a: &Bigi<N>) -> Bigi<N> {
        self.mul(a, &Bigi::<N>::from(1))
    }

    /// Montgomery multiplication over the images.
    pub fn mul(&self, a: &Bigi<N>, b: &Bigi<N>) -> Bigi<N> {
        let t = *a * b;
        if t.is_zero() {
            return Bigi::<N>::from(0);
        }
        let mut res = (
            ((t.mod_2k(self.k) * &self.ni).mod_2k(self.k) * &self.n) >> self.k
        ) + &(t >> self.k) + &Bigi::<N>::from(1);
        while res >= self.n {
            res -= &self.n;
        }
        res
    }

    /// Montgomery exponentiation over the images.
    pub fn powmod(&self, a: &Bigi<N>, p: &Bigi<N>) -> Bigi<N> {
        let mut res = self.to_repr(&Bigi::<N>::from(1));
        let mut a2 = a.clone();
        for bit in 0..p.bit_length() {
            if p.get_bit(bit) {
                res = self.mul(&res, &a2);
            }
            a2 = self.mul(&a2, &a2);
        }
        res
    }
}


#[cfg(test)]
mod tests {
    use crate::bigi;
    use super::*;
    use crate::prime::gen_prime;
    use test::Bencher;

    #[test]
    fn test_to_repr() {
        let n = bigi![4; 23];
        let mgr = MontgomeryAlg::new(5, &n);

        assert_eq!(mgr.to_repr(&bigi![4; 6]), bigi![4; 8]);
        assert_eq!(mgr.to_repr(&bigi![4; 1]), bigi![4; 9]);
        assert_eq!(mgr.to_repr(&bigi![4; 2]), bigi![4; 18]);
        assert_eq!(mgr.to_repr(&bigi![4; 12]), bigi![4; 16]);
        assert_eq!(mgr.to_repr(&bigi![4; 0]), bigi![4; 0]);
        assert_eq!(mgr.to_repr(&bigi![4; 22]), bigi![4; 14]);
    }

    #[test]
    fn test_from_repr() {
        let n = bigi![4; 23];
        let mgr = MontgomeryAlg::new(5, &n);

        assert_eq!(mgr.from_repr(&bigi![4; 8]), bigi![4; 6]);
        assert_eq!(mgr.from_repr(&bigi![4; 9]), bigi![4; 1]);
        assert_eq!(mgr.from_repr(&bigi![4; 18]), bigi![4; 2]);
        assert_eq!(mgr.from_repr(&bigi![4; 16]), bigi![4; 12]);
        assert_eq!(mgr.from_repr(&bigi![4; 0]), bigi![4; 0]);
        assert_eq!(mgr.from_repr(&bigi![4; 14]), bigi![4; 22]);
    }

    #[test]
    fn test_mul() {
        let n = bigi![4; 23];
        let mgr = MontgomeryAlg::new(5, &n);

        assert_eq!(mgr.mul(&bigi![4; 8], &bigi![4; 9]), bigi![4; 8]);
        assert_eq!(mgr.mul(&bigi![4; 8], &bigi![4; 18]), bigi![4; 16]);
        assert_eq!(mgr.mul(&bigi![4; 9], &bigi![4; 9]), bigi![4; 9]);
    }

    #[test]
    fn test_powmod() {
        let n = bigi![4; 23];
        let mgr = MontgomeryAlg::new(5, &n);

        assert_eq!(mgr.powmod(&bigi![4; 9], &bigi![4; 12]), bigi![4; 9]);
    }

    #[bench]
    fn bench_to_repr_256(bencher: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let n = gen_prime::<_, 8>(&mut rng, 256);
        let x = Bigi::gen_random(&mut rng, 256, false) % &n;
        let mgr = MontgomeryAlg::new(256, &n);
        bencher.iter(|| {
            mgr.to_repr(&x);
        });
    }

    #[bench]
    fn bench_from_repr_256(bencher: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let n = gen_prime::<_, 8>(&mut rng, 256);
        let x = Bigi::gen_random(&mut rng, 256, false) % &n;
        let mgr = MontgomeryAlg::new(256, &n);
        bencher.iter(|| {
            mgr.from_repr(&x);
        });
    }

    #[bench]
    fn bench_mul_256(bencher: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let n = gen_prime::<_, 8>(&mut rng, 256);
        let x = Bigi::gen_random(&mut rng, 256, false) % &n;
        let y = Bigi::gen_random(&mut rng, 256, false) % &n;
        let mgr = MontgomeryAlg::new(256, &n);
        bencher.iter(|| {
            mgr.mul(&x, &y);
        });
    }

    #[bench]
    fn bench_powmod_256(bencher: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let n = gen_prime::<_, 8>(&mut rng, 256);
        let x = Bigi::gen_random(&mut rng, 256, false) % &n;
        let y = Bigi::gen_random(&mut rng, 256, false) % &n;
        bencher.iter(|| {
            let mgr = MontgomeryAlg::new(256, &n);
            let xm = mgr.to_repr(&x);
            let zm = mgr.powmod(&xm, &y);
            let _ = mgr.from_repr(&zm);
        });
    }
}
