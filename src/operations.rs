//! This module implements basic arithmetic operations: addition, subtraction,
//! multiplication, division, modular exponentiation, comparison, shift right,
//! shift left and some other useful functions.

use std::{ops, cmp};
use crate::base::Bigi;


impl<const N: usize> Bigi<N> {
    /// Checks if the integer is zero.
    /// ```rust
    /// use bigi::{bigi, Bigi};
    ///
    /// let z = bigi![8; 0];
    /// assert_eq!(z.is_zero(), true);
    /// ```
    pub fn is_zero(&self) -> bool {
        for i in 0..N {
            if self.digits[i] > 0 {
                return false;
            }
        }
        true
    }

    /// Checks if the integer is odd.
    /// ```rust
    /// use bigi::{bigi, Bigi};
    ///
    /// let z = bigi![8; 17, 12];
    /// assert_eq!(z.is_odd(), true);
    /// ```
    pub fn is_odd(&self) -> bool {
        self.digits[0] & 1 == 1
    }

    /// Checks if the integer is even.
    /// ```rust
    /// use bigi::{bigi, Bigi};
    ///
    /// let z = bigi![8; 18, 12];
    /// assert_eq!(z.is_even(), true);
    /// ```
    pub fn is_even(&self) -> bool {
        self.digits[0] & 1 == 0
    }

    /// Gets the length of the integer in bits.
    /// ```rust
    /// use bigi::{bigi, Bigi};
    ///
    /// let z = bigi![8; 18, 12];
    /// assert_eq!(z.bit_length(), 68);
    /// ```
    pub fn bit_length(&self) -> usize {
        let idx = self.get_order();
        if idx > 0 {
            let val = self.digits[idx - 1];
            let mut r = 0;
            let mut s = 32;
            while s > 0 {
                let t = 1 << (r + s);
                if val >= t {
                    r += s;
                }
                s >>= 1;
            }
            ((idx - 1) << 6) + r + 1
        } else {
            0
        }
    }

    /// Gets certain bit of the integer.
    /// ```rust
    /// use bigi::{bigi, Bigi};
    ///
    /// let z = bigi![8; 18, 12];
    /// assert_eq!(z.get_bit(66), true);
    /// ```
    pub fn get_bit(&self, bit: usize) -> bool {
        let quot = bit >> 6;
        let rem = bit & 63;
        (self.digits[quot] & (1 << rem)) != 0
    }

    /// Gets `index + 1` where `index` is the idnex of the last non-zero digit.
    /// ```rust
    /// use bigi::{bigi, Bigi};
    ///
    /// let z = bigi![8; 18, 12];
    /// assert_eq!(z.get_order(), 2);
    /// ```
    pub fn get_order(&self) -> usize {
        let mut idx = N;
        while (idx > 0) && (self.digits[idx - 1] == 0) {
            idx -= 1;
        }
        idx
    }

    /// Performs division by given *divisor*. The funcion returns the quotient.
    /// This method changes the object so it equals to the reminder in the end.
    /// ```rust
    /// use bigi::{bigi, Bigi};
    ///
    /// let mut a = bigi![8; 14];
    /// let b = bigi![8; 4];
    /// let c = a.divide(&b);
    /// assert_eq!(a, bigi![8; 2]);
    /// assert_eq!(c, bigi![8; 3]);
    /// ```
    pub fn divide(&mut self, divisor: &Bigi<N>) -> Bigi<N> {
        let mut res = Bigi::<N>::new();

        let order1 = self.get_order();
        let order2 = divisor.get_order();

        if order1 >= order2 {
            let mut shf = order1 - order2;

            loop {
                let extra = if order2 + shf < N {
                    self.digits[order2 + shf]
                } else { 0u64 };

                // Skipping if the divider is less than shifted divisor
                if extra == 0 {
                    let is_less: bool = {
                        let mut result = false;
                        for i in (0..order2).rev() {
                            if self.digits[i + shf] > divisor.digits[i] {
                                result = false;
                                break;
                            }
                            if self.digits[i + shf] < divisor.digits[i] {
                                result = true;
                                break;
                            }
                        }
                        result
                    };

                    if is_less {
                        if shf != 0 {
                            shf -= 1;
                            continue;
                        } else {
                            break;
                        }
                    }
                }

                // Calculating factor
                let factor = {
                    let top = self.lead_u128();
                    let result;

                    let bottom = if extra > 0 {
                        divisor.digits[order2 - 1] as u128
                    } else {
                        if order2 == 1 && shf > 0 {
                            divisor.lead_u128() << 64
                        } else {
                            divisor.lead_u128()
                        }
                    };

                    if top == bottom {
                        result = 1;
                    } else {
                        result = top / (bottom + 1);
                    }

                    result as u64
                };

                // Adding factor to the result
                res.digits[shf] += factor;

                // Reducing dividend
                let mut fw: u128 = 0;
                for i in 0..order2 {
                    fw = (divisor.digits[i] as u128) * (factor as u128) + fw;
                    let pair = self.digits[i + shf].overflowing_sub(fw as u64);
                    self.digits[i + shf] = pair.0;
                    fw >>= 64;
                    if pair.1 {
                        fw += 1;
                    }
                }
                if fw > 0 && order2 + shf < N {
                    self.digits[order2 + shf] -= fw as u64;
                }
            }
        }

        res
    }

