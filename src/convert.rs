//! This module implements converting between `Bigi` with different sizes and
//! with `u64`.
//!
//! ```rust
//! use bigi::Bigi;
//!
//! let a = Bigi::<4>::from(25);
//! assert_eq!(Bigi::<8>::from(&a), Bigi::<8>::from(25));
//! assert_eq!(u64::from(&a), 25);
//! ```

use std::{convert, cmp};
use crate::base::Bigi;


impl<const N: usize> convert::From<u64> for Bigi<N> {
    fn from(z: u64) -> Self {
        let mut res = Self::new();
        res.digits[0] = z;
        res
    }
}


impl<const N: usize> convert::From<&Bigi<N>> for u64 {
    fn from(a: &Bigi<N>) -> u64 {
        a.digits[0]
    }
}


impl<const N: usize, const M: usize> convert::From<&Bigi<M>> for Bigi<N> {
    fn from(a: &Bigi<M>) -> Self {
        let mut res = Bigi::<N>::new();
        let size = cmp::min(N, M);
        res.digits[..size].clone_from_slice(&a.digits[..size]);
        res
    }
}


#[cfg(test)]
mod tests {
    use crate::bigi;
    use super::*;
    use test::Bencher;

    #[test]
    fn test_from_u64() {
        assert_eq!(Bigi::<8>::from(1000000000000), bigi![8; 1000000000000]);
    }

    #[test]
    fn test_to_u64() {
        assert_eq!(u64::from(&bigi![8; 1000000000000]), 1000000000000);
    }

    #[test]
    fn test_from_bigi() {
        assert_eq!(
            Bigi::<8>::from(&bigi![4; 2, 4, 0, 11]),
            bigi![8; 2, 4, 0, 11, 0, 0, 0, 0]
        );
        assert_eq!(
            Bigi::<4>::from(&bigi![8; 2, 4, 0, 11, 5, 87, 1, 111]),
            bigi![4; 2, 4, 0, 11]
        );
    }

    #[bench]
    fn bench_from_u64(bencher: &mut Bencher) {
        bencher.iter(|| Bigi::<8>::from(1000000000000));
    }

    #[bench]
    fn bench_to_u64(bencher: &mut Bencher) {
        let a = bigi![8; 1000000000000];
        bencher.iter(|| u64::from(&a));
    }

    #[bench]
    fn bench_from_bigi(bencher: &mut Bencher) {
        let a = bigi![4; 2, 4, 0, 11];
        bencher.iter(|| Bigi::<8>::from(&a));
    }
}
