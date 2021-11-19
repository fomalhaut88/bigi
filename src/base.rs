//! This module implements basics for Bigi.

use std::cmp;


/// Type for multiprecision integers.
#[derive(Debug, Clone, Copy)]
pub struct Bigi<const N: usize> {
    pub digits: [u64; N]
}


impl<const N: usize> Bigi<N> {
    /// Creates a zero ingeter.
    /// ```rust
    /// use bigi::Bigi;
    ///
    /// let z = Bigi::<8>::new();
    /// ```
    pub fn new() -> Self {
        let digits: [u64; N] = [0; N];
        Self { digits }
    }

    /// Creates an integer with digits given as a vector of *u64*.
    /// ```rust
    /// use bigi::Bigi;
    ///
    /// let z = Bigi::<8>::from_vec(&vec![2, 4, 0, 11]);
    /// ```
    pub fn from_vec(v: &Vec<u64>) -> Self {
        let mut res = Self::new();
        let size = cmp::min(N, v.len());
        res.digits[..size].clone_from_slice(&v[..size]);
        res
    }

    /// Converts an integer to a vector of *u64*.
    /// ```rust
    /// use bigi::Bigi;
    ///
    /// let z = Bigi::<8>::new();
    /// assert_eq!(z.to_vec(), [0, 0, 0, 0, 0, 0, 0, 0]);
    /// ```
    pub fn to_vec(&self) -> Vec<u64> {
        self.digits.to_vec()
    }
}


/// A macros to create an integer by listing its *u64* digits.
/// ```rust
/// use bigi::{bigi, Bigi};
///
/// let z = bigi![8; 2, 4, 0, 11];
/// ```
#[macro_export]
macro_rules! bigi {
    ($n:expr; $($x:expr),*) => [{
        let mut digits: [u64; $n] = [0; $n];
        let mut idx: usize = 0;

        $(
            #[allow(unused_assignments)]
            {
                digits[idx] = $x;
                idx += 1;
            }
        )*
        Bigi { digits }
    }]
}


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_macro_bigi() {
        let a = bigi![8; 2, 4, 0, 11, 5, 87, 1, 111];
        assert_eq!(a.to_vec(), vec![2, 4, 0, 11, 5, 87, 1, 111]);
    }

    #[bench]
    fn bench_macro_bigi(bencher: &mut Bencher) {
        bencher.iter(|| bigi![8; 2, 4, 0, 11, 5, 87, 1, 111]);
    }

    #[bench]
    fn bench_from_vec(bencher: &mut Bencher) {
        let v: Vec<u64> = vec![2, 4, 0, 11, 5, 87, 1, 111];
        bencher.iter(|| Bigi::<8>::from_vec(&v));
    }
}
