extern crate rand;

use rand::Rng;
use rand::distributions::{Distribution, Standard};
use crate::{bigi, base::{Bigi, BigiType, BIGI_MAX_DIGITS, BIGI_TYPE_BITS}};


impl Distribution<Bigi> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Bigi {
        let mut res = bigi![0];
        for i in 0..BIGI_MAX_DIGITS {
            res.digits[i] = rng.gen();
        }
        res.update_order();
        res
    }
}


impl Bigi {
    pub fn gen_random<R: Rng + ?Sized>(rng: &mut R, bits: usize, strict: bool) -> Bigi {
        let mut res = bigi![0];

        let quotient = bits / BIGI_TYPE_BITS;
        let remainder = bits % BIGI_TYPE_BITS;

        for i in 0..quotient {
            res.digits[i] = rng.gen();
        }

        if remainder > 0 {
            res.digits[quotient] = rng.gen::<BigiType>() % (1 << remainder);
            if strict {
                res.digits[quotient] |= 1 << (remainder - 1);
            }
        } else {
            if strict {
                res.digits[quotient - 1] |= 1 << (BIGI_TYPE_BITS - 1);
            }
        }

        res.update_order();

        res
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rand_sample() {
        let mut rng = rand::thread_rng();
        let x = rng.gen::<Bigi>();
        assert!(x.digits[0] > 0);
        assert!(x.digits[BIGI_MAX_DIGITS - 1] > 0);
    }

    #[test]
    fn gen_random() {
        let mut rng = rand::thread_rng();
        let x = Bigi::gen_random(&mut rng, 128, true);
        assert!(x.digits[0] > 0);
        assert!(x.digits[3] > 0);
        assert!(x.digits[4] == 0);
    }
}
