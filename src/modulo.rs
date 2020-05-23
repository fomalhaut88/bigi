use crate::base::{Bigi};
use crate::prime::{euclidean_extended, sqrt_mod};


pub struct Modulo {
    pub modulo: Bigi,
}


impl Modulo {
    pub fn new(m: &Bigi) -> Self {
        Self { modulo: *m }
    }

    pub fn normalize(&self, x: &mut Bigi) {
        x.divide(&self.modulo);
    }

    pub fn add(&self, x: &Bigi, y: &Bigi) -> Bigi {
        let mut r = *x + y;
        while r >= self.modulo {
            r -= &self.modulo;
        }
        r
    }

    pub fn sub(&self, x: &Bigi, y: &Bigi) -> Bigi {
        if x >= y {
            *x - y
        } else {
            self.modulo - y + x
        }
    }

    pub fn mul(&self, x: &Bigi, y: &Bigi) -> Bigi {
        (*x * y) % &self.modulo
    }

    pub fn div(&self, x: &Bigi, y: &Bigi) -> Bigi {
        self.mul(x, &self.inv(y))
    }

    pub fn inv(&self, x: &Bigi) -> Bigi {
        euclidean_extended(&x, &self.modulo).1
    }

    pub fn pow(&self, x: &Bigi, k: &Bigi) -> Bigi {
        x.powmod(k, &self.modulo)
    }

    pub fn sqrt(&self, x: &Bigi) -> Result<(Bigi, Bigi), &'static str> {
        sqrt_mod(x, &self.modulo)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{bigi, BIGI_MAX_DIGITS};

    #[test]
    fn test_normalize() {
        let m = Modulo::new(&bigi![19]);
        let mut x: Bigi;

        x = bigi![25];
        m.normalize(&mut x);
        assert_eq!(x, bigi![6]);

        x = bigi![0];
        m.normalize(&mut x);
        assert_eq!(x, bigi![0]);

        x = bigi![11];
        m.normalize(&mut x);
        assert_eq!(x, bigi![11]);

        x = bigi![1];
        m.normalize(&mut x);
        assert_eq!(x, bigi![1]);

        x = bigi![192];
        m.normalize(&mut x);
        assert_eq!(x, bigi![2]);

        x = bigi![38];
        m.normalize(&mut x);
        assert_eq!(x, bigi![0]);
    }

    #[test]
    fn test_add() {
        let m = Modulo::new(&bigi![19]);
        assert_eq!(m.add(&bigi![3], &bigi![7]), bigi![10]);
        assert_eq!(m.add(&bigi![13], &bigi![10]), bigi![4]);
        assert_eq!(m.add(&bigi![13], &bigi![6]), bigi![0]);
        assert_eq!(m.add(&bigi![13], &bigi![0]), bigi![13]);
        assert_eq!(m.add(&bigi![0], &bigi![6]), bigi![6]);
        assert_eq!(m.add(&bigi![0], &bigi![0]), bigi![0]);
    }

    #[test]
    fn test_sub() {
        let m = Modulo::new(&bigi![19]);
        assert_eq!(m.sub(&bigi![3], &bigi![7]), bigi![15]);
        assert_eq!(m.sub(&bigi![13], &bigi![10]), bigi![3]);
        assert_eq!(m.sub(&bigi![13], &bigi![0]), bigi![13]);
        assert_eq!(m.sub(&bigi![0], &bigi![6]), bigi![13]);
        assert_eq!(m.sub(&bigi![0], &bigi![0]), bigi![0]);
    }

    #[test]
    fn test_mul() {
        let m = Modulo::new(&bigi![19]);
        assert_eq!(m.mul(&bigi![3], &bigi![4]), bigi![12]);
        assert_eq!(m.mul(&bigi![13], &bigi![10]), bigi![16]);
        assert_eq!(m.mul(&bigi![13], &bigi![0]), bigi![0]);
        assert_eq!(m.mul(&bigi![0], &bigi![6]), bigi![0]);
        assert_eq!(m.mul(&bigi![0], &bigi![0]), bigi![0]);
    }

    #[test]
    fn test_div() {
        let m = Modulo::new(&bigi![19]);
        assert_eq!(m.div(&bigi![12], &bigi![3]), bigi![4]);
        assert_eq!(m.div(&bigi![4], &bigi![13]), bigi![12]);
        assert_eq!(m.div(&bigi![0], &bigi![6]), bigi![0]);
    }

    #[test]
    fn test_inv() {
        let m = Modulo::new(&bigi![19]);
        assert_eq!(m.inv(&bigi![3]), bigi![13]);
        assert_eq!(m.inv(&bigi![13]), bigi![3]);
        assert_eq!(m.inv(&bigi![1]), bigi![1]);
    }

    #[test]
    fn test_pow() {
        let m = Modulo::new(&bigi![19]);
        assert_eq!(m.pow(&bigi![2], &bigi![4]), bigi![16]);
        assert_eq!(m.pow(&bigi![3], &bigi![5]), bigi![15]);
        assert_eq!(m.pow(&bigi![1], &bigi![16]), bigi![1]);
        assert_eq!(m.pow(&bigi![0], &bigi![6]), bigi![0]);
    }

    #[test]
    fn test_sqrt_mod() {
        let m = Modulo::new(&bigi![19]);
        assert_eq!(m.sqrt(&bigi![2]), Err("Non-quadratic residue"));
        assert_eq!(m.sqrt(&bigi![5]), Ok((bigi![9], bigi![10])));
        assert_eq!(m.sqrt(&bigi![16]), Ok((bigi![4], bigi![15])));
        assert_eq!(m.sqrt(&bigi![1]), Ok((bigi![1], bigi![18])));
    }
}
