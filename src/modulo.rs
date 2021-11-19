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
        let mut r = *x + y;
        while r >= self.modulo {
            r -= &self.modulo;
        }
        r
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
        (*x * y) % &self.modulo
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
        let m = Modulo::new(&bigi![8; 19]);
        let mut x: Bigi<8>;

        x = bigi![8; 25];
        m.normalize(&mut x);
        assert_eq!(x, bigi![8; 6]);

        x = bigi![8; 0];
        m.normalize(&mut x);
        assert_eq!(x, bigi![8; 0]);

        x = bigi![8; 11];
        m.normalize(&mut x);
        assert_eq!(x, bigi![8; 11]);

        x = bigi![8; 1];
        m.normalize(&mut x);
        assert_eq!(x, bigi![8; 1]);

        x = bigi![8; 192];
        m.normalize(&mut x);
        assert_eq!(x, bigi![8; 2]);

        x = bigi![8; 38];
        m.normalize(&mut x);
        assert_eq!(x, bigi![8; 0]);
    }

    #[test]
    fn test_add() {
        let m = Modulo::new(&bigi![8; 19]);
        assert_eq!(m.add(&bigi![8; 3], &bigi![8; 7]), bigi![8; 10]);
        assert_eq!(m.add(&bigi![8; 13], &bigi![8; 10]), bigi![8; 4]);
        assert_eq!(m.add(&bigi![8; 13], &bigi![8; 6]), bigi![8; 0]);
        assert_eq!(m.add(&bigi![8; 13], &bigi![8; 0]), bigi![8; 13]);
        assert_eq!(m.add(&bigi![8; 0], &bigi![8; 6]), bigi![8; 6]);
        assert_eq!(m.add(&bigi![8; 0], &bigi![8; 0]), bigi![8; 0]);
    }

    #[test]
    fn test_sub() {
        let m = Modulo::new(&bigi![8; 19]);
        assert_eq!(m.sub(&bigi![8; 3], &bigi![8; 7]), bigi![8; 15]);
        assert_eq!(m.sub(&bigi![8; 13], &bigi![8; 10]), bigi![8; 3]);
        assert_eq!(m.sub(&bigi![8; 13], &bigi![8; 0]), bigi![8; 13]);
        assert_eq!(m.sub(&bigi![8; 0], &bigi![8; 6]), bigi![8; 13]);
        assert_eq!(m.sub(&bigi![8; 0], &bigi![8; 0]), bigi![8; 0]);
    }

    #[test]
    fn test_mul() {
        let m = Modulo::new(&bigi![8; 19]);
        assert_eq!(m.mul(&bigi![8; 3], &bigi![8; 4]), bigi![8; 12]);
        assert_eq!(m.mul(&bigi![8; 13], &bigi![8; 10]), bigi![8; 16]);
        assert_eq!(m.mul(&bigi![8; 13], &bigi![8; 0]), bigi![8; 0]);
        assert_eq!(m.mul(&bigi![8; 0], &bigi![8; 6]), bigi![8; 0]);
        assert_eq!(m.mul(&bigi![8; 0], &bigi![8; 0]), bigi![8; 0]);
    }

    #[test]
    fn test_div() {
        let m = Modulo::new(&bigi![8; 19]);
        assert_eq!(m.div(&bigi![8; 12], &bigi![8; 3]), bigi![8; 4]);
        assert_eq!(m.div(&bigi![8; 4], &bigi![8; 13]), bigi![8; 12]);
        assert_eq!(m.div(&bigi![8; 0], &bigi![8; 6]), bigi![8; 0]);
    }

    #[test]
    fn test_inv() {
        let m = Modulo::new(&bigi![8; 19]);
        assert_eq!(m.inv(&bigi![8; 3]), bigi![8; 13]);
        assert_eq!(m.inv(&bigi![8; 13]), bigi![8; 3]);
        assert_eq!(m.inv(&bigi![8; 1]), bigi![8; 1]);
    }

    #[test]
    fn test_pow() {
        let m = Modulo::new(&bigi![8; 19]);
        assert_eq!(m.pow(&bigi![8; 2], &bigi![8; 4]), bigi![8; 16]);
        assert_eq!(m.pow(&bigi![8; 3], &bigi![8; 5]), bigi![8; 15]);
        assert_eq!(m.pow(&bigi![8; 1], &bigi![8; 16]), bigi![8; 1]);
        assert_eq!(m.pow(&bigi![8; 0], &bigi![8; 6]), bigi![8; 0]);
    }

    #[test]
    fn test_sqrt_mod() {
        let m = Modulo::new(&bigi![8; 19]);
        assert_eq!(m.sqrt(&bigi![8; 2]), Err("Non-quadratic residue"));
        assert_eq!(m.sqrt(&bigi![8; 5]), Ok((bigi![8; 9], bigi![8; 10])));
        assert_eq!(m.sqrt(&bigi![8; 16]), Ok((bigi![8; 4], bigi![8; 15])));
        assert_eq!(m.sqrt(&bigi![8; 1]), Ok((bigi![8; 1], bigi![8; 18])));
    }
}