    /// Performs power `p` and modulo of the division by `m`.
    /// ```rust
    /// use bigi::{bigi, Bigi};
    ///
    /// let a = bigi![8; 3];
    /// let p = bigi![8; 4];
    /// let m = bigi![8; 7];
    /// let r = a.powmod(&p, &m);
    /// assert_eq!(r, bigi![8; 4]);
    /// ```
    pub fn powmod(&self, p: &Bigi<N>, m: &Bigi<N>) -> Bigi<N> {
        let mut res = Bigi::<N>::from(1);
        let mut x = self.clone();
        for bit in 0..p.bit_length() {
            if p.get_bit(bit) {
                res = res * &x;
                res.divide(&m);
            }
            x = x * &x;
            x.divide(&m);
        }
        res
    }

    /// Calculates the reminder of the division by 2 power `k`.
    /// ```rust
    /// use bigi::{bigi, Bigi};
    ///
    /// let a = bigi![8; 123];
    /// assert_eq!(a.mod_2k(5), bigi![8; 27]);
    /// ```
    pub fn mod_2k(&self, k: usize) -> Bigi<N> {
        let mut res = self.clone();
        let q = k >> 6;
        let r = k & 63;
        for i in (q + 1)..N {
            res.digits[i] = 0;
        }
        res.digits[q] &= (1 << r) - 1;
        res
    }

    fn lead_u128(&self) -> u128 {
        for i in (0..N).rev() {
            if self.digits[i] != 0 {
                if i == 0 {
                    return self.digits[i] as u128;
                }
                else {
                    return ((self.digits[i] as u128) << 64) +
                           (self.digits[i - 1] as u128);
                }
            }
        }
        0
    }
}


impl<const N: usize> ops::Add<&Bigi<N>> for Bigi<N> {
    type Output = Bigi<N>;

    fn add(self, other: &Bigi<N>) -> Bigi<N> {
        let mut res = self.clone();
        res += other;
        res
    }
}


impl<const N: usize> ops::AddAssign<&Bigi<N>> for Bigi<N> {
    fn add_assign(&mut self, other: &Bigi<N>) {
        let mut fw: u64 = 0;
        for i in 0..N {
            let pair = self.digits[i].overflowing_add(other.digits[i]);
            self.digits[i] = pair.0.overflowing_add(fw).0;
            fw = (pair.1 || (fw == 1 && self.digits[i] == 0)) as u64;
        }
    }
}


impl<const N: usize> ops::Sub<&Bigi<N>> for Bigi<N> {
    type Output = Bigi<N>;

    fn sub(self, other: &Bigi<N>) -> Bigi<N> {
        let mut res = self.clone();
        res -= other;
        res
    }
}


impl<const N: usize> ops::SubAssign<&Bigi<N>> for Bigi<N> {
    fn sub_assign(&mut self, other: &Bigi<N>) {
        let mut fw: u64 = 0;
        for i in 0..N {
            let pair = self.digits[i].overflowing_sub(other.digits[i]);
            self.digits[i] = pair.0.overflowing_sub(fw).0;
            fw = (pair.1 || (fw == 1 && pair.0 == 0)) as u64;
        }
    }
}


impl<const N: usize> ops::Mul<&Bigi<N>> for Bigi<N> {
    type Output = Bigi<N>;

    fn mul(self, other: &Bigi<N>) -> Bigi<N> {
        let mut res = Bigi::<N>::new();
        for i in 0..N {
            let mut fw: u128 = 0;
            for j in 0..(N - i) {
                fw = (other.digits[i] as u128) * (self.digits[j] as u128) +
                     (res.digits[i + j] as u128) + fw;
                res.digits[i + j] = fw as u64;
                fw >>= 64;
            }
        }
        res
    }
}


impl<const N: usize> ops::MulAssign<&Bigi<N>> for Bigi<N> {
    fn mul_assign(&mut self, other: &Bigi<N>) {
        *self = *self * other;
    }
}


impl<const N: usize> ops::Div<&Bigi<N>> for Bigi<N> {
    type Output = Bigi<N>;

    fn div(self, other: &Bigi<N>) -> Bigi<N> {
        let mut dividend = self.clone();
        dividend.divide(other)
    }
}


