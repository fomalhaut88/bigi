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
            if strict && quotient > 0 {
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
    fn test_rand_sample() {
        let mut rng = rand::thread_rng();
        let x = rng.gen::<Bigi>();
        assert!(x.digits[0] > 0);
        assert!(x.digits[BIGI_MAX_DIGITS - 1] > 0);
    }

    #[test]
    fn test_gen_random() {
        let mut rng = rand::thread_rng();

        assert_eq!(Bigi::gen_random(&mut rng, 128, true).bit_length(), 128);
        assert_eq!(Bigi::gen_random(&mut rng, 65, true).bit_length(), 65);
        assert_eq!(Bigi::gen_random(&mut rng, 96, true).bit_length(), 96);
        assert_eq!(Bigi::gen_random(&mut rng, 33, true).bit_length(), 33);
        assert_eq!(Bigi::gen_random(&mut rng, 15, true).bit_length(), 15);
        assert_eq!(Bigi::gen_random(&mut rng, 3, true).bit_length(), 3);
        assert_eq!(Bigi::gen_random(&mut rng, 1, true).bit_length(), 1);
        assert_eq!(Bigi::gen_random(&mut rng, 0, true).bit_length(), 0);
    }
}
