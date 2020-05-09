use std::{ops, cmp};
use crate::{bigi, base::{Bigi, BigiType, Bigi2Type, BIGI_MAX_DIGITS, BIGI_TYPE_BITS}};


impl Bigi {
    pub fn is_zero(&self) -> bool {
        self.order == 0
    }

    pub fn is_odd(&self) -> bool {
        self.digits[0] % 2 == 1
    }

    pub fn is_even(&self) -> bool {
        self.digits[0] % 2 == 0
    }

    pub fn bit_length(&self) -> usize {
        if self.order != 0 {
            let l = {
                let mut l = 0;
                let mut digit = self.digits[self.order - 1];
                while digit != 0 {
                    digit >>= 1;
                    l += 1;
                }
                l
            };
            l + BIGI_TYPE_BITS * (self.order - 1)
        } else {
            0
        }
    }

    pub fn get_bit(&self, bit: usize) -> bool {
        let quot = bit / BIGI_TYPE_BITS;
        let rem = bit % BIGI_TYPE_BITS;
        (self.digits[quot] & (1 << rem)) != 0
    }

    fn lead_digit2(&self) -> Bigi2Type {
        for i in (0..BIGI_MAX_DIGITS).rev() {
            if self.digits[i] != 0 {
                if i == 0 {
                    return self.digits[i] as Bigi2Type;
                }
                else {
                    return ((self.digits[i] as Bigi2Type) << BIGI_TYPE_BITS) + (self.digits[i - 1] as Bigi2Type);
                }
            }
        }
        0
    }

    pub fn divide(&mut self, divisor: &Bigi) -> Bigi {
        let mut res = bigi![0];
        let order1 = self.order;
        let order2 = divisor.order;

        if order1 < order2 {
            return bigi![0];
        }

        let mut shf = order1 - order2;

        loop {
            let extra = if order2 + shf < BIGI_MAX_DIGITS { self.digits[order2 + shf] } else { 0 as BigiType };

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
                let top = self.lead_digit2();
                let result;

                let bottom = if extra > 0 {
                    divisor.digits[order2 - 1] as Bigi2Type
                } else {
                    if order2 == 1 && shf > 0 {
                        divisor.lead_digit2() << BIGI_TYPE_BITS
                    } else {
                        divisor.lead_digit2()
                    }
                };

                if top == bottom {
                    result = 1;
                } else {
                    result = top / (bottom + 1);
                }

                result as BigiType
            };

            // Adding factor to the result
            res.digits[shf] += factor;

            // Reducing dividend
            let mut fw: Bigi2Type = 0;
            for i in 0..order2 {
                fw = (divisor.digits[i] as Bigi2Type) * (factor as Bigi2Type) + fw;
                let pair = self.digits[i + shf].overflowing_sub(fw as BigiType);
                self.digits[i + shf] = pair.0;
                fw >>= BIGI_TYPE_BITS;
                if pair.1 {
                    fw += 1;
                }
            }
            if fw > 0 && order2 + shf < BIGI_MAX_DIGITS {
                self.digits[order2 + shf] -= fw as BigiType;
            }
        }

        self.update_order();
        res.update_order();

        return res;
    }

    pub fn powmod(&self, p: &Bigi, m: &Bigi) -> Bigi {
        let mut res = bigi![1];
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

    pub fn mod_2k(&self, k: usize) -> Bigi {
        let mut res = self.clone();
        let q = k / BIGI_TYPE_BITS;
        let r = k % BIGI_TYPE_BITS;
        for i in (q + 1)..self.order {
            res.digits[i] = 0;
        }
        res.digits[q] %= 1 << r;
        res.update_order();
        res
    }
}


impl cmp::PartialEq for Bigi {
    fn eq(&self, other: &Bigi) -> bool {
        let order = cmp::max(self.order, other.order);
        for i in 0..order {
            if self.digits[i] != other.digits[i] {
                return false;
            }
        }
        true
    }
}


