use rand::Rng;
use rand::distributions::{Distribution, Standard};
use crate::base::Bigi;


impl<const N: usize> Distribution<Bigi<N>> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Bigi<N> {
        let mut res = Bigi::<N>::new();
        for i in 0..N {
            res.digits[i] = rng.gen();
        }
        res
    }
}


impl<const N: usize> Bigi<N> {
    /// Generates a random integer from `0` to `(1 << bits) - 1` inclusively if
    /// `strict = false` and from `1 << (bits - 1)` to `(1 << bits) - 1` if
    /// `strict = true`.
    /// ```rust
    /// use bigi::Bigi;
    ///
    /// let mut rng = rand::thread_rng();
    /// let z = Bigi::<8>::gen_random(&mut rng, 256, true);
    /// ```
    pub fn gen_random<R: Rng + ?Sized>(rng: &mut R, bits: usize,
                                       strict: bool) -> Self {
        let mut res = Bigi::<N>::new();

        let quotient = bits >> 6;
        let remainder = bits & 63;

        for i in 0..quotient {
            res.digits[i] = rng.gen();
        }

        if remainder > 0 {
            let pw = 1 << remainder;
            res.digits[quotient] = rng.gen::<u64>() & (pw - 1);
            if strict {
                res.digits[quotient] |= pw >> 1;
            }
        } else {
            if strict && quotient > 0 {
                res.digits[quotient - 1] |= 1 << 63;
            }
        }

        res
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_rand_sample() {
        let mut rng = rand::thread_rng();
        let x = rng.gen::<Bigi<4>>();
        assert!(x.digits[0] > 0);
        assert!(x.digits[3] > 0);
    }

    #[bench]
    fn bench_gen_random_256(bencher: &mut Bencher) {
        let mut rng = rand::thread_rng();
        bencher.iter(|| Bigi::<8>::gen_random(&mut rng, 256, false));
    }
}
