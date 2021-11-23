//! This modulo implements modular arithmetics as methods of the type **Modulo**.

use crate::base::Bigi;
use crate::prime::{euclidean_extended, sqrt_mod};


pub struct Modulo<const N: usize> {
    pub modulo: Bigi<N>,
}


impl<const N: usize> Modulo<N> {
    /// Creates a modulo instance from the given integer.
    pub fn new(m: &Bigi<N>) -> Self {
        Self { modulo: *m }
    }

    /// Transforms given `x` into its reminder of the division `x` by the modulo.
    pub fn normalize(&self, x: &mut Bigi<N>) {
        x.divide(&self.modulo);
    }

    /// Modular addition.
    pub fn add(&self, x: &Bigi<N>, y: &Bigi<N>) -> Bigi<N> {
        let yi = self.modulo - y;
        if *x < yi {
            *x + y
        } else {
            *x - &yi
        }
    }

    /// Modular subtraction.
    pub fn sub(&self, x: &Bigi<N>, y: &Bigi<N>) -> Bigi<N> {
        if x >= y {
            *x - y
        } else {
            self.modulo - y + x
        }
    }

    /// Modular multiplication.
    pub fn mul(&self, x: &Bigi<N>, y: &Bigi<N>) -> Bigi<N> {
        let mut pair = x.multiply_overflowing(y);
        pair.0.divide_overflowing(&self.modulo, &pair.1);
        pair.0
    }

    /// Modular division.
    pub fn div(&self, x: &Bigi<N>, y: &Bigi<N>) -> Bigi<N> {
        self.mul(x, &self.inv(y))
    }

    /// Modular inverse (using extended Euclidean algorithm).
    pub fn inv(&self, x: &Bigi<N>) -> Bigi<N> {
        euclidean_extended(&x, &self.modulo).1
    }

    /// Modular exponentiation.
    pub fn pow(&self, x: &Bigi<N>, k: &Bigi<N>) -> Bigi<N> {
        x.powmod(k, &self.modulo)
    }

    /// Modular square root (using Tonelli–Shanks algorithm).
    pub fn sqrt(&self, x: &Bigi<N>) -> Result<(Bigi<N>, Bigi<N>), &'static str> {
        sqrt_mod(x, &self.modulo)
    }
}


#[cfg(test)]
mod tests {
    use crate::bigi;
    use super::*;

    #[test]
    fn test_normalize() {
        let m = Modulo::new(&bigi![4; 19]);
        let mut x: Bigi<4>;

        x = bigi![4; 25];
        m.normalize(&mut x);
        assert_eq!(x, bigi![4; 6]);

        x = bigi![4; 0];
        m.normalize(&mut x);
        assert_eq!(x, bigi![4; 0]);

        x = bigi![4; 11];
        m.normalize(&mut x);
        assert_eq!(x, bigi![4; 11]);

        x = bigi![4; 1];
        m.normalize(&mut x);
        assert_eq!(x, bigi![4; 1]);

        x = bigi![4; 192];
        m.normalize(&mut x);
        assert_eq!(x, bigi![4; 2]);

        x = bigi![4; 38];
        m.normalize(&mut x);
        assert_eq!(x, bigi![4; 0]);
    }

    #[test]
    fn test_add() {
        let m = Modulo::new(&bigi![4; 19]);
        assert_eq!(m.add(&bigi![4; 3], &bigi![4; 7]), bigi![4; 10]);
        assert_eq!(m.add(&bigi![4; 13], &bigi![4; 10]), bigi![4; 4]);
        assert_eq!(m.add(&bigi![4; 13], &bigi![4; 6]), bigi![4; 0]);
        assert_eq!(m.add(&bigi![4; 13], &bigi![4; 0]), bigi![4; 13]);
        assert_eq!(m.add(&bigi![4; 0], &bigi![4; 6]), bigi![4; 6]);
        assert_eq!(m.add(&bigi![4; 0], &bigi![4; 0]), bigi![4; 0]);
    }

    #[test]
    fn test_sub() {
        let m = Modulo::new(&bigi![4; 19]);
        assert_eq!(m.sub(&bigi![4; 3], &bigi![4; 7]), bigi![4; 15]);
        assert_eq!(m.sub(&bigi![4; 13], &bigi![4; 10]), bigi![4; 3]);
        assert_eq!(m.sub(&bigi![4; 13], &bigi![4; 0]), bigi![4; 13]);
        assert_eq!(m.sub(&bigi![4; 0], &bigi![4; 6]), bigi![4; 13]);
        assert_eq!(m.sub(&bigi![4; 0], &bigi![4; 0]), bigi![4; 0]);
    }

    #[test]
    fn test_mul() {
        let m = Modulo::new(&bigi![4; 19]);
        assert_eq!(m.mul(&bigi![4; 3], &bigi![4; 4]), bigi![4; 12]);
        assert_eq!(m.mul(&bigi![4; 13], &bigi![4; 10]), bigi![4; 16]);
        assert_eq!(m.mul(&bigi![4; 13], &bigi![4; 0]), bigi![4; 0]);
        assert_eq!(m.mul(&bigi![4; 0], &bigi![4; 6]), bigi![4; 0]);
        assert_eq!(m.mul(&bigi![4; 0], &bigi![4; 0]), bigi![4; 0]);
    }

    #[test]
    fn test_div() {
        let m = Modulo::new(&bigi![4; 19]);
        assert_eq!(m.div(&bigi![4; 12], &bigi![4; 3]), bigi![4; 4]);
        assert_eq!(m.div(&bigi![4; 4], &bigi![4; 13]), bigi![4; 12]);
        assert_eq!(m.div(&bigi![4; 0], &bigi![4; 6]), bigi![4; 0]);
    }

    #[test]
    fn test_inv() {
        let m = Modulo::new(&bigi![4; 19]);
        assert_eq!(m.inv(&bigi![4; 3]), bigi![4; 13]);
        assert_eq!(m.inv(&bigi![4; 13]), bigi![4; 3]);
        assert_eq!(m.inv(&bigi![4; 1]), bigi![4; 1]);
    }

    #[test]
    fn test_pow() {
        let m = Modulo::new(&bigi![4; 19]);
        assert_eq!(m.pow(&bigi![4; 2], &bigi![4; 4]), bigi![4; 16]);
        assert_eq!(m.pow(&bigi![4; 3], &bigi![4; 5]), bigi![4; 15]);
        assert_eq!(m.pow(&bigi![4; 1], &bigi![4; 16]), bigi![4; 1]);
        assert_eq!(m.pow(&bigi![4; 0], &bigi![4; 6]), bigi![4; 0]);
    }

    #[test]
    fn test_sqrt_mod() {
        let m = Modulo::new(&bigi![4; 19]);
        assert_eq!(m.sqrt(&bigi![4; 2]), Err("Non-quadratic residue"));
        assert_eq!(m.sqrt(&bigi![4; 5]), Ok((bigi![4; 9], bigi![4; 10])));
        assert_eq!(m.sqrt(&bigi![4; 16]), Ok((bigi![4; 4], bigi![4; 15])));
        assert_eq!(m.sqrt(&bigi![4; 1]), Ok((bigi![4; 1], bigi![4; 18])));
    }
}