impl<const N: usize> ops::DivAssign<&Bigi<N>> for Bigi<N> {
    fn div_assign(&mut self, other: &Bigi<N>) {
        *self = *self / other;
    }
}


impl<const N: usize> ops::Rem<&Bigi<N>> for Bigi<N> {
    type Output = Bigi<N>;

    fn rem(self, other: &Bigi<N>) -> Bigi<N> {
        let mut res = self.clone();
        res %= other;
        res
    }
}


impl<const N: usize> ops::RemAssign<&Bigi<N>> for Bigi<N> {
    fn rem_assign(&mut self, other: &Bigi<N>) {
        self.divide(other);
    }
}


impl<const N: usize> ops::ShlAssign<usize> for Bigi<N> {
    fn shl_assign(&mut self, rhs: usize) {
        let rhs_q = rhs >> 6;
        let rhs_r = rhs & 63;
        let mut extra: u64 = 0;

        for i in (0..(N - rhs_q)).rev() {
            if rhs_r > 0 {
                extra = self.digits[i] >> (64 - rhs_r);
            }
            if i < N - rhs_q - 1 {
                self.digits[i + rhs_q + 1] += extra;
            }
            self.digits[i + rhs_q] = self.digits[i] << rhs_r;
        }

        for i in 0..rhs_q {
            self.digits[i] = 0;
        }
    }
}


impl<const N: usize> ops::Shl<usize> for Bigi<N> {
    type Output = Bigi<N>;

    fn shl(self, rhs: usize) -> Bigi<N> {
        let mut res = self.clone();
        res <<= rhs;
        res
    }
}


impl<const N: usize> ops::ShrAssign<usize> for Bigi<N> {
    fn shr_assign(&mut self, rhs: usize) {
        let rhs_q = rhs >> 6;
        let rhs_r = rhs & 63;
        let mut extra: u64 = 0;

        for i in rhs_q..N {
            if rhs_r > 0 {
                extra = self.digits[i] << (64 - rhs_r);
            }
            self.digits[i - rhs_q] = self.digits[i] >> rhs_r;
            if i > rhs_q {
                self.digits[i - rhs_q - 1] += extra;
            }
        }

        for i in 0..rhs_q {
            self.digits[N - i - 1] = 0;
        }
    }
}


impl<const N: usize> ops::Shr<usize> for Bigi<N> {
    type Output = Bigi<N>;

    fn shr(self, rhs: usize) -> Bigi<N> {
        let mut res = self.clone();
        res >>= rhs;
        res
    }
}


impl<const N: usize> cmp::PartialEq for Bigi<N> {
    fn eq(&self, other: &Self) -> bool {
        self.digits == other.digits
    }
}


impl<const N: usize> cmp::PartialOrd for Bigi<N> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        for i in (0..N).rev() {
            if self.digits[i] > other.digits[i] {
                return Some(cmp::Ordering::Greater);
            }
            if self.digits[i] < other.digits[i] {
                return Some(cmp::Ordering::Less);
            }
        }
        Some(cmp::Ordering::Equal)
    }
}


#[cfg(test)]
mod tests {
    use crate::bigi;
    use super::*;
    use test::Bencher;

    #[test]
    fn test_add() {
        assert_eq!(bigi![8; 2] + &bigi![8; 3], bigi![8; 5]);
        assert_eq!(
            bigi![8; 3567587328, 232, 0, 29] +
            &bigi![8; 12312344, 1, 1234098120, 21556, 134236576],
            bigi![8; 3579899672, 233, 1234098120, 21585, 134236576]
        );
    }

    #[test]
    fn test_sub() {
        assert_eq!(bigi![8; 5] - &bigi![8; 3], bigi![8; 2]);
        assert_eq!(
            bigi![8; 3579899672, 233, 1234098120, 21585, 134236576] -
            &bigi![8; 12312344, 1, 1234098120, 21556, 134236576],
            bigi![8; 3567587328, 232, 0, 29]
        );
    }

    #[test]
    fn test_mul() {
        assert_eq!(bigi![8; 5] * &bigi![8; 2], bigi![8; 10]);
        assert_eq!(
            bigi![8; 12312344, 1, 1234098120, 21556, 134236576] *
            &bigi![8; 3567587328, 232, 0, 29],
            bigi![8; 43925362432376832, 6424051136, 4402752814420623592,
                     77189580264184, 478900707496709949, 66931731112,
                     625124, 3892860704]
        );
    }

    #[test]
    fn test_divide() {
        let mut a = bigi![8; 43925362432376842, 6424051136,
                             4402752814420623592, 77189580264184,
                             478900707496709949, 66931731112,
                             625124, 3892860704];
        let b = bigi![8; 3567587328, 232, 0, 29];
        let c = a.divide(&b);
        assert_eq!(a, bigi![8; 10]);
        assert_eq!(c, bigi![8; 12312344, 1, 1234098120, 21556, 134236576]);
    }