impl cmp::PartialOrd for Bigi {
    fn partial_cmp(&self, other: &Bigi) -> Option<cmp::Ordering> {
        let order = cmp::max(self.order, other.order);
        for i in (0..order).rev() {
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


impl ops::Add<&Bigi> for Bigi {
    type Output = Bigi;

    fn add(self, other: &Bigi) -> Bigi {
        let mut res = self.clone();
        res += other;
        res
    }
}


impl ops::AddAssign<&Bigi> for Bigi {
    fn add_assign(&mut self, other: &Bigi) {
        let mut fw: BigiType = 0;

        let order = cmp::max(self.order, other.order) + 1;

        for i in 0..cmp::min(BIGI_MAX_DIGITS, order) {
            let pair = self.digits[i].overflowing_add(other.digits[i]);
            self.digits[i] = pair.0.overflowing_add(fw).0;
            fw = (pair.1 || (fw == 1 && self.digits[i] == 0)) as BigiType;
        }

        self.update_order();
    }
}


impl ops::Sub<&Bigi> for Bigi {
    type Output = Bigi;

    fn sub(self, other: &Bigi) -> Bigi {
        let mut res = self.clone();
        res -= other;
        res
    }
}


impl ops::SubAssign<&Bigi> for Bigi {
    fn sub_assign(&mut self, other: &Bigi) {
        let order = if *self > *other { self.order } else { BIGI_MAX_DIGITS };

        let mut fw: BigiType = 0;

        for i in 0..order {
            let pair = self.digits[i].overflowing_sub(other.digits[i]);
            self.digits[i] = pair.0.overflowing_sub(fw).0;
            fw = (pair.1 || (fw == 1 && pair.0 == 0)) as BigiType;
        }

        self.update_order();
    }
}


impl ops::Mul<&Bigi> for Bigi {
    type Output = Bigi;

    fn mul(self, other: &Bigi) -> Bigi {
        let mut res = bigi![0];
        for i in 0..other.order {
            let mut fw: Bigi2Type = 0;
            let order = cmp::min(self.order + 1, BIGI_MAX_DIGITS - i);
            for j in 0..order {
                fw = (other.digits[i] as Bigi2Type) * (self.digits[j] as Bigi2Type) + (res.digits[i + j] as Bigi2Type) + fw;
                res.digits[i + j] = fw as BigiType;
                fw >>= BIGI_TYPE_BITS;
            }
        }
        res.update_order();
        res
    }
}


impl ops::MulAssign<&Bigi> for Bigi {
    fn mul_assign(&mut self, other: &Bigi) {
        *self = *self * other;
    }
}


impl ops::Div<&Bigi> for Bigi {
    type Output = Bigi;

    fn div(self, other: &Bigi) -> Bigi {
        let mut dividend = self.clone();
        dividend.divide(other)
    }
}


impl ops::DivAssign<&Bigi> for Bigi {
    fn div_assign(&mut self, other: &Bigi) {
        *self = *self / other;
    }
}


impl ops::Rem<&Bigi> for Bigi {
    type Output = Bigi;

    fn rem(self, other: &Bigi) -> Bigi {
        let mut res = self.clone();
        res %= other;
        res
    }
}


impl ops::RemAssign<&Bigi> for Bigi {
    fn rem_assign(&mut self, other: &Bigi) {
        self.divide(other);
    }
}


impl ops::ShlAssign<usize> for Bigi {
    fn shl_assign(&mut self, rhs: usize) {
        let rhs_q = rhs / BIGI_TYPE_BITS;
        let rhs_r = rhs % BIGI_TYPE_BITS;
        let mut extra: BigiType = 0;

        for i in (0..(BIGI_MAX_DIGITS - rhs_q)).rev() {
            if rhs_r > 0 {
                extra = self.digits[i] >> (BIGI_TYPE_BITS - rhs_r);
            }
            if i < BIGI_MAX_DIGITS - rhs_q - 1 {
                self.digits[i + rhs_q + 1] += extra;
            }
            self.digits[i + rhs_q] = self.digits[i] << rhs_r;
        }

        for i in 0..rhs_q {
            self.digits[i] = 0;
        }

        self.update_order();
    }
}


impl ops::Shl<usize> for Bigi {
    type Output = Bigi;

    fn shl(self, rhs: usize) -> Bigi {
        let mut res = self.clone();
        res <<= rhs;
        res
    }
}


impl ops::ShrAssign<usize> for Bigi {
    fn shr_assign(&mut self, rhs: usize) {
        let rhs_q = rhs / BIGI_TYPE_BITS;
        let rhs_r = rhs % BIGI_TYPE_BITS;
        let mut extra: BigiType = 0;

        for i in rhs_q..self.order {
            if rhs_r > 0 {
                extra = self.digits[i] << (BIGI_TYPE_BITS - rhs_r);
            }
            self.digits[i - rhs_q] = self.digits[i] >> rhs_r;
            if i > rhs_q {
                self.digits[i - rhs_q - 1] += extra;
            }
        }

        for i in 0..rhs_q {
            self.digits[self.order - i - 1] = 0;
        }

        self.update_order();
    }
}


impl ops::Shr<usize> for Bigi {
    type Output = Bigi;

    fn shr(self, rhs: usize) -> Bigi {
        let mut res = self.clone();
        res >>= rhs;
        res
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_add() {
        assert_eq!(bigi![2] + &bigi![3], bigi![5]);
        assert_eq!(bigi![3567587328, 232, 0, 29] + &bigi![12312344, 1, 1234098120, 21556, 134236576], bigi![3579899672, 233, 1234098120, 21585, 134236576]);
    }

    #[test]
    fn test_sub() {
        assert_eq!(bigi![5] - &bigi![2], bigi![3]);
        assert_eq!(bigi![12312344, 1, 1234098120, 21556, 134236576] - &bigi![3567587328, 232, 0, 29], bigi![739692312, 4294967064, 1234098119, 21527, 134236576]);
    }

    #[test]
    fn test_mul() {
        assert_eq!(bigi![5] * &bigi![2], bigi![10]);
        assert_eq!(bigi![12312344, 1, 1234098120, 21556, 134236576] * &bigi![3567587328, 232, 0, 29], bigi![1751744512, 2139311010, 2707718377, 1453116243, 4177958257, 2618724431, 625139, 3892860704]);
    }

    #[test]
    fn test_div() {
        assert_eq!(bigi![5] / &bigi![2], bigi![2]);
        assert_eq!(bigi![12312344, 1, 1234098120, 21556, 134236576] / &bigi![3567587328, 232, 0, 29], bigi![1925330910, 4628847]);
    }

    #[test]
    fn test_rem() {
        assert_eq!(bigi![5] % &bigi![2], bigi![1]);
        assert_eq!(bigi![12312344, 1, 1234098120, 21556, 134236576] % &bigi![3567587328, 232, 0, 29], bigi![52952856, 1040751155, 156360589, 14]);
    }

    #[test]
    fn test_shl() {
        assert_eq!(bigi![100] << 2, bigi![400]);
        assert_eq!(bigi![100, 1] << 40, bigi![0, 25600, 256]);
        assert_eq!(bigi![3567587328, 232, 0, 29] << 96, bigi![0, 0, 0, 3567587328, 232, 0, 29]);
    }

    #[test]
    fn test_shr() {
        assert_eq!(bigi![400] >> 2, bigi![100]);
        assert_eq!(bigi![0, 25600, 256] >> 40, bigi![100, 1]);
        assert_eq!(bigi![1751744512, 2139311010, 2707718377, 1453116243, 4177958257, 2618724431, 625139, 3892860704] >> 128, bigi![4177958257, 2618724431, 625139, 3892860704]);
    }

    #[test]
    fn test_is_odd() {
        assert_eq!(bigi![5, 26].is_odd(), true);
        assert_eq!(bigi![0, 26].is_odd(), false);
        assert_eq!(bigi![0].is_odd(), false);
    }

    #[test]
    fn test_is_even() {
        assert_eq!(bigi![5, 26].is_even(), false);
        assert_eq!(bigi![0, 26].is_even(), true);
        assert_eq!(bigi![0].is_even(), true);
    }

    #[test]
    fn test_bit_length() {
        assert_eq!(bigi![29].bit_length(), 5);
        assert_eq!(bigi![8].bit_length(), 4);
        assert_eq!(bigi![0].bit_length(), 0);
        assert_eq!(bigi![0, 0, 29].bit_length(), 69);
        assert_eq!(bigi![0, 0, 8].bit_length(), 68);
        assert_eq!(bigi![0, 0, 0, 1].bit_length(), 97);
        assert_eq!(bigi![0, 0, 0, 2].bit_length(), 98);
        assert_eq!(bigi![0, 0, 0, 4294967295].bit_length(), 128);
    }

    #[test]
    fn test_is_zero() {
        assert_eq!(bigi![0].is_zero(), true);
        assert_eq!(bigi![1].is_zero(), false);
        assert_eq!(bigi![5].is_zero(), false);
        assert_eq!(bigi![0, 0].is_zero(), true);
        assert_eq!(bigi![0, 1].is_zero(), false);
        assert_eq!(bigi![0, 10].is_zero(), false);
        assert_eq!(bigi![5, 10].is_zero(), false);
    }

    #[test]
    fn test_mod_2k() {
        assert_eq!(bigi![26].mod_2k(3), bigi![2]);
        assert_eq!(bigi![26].mod_2k(1), bigi![0]);
        assert_eq!(bigi![1].mod_2k(5), bigi![1]);
        assert_eq!(bigi![1, 45].mod_2k(5), bigi![1]);
        assert_eq!(bigi![1, 45].mod_2k(38), bigi![1, 45]);
        assert_eq!(bigi![1751744512, 2139311010, 2707718377, 1453116243, 4177958257, 2618724431, 625139, 3892860704].mod_2k(120), bigi![1751744512, 2139311010, 2707718377, 1453116243, 4177958257, 2618724431, 625139, 3892860704] % &(bigi![1] << 120));
        assert_eq!(bigi![1751744512, 2139311010, 2707718377, 1453116243, 4177958257, 2618724431, 625139, 3892860704].mod_2k(96), bigi![1751744512, 2139311010, 2707718377, 1453116243, 4177958257, 2618724431, 625139, 3892860704] % &(bigi![1] << 96));
    }

    #[bench]
    fn bench_add_256(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let x = Bigi::gen_random(&mut rng, 256, false);
        let y = Bigi::gen_random(&mut rng, 256, false);
        b.iter(|| x + &y);
    }

    #[bench]
    fn bench_sub_256(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let x = Bigi::gen_random(&mut rng, 256, false);
        let y = Bigi::gen_random(&mut rng, 256, false);
        b.iter(|| x - &y);
    }

    #[bench]
    fn bench_mul_256(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let x = Bigi::gen_random(&mut rng, 256, false);
        let y = Bigi::gen_random(&mut rng, 256, false);
        b.iter(|| x * &y);
    }

    #[bench]
    fn bench_div_256_256(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let x = Bigi::gen_random(&mut rng, 256, false);
        let y = Bigi::gen_random(&mut rng, 256, false);
        b.iter(|| x / &y);
    }

    #[bench]
    fn bench_div_256_128(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let x = Bigi::gen_random(&mut rng, 256, false);
        let y = Bigi::gen_random(&mut rng, 128, false);
        b.iter(|| x / &y);
    }

    #[bench]
    fn bench_mod_256_256(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let x = Bigi::gen_random(&mut rng, 256, false);
        let y = Bigi::gen_random(&mut rng, 256, false);
        b.iter(|| x % &y);
    }

    #[bench]
    fn bench_mod_256_128(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let x = Bigi::gen_random(&mut rng, 256, false);
        let y = Bigi::gen_random(&mut rng, 128, false);
        b.iter(|| x % &y);
    }

    #[bench]
    fn bench_cmp_256(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let x = Bigi::gen_random(&mut rng, 256, false);
        let y = Bigi::gen_random(&mut rng, 256, false);
        b.iter(|| x > y);
    }

    #[bench]
    fn bench_powmod_256(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let p = Bigi::gen_random(&mut rng, 256, false);
        let m = Bigi::gen_random(&mut rng, 256, false);
        let x = Bigi::gen_random(&mut rng, 256, false) % &m;
        b.iter(|| x.powmod(&p, &m));
    }

    #[bench]
    fn bench_shr_256(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let x = Bigi::gen_random(&mut rng, 256, false);
        b.iter(|| x >> 128);
    }

    #[bench]
    fn bench_shl_256(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let x = Bigi::gen_random(&mut rng, 256, false);
        b.iter(|| x << 128);
    }

    #[bench]
    fn bench_mod_2k_256(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let x = Bigi::gen_random(&mut rng, 256, false);
        b.iter(|| x.mod_2k(120));
    }
}
