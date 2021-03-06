use std::{fmt, mem, cmp};
use crate::{bigi};
use crate::base::{Bigi, BigiType, BIGI_MAX_DIGITS, BIGI_BYTES, BIGI_TYPE_BYTES};


impl Bigi {
    pub fn to_decimal(&self) -> String {
        let mut decimal = String::new();
        let mut value = self.clone();
        let divisor = bigi![10];
        let zero = bigi![0];

        while value > zero {
            let new_value = value.divide(&divisor);
            decimal = value.digits[0].to_string() + &decimal;
            value = new_value;
        }

        if decimal.is_empty() {
            decimal += "0";
        }

        decimal
    }

    pub fn from_decimal(decimal: &str) -> Bigi {
        let mut res = bigi![0];
        for ch in decimal.chars() {
            let digit = ch.to_string().parse::<BigiType>().unwrap();
            res = res * &bigi![10] + &bigi![digit];
        }
        res.update_order();
        res
    }

    pub fn to_hex(&self) -> String {
        let mut hex = "0x".to_string();
        let mut is_started = false;

        for i in (0..BIGI_MAX_DIGITS).rev() {
            if is_started || self.digits[i] > 0 {
                if is_started {
                    hex += &format!("{:08X}", self.digits[i]);
                } else {
                    hex += &format!("{:X}", self.digits[i]);
                    is_started = true;
                }
            }
        }

        if hex == "0x" {
            hex += "0";
        }

        hex
    }

    pub fn from_hex(hex: &str) -> Bigi {
        let hex_without_pref = hex.trim_start_matches("0x");
        let mut res = bigi![0];

        let length = hex_without_pref.chars().count();

        for i in 0..BIGI_MAX_DIGITS {
            if 8 * i >= length {
                break;
            }

            let start_idx = if length >= 8 * (i + 1) { length - 8 * (i + 1) } else { 0 };
            let end_idx = length - 8 * i;
            let hex_sliced = &hex_without_pref[start_idx..end_idx];
            res.digits[i] = BigiType::from_str_radix(hex_sliced, 16).unwrap();
        }

        res.update_order();

        res
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        (unsafe {
            mem::transmute::<_, [u8; BIGI_BYTES]>(self.digits)
        }).to_vec()
    }

    pub fn from_bytes(bytes: &[u8]) -> Bigi {
        let mut res = bigi![0];

        for i in 0..BIGI_MAX_DIGITS {
            let start = BIGI_TYPE_BYTES * i;
            let end = cmp::min(start + BIGI_TYPE_BYTES, bytes.len());

            if start >= bytes.len() {
                break;
            }

            let mut buffer: [u8; BIGI_TYPE_BYTES] = [0; BIGI_TYPE_BYTES];
            buffer[..(end - start)].clone_from_slice(&bytes[start..end]);
            res.digits[i] = unsafe { mem::transmute(buffer) };
        }

        res.update_order();
        res
    }
}


impl fmt::Display for Bigi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_decimal())
    }
}