    #[bench]
    fn bench_is_zero(bencher: &mut Bencher) {
        let x = bigi![8; 0];
        bencher.iter(|| x.is_zero());
    }

    #[bench]
    fn bench_get_bit(bencher: &mut Bencher) {
        let x = bigi![8; 3411848022234306463, 14482971280477013830,
                         16242343048349248772, 4571967601559393757];
        bencher.iter(|| x.get_bit(130));
    }

    #[bench]
    fn bench_bit_length(bencher: &mut Bencher) {
        let x = bigi![8; 3411848022234306463, 14482971280477013830,
                         16242343048349248772, 4571967601559393757];
        bencher.iter(|| x.bit_length());
    }

    #[bench]
    fn bench_get_order(bencher: &mut Bencher) {
        let x = bigi![8; 3411848022234306463, 14482971280477013830,
                         16242343048349248772, 4571967601559393757];
        bencher.iter(|| x.get_order());
    }

    #[bench]
    fn bench_add_256(bencher: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let x = Bigi::<8>::gen_random(&mut rng, 256, false);
        let y = Bigi::<8>::gen_random(&mut rng, 256, false);
        bencher.iter(|| x + &y);
    }

    #[bench]
    fn bench_sub_256(bencher: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let x = Bigi::<8>::gen_random(&mut rng, 256, false);
        let y = Bigi::<8>::gen_random(&mut rng, 256, false);
        bencher.iter(|| x - &y);
    }

    #[bench]
    fn bench_mul_256(bencher: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let x = Bigi::<8>::gen_random(&mut rng, 256, false);
        let y = Bigi::<8>::gen_random(&mut rng, 256, false);
        bencher.iter(|| x * &y);
    }

    #[bench]
    fn bench_divide_256_256(bencher: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let mut x = Bigi::<8>::gen_random(&mut rng, 256, false);
        let y = Bigi::<8>::gen_random(&mut rng, 256, false);
        bencher.iter(|| x.divide(&y));
    }

    #[bench]
    fn bench_divide_256_128(bencher: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let mut x = Bigi::<8>::gen_random(&mut rng, 256, false);
        let y = Bigi::<8>::gen_random(&mut rng, 128, false);
        bencher.iter(|| x.divide(&y));
    }

    #[bench]
    fn bench_div_256_256(bencher: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let x = Bigi::<8>::gen_random(&mut rng, 256, false);
        let y = Bigi::<8>::gen_random(&mut rng, 256, false);
        bencher.iter(|| x / &y);
    }

    #[bench]
    fn bench_div_256_128(bencher: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let x = Bigi::<8>::gen_random(&mut rng, 256, false);
        let y = Bigi::<8>::gen_random(&mut rng, 128, false);
        bencher.iter(|| x / &y);
    }

    #[bench]
    fn bench_mod_256_256(bencher: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let x = Bigi::<8>::gen_random(&mut rng, 256, false);
        let y = Bigi::<8>::gen_random(&mut rng, 256, false);
        bencher.iter(|| x % &y);
    }

    #[bench]
    fn bench_mod_256_128(bencher: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let x = Bigi::<8>::gen_random(&mut rng, 256, false);
        let y = Bigi::<8>::gen_random(&mut rng, 128, false);
        bencher.iter(|| x % &y);
    }

    #[bench]
    fn bench_powmod_256(bencher: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let p = Bigi::<8>::gen_random(&mut rng, 256, false);
        let m = Bigi::<8>::gen_random(&mut rng, 256, false);
        let x = Bigi::<8>::gen_random(&mut rng, 256, false) % &m;
        bencher.iter(|| x.powmod(&p, &m));
    }

    #[bench]
    fn bench_mod_2k_256(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let x = Bigi::<8>::gen_random(&mut rng, 256, false);
        b.iter(|| x.mod_2k(143));
    }

    #[bench]
    fn bench_shl_256(bencher: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let x = Bigi::<8>::gen_random(&mut rng, 256, false);
        bencher.iter(|| x << 143);
    }

    #[bench]
    fn bench_shr_256(bencher: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let x = Bigi::<8>::gen_random(&mut rng, 256, false);
        bencher.iter(|| x >> 143);
    }

    #[bench]
    fn bench_eq_256(bencher: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let x = Bigi::<8>::gen_random(&mut rng, 256, false);
        let y = Bigi::<8>::gen_random(&mut rng, 256, false);
        bencher.iter(|| x == y);
    }

    #[bench]
    fn bench_cmp_256(bencher: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let x = Bigi::<8>::gen_random(&mut rng, 256, false);
        let y = Bigi::<8>::gen_random(&mut rng, 256, false);
        bencher.iter(|| x < y);
    }
}
