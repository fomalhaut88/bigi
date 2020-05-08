use crate::{bigi, BIGI_MAX_DIGITS};
use crate::base::{Bigi};
use crate::prime::euclidean_extended;


pub struct MontgomeryAlg {
    k: usize,
    n: Bigi,
    ni: Bigi,
}


impl MontgomeryAlg {
    pub fn new(k: usize, n: &Bigi) -> Self {
        assert!(k >= n.bit_length());
        let ni = euclidean_extended(&(bigi![1]<< k), n).2;
        Self { k: k, n: *n, ni: ni}
    }

    pub fn to_repr(&self, a: &Bigi) -> Bigi {
        (*a << self.k) % &self.n
    }

    pub fn from_repr(&self, a: &Bigi) -> Bigi {
        self.mul(a, &bigi![1])
    }

    pub fn mul(&self, a: &Bigi, b: &Bigi) -> Bigi {
        let t = *a * b;
        if t.is_zero() {
            return bigi![0];
        }
        let mut res = (
            ((t.mod_2k(self.k) * &self.ni).mod_2k(self.k) * &self.n) >> self.k
        ) + &(t >> self.k) + &bigi![1];
        while res >= self.n {
            res -= &self.n;
        }
        res
    }

    pub fn powmod(&self, a: &Bigi, p: &Bigi) -> Bigi {
        let mut res = self.to_repr(&bigi![1]);
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
    use super::*;
    use crate::prime::gen_prime;
    use test::Bencher;

    #[test]
    fn test_to_repr() {
        let n = bigi![23];
        let mgr = MontgomeryAlg::new(5, &n);

        assert_eq!(mgr.to_repr(&bigi![6]), bigi![8]);
        assert_eq!(mgr.to_repr(&bigi![1]), bigi![9]);
        assert_eq!(mgr.to_repr(&bigi![2]), bigi![18]);
        assert_eq!(mgr.to_repr(&bigi![12]), bigi![16]);
        assert_eq!(mgr.to_repr(&bigi![0]), bigi![0]);
        assert_eq!(mgr.to_repr(&bigi![22]), bigi![14]);
    }

    #[test]
    fn test_from_repr() {
        let n = bigi![23];
        let mgr = MontgomeryAlg::new(5, &n);

        assert_eq!(mgr.from_repr(&bigi![8]), bigi![6]);
        assert_eq!(mgr.from_repr(&bigi![9]), bigi![1]);
        assert_eq!(mgr.from_repr(&bigi![18]), bigi![2]);
        assert_eq!(mgr.from_repr(&bigi![16]), bigi![12]);
        assert_eq!(mgr.from_repr(&bigi![0]), bigi![0]);
        assert_eq!(mgr.from_repr(&bigi![14]), bigi![22]);
    }

    #[test]
    fn test_mul() {
        let n = bigi![23];
        let mgr = MontgomeryAlg::new(5, &n);

        assert_eq!(mgr.mul(&bigi![8], &bigi![9]), bigi![8]);
        assert_eq!(mgr.mul(&bigi![8], &bigi![18]), bigi![16]);
        assert_eq!(mgr.mul(&bigi![9], &bigi![9]), bigi![9]);
    }

    #[test]
    fn test_powmod() {
        let n = bigi![23];
        let mgr = MontgomeryAlg::new(5, &n);

        assert_eq!(mgr.powmod(&bigi![9], &bigi![12]), bigi![9]);
    }

    #[bench]
    fn bench_powmod_256(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let n = gen_prime(&mut rng, 256);
        let x = Bigi::gen_random(&mut rng, 256, false) % &n;
        let y = Bigi::gen_random(&mut rng, 256, false) % &n;
        b.iter(|| {
            let mgr = MontgomeryAlg::new(256, &n);
            let xm = mgr.to_repr(&x);
            let zm = mgr.powmod(&xm, &y);
            let _ = mgr.from_repr(&zm);
        });
    }
}