impl fmt::Debug for Bigi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_decimal())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_to_decimal() {
        assert_eq!(bigi![0].to_decimal(), "0");
        assert_eq!(bigi![28].to_decimal(), "28");
        assert_eq!(bigi![3567587328, 232, 0, 29].to_decimal(), "2297616712913665791212774559744");
        assert_eq!(bigi![1095730448, 4182494152, 1237151645, 2758405462, 326553159, 676715791, 3696360208, 124666110, 3763773783, 4178675595, 2060978931, 1646944531, 3635014661, 3619226613, 1932766206, 4102576934].to_decimal(), "12807213597679137463932514806347052655957637939275876135854488682117351437458783714615759068249821520207988536511439106887893347291351711661378175857820944");
    }

    #[test]
    fn test_from_decimal() {
        assert_eq!(Bigi::from_decimal("0"), bigi![0]);
        assert_eq!(Bigi::from_decimal("28"), bigi![28]);
        assert_eq!(Bigi::from_decimal("2297616712913665791212774559744"), bigi![3567587328, 232, 0, 29]);
        assert_eq!(Bigi::from_decimal("12807213597679137463932514806347052655957637939275876135854488682117351437458783714615759068249821520207988536511439106887893347291351711661378175857820944"), bigi![1095730448, 4182494152, 1237151645, 2758405462, 326553159, 676715791, 3696360208, 124666110, 3763773783, 4178675595, 2060978931, 1646944531, 3635014661, 3619226613, 1932766206, 4102576934]);
    }

    #[test]
    fn test_to_hex() {
        assert_eq!(bigi![0].to_hex(), "0x0");
        assert_eq!(bigi![28].to_hex(), "0x1C");
        assert_eq!(bigi![3567587328, 232, 0, 29].to_hex(), "0x1D00000000000000E8D4A51000");
        assert_eq!(bigi![1095730448, 4182494152, 1237151645, 2758405462, 326553159, 676715791, 3696360208, 124666110, 3763773783, 4178675595, 2060978931, 1646944531, 3635014661, 3619226613, 1932766206, 4102576934].to_hex(), "0xF4885B267333ABFED7B903F5D8A9EC05622A61137AD80AF3F911878BE056A157076E40FEDC51FB102855DD0F1376CE47A469ED5649BD6F9DF94BCBC8414F8510");
    }

    #[test]
    fn test_from_hex() {
        assert_eq!(Bigi::from_hex("0x0"), bigi![0]);
        assert_eq!(Bigi::from_hex("0x1C"), bigi![28]);
        assert_eq!(Bigi::from_hex("0x1D00000000000000E8D4A51000"), bigi![3567587328, 232, 0, 29]);
        assert_eq!(Bigi::from_hex("0xF4885B267333ABFED7B903F5D8A9EC05622A61137AD80AF3F911878BE056A157076E40FEDC51FB102855DD0F1376CE47A469ED5649BD6F9DF94BCBC8414F8510"), bigi![1095730448, 4182494152, 1237151645, 2758405462, 326553159, 676715791, 3696360208, 124666110, 3763773783, 4178675595, 2060978931, 1646944531, 3635014661, 3619226613, 1932766206, 4102576934]);
    }

    #[test]
    fn test_to_bytes() {
        assert_eq!(bigi![25].to_bytes(), vec![25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(bigi![25, 11].to_bytes(), vec![25, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(bigi![1000, 11].to_bytes(), vec![232, 3, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_from_bytes() {
        assert_eq!(Bigi::from_bytes(&vec![25]), bigi![25]);
        assert_eq!(Bigi::from_bytes(&vec![25, 0, 0, 0, 11]), bigi![25, 11]);
        assert_eq!(Bigi::from_bytes(&vec![232, 3, 0, 0, 11]), bigi![1000, 11]);
        assert_eq!(Bigi::from_bytes(&vec![232, 3, 123, 250, 11]), bigi![4202365928, 11]);
        assert_eq!(Bigi::from_bytes(&vec![232, 3, 123, 250]), bigi![4202365928]);
    }

    #[bench]
    fn bench_to_decimal_256(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let x = Bigi::gen_random(&mut rng, 256, false);
        b.iter(|| x.to_decimal());
    }

    #[bench]
    fn bench_from_decimal_256(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let x = Bigi::gen_random(&mut rng, 256, false);
        let y = x.to_decimal();
        b.iter(|| Bigi::from_decimal(&y));
    }

    #[bench]
    fn bench_to_hex_256(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let x = Bigi::gen_random(&mut rng, 256, false);
        b.iter(|| x.to_hex());
    }

    #[bench]
    fn bench_from_hex_256(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let x = Bigi::gen_random(&mut rng, 256, false);
        let y = x.to_hex();
        b.iter(|| Bigi::from_hex(&y));
    }

    #[bench]
    fn bench_to_bytes_256(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let x = Bigi::gen_random(&mut rng, 256, false);
        b.iter(|| x.to_bytes());
    }

    #[bench]
    fn bench_from_bytes_256(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let x = Bigi::gen_random(&mut rng, 256, false);
        let y = x.to_bytes();
        b.iter(|| Bigi::from_bytes(&y));
    }
}
