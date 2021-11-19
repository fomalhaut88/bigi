//! This module implements methods to format Bigi into decimal string,
//! hex string, bytes and back.
//!
//! ```rust
//! use bigi::Bigi;
//!
//! let a = Bigi::<4>::from(28);
//! assert_eq!(a.to_hex(), "0x1C");
//! ```

use crate::base::Bigi;


impl<const N: usize> Bigi<N> {
    /// Converts the integer into a decimal string.
    pub fn to_decimal(&self) -> String {
        let mut decimal = String::new();
        let mut value = self.clone();
        let ten = Bigi::<N>::from(10);
        let zero = Bigi::<N>::from(0);

        while value > zero {
            let new_value = value.divide(&ten);
            decimal = value.digits[0].to_string() + &decimal;
            value = new_value;
        }

        if decimal.is_empty() {
            decimal += "0";
        }

        decimal
    }

    /// Converts decimal string into an integer.
    pub fn from_decimal(decimal: &str) -> Bigi<N> {
        let mut res = Bigi::<N>::from(0);
        let ten = Bigi::<N>::from(10);
        for ch in decimal.chars() {
            let digit = ch.to_string().parse::<u64>().unwrap();
            res = res * &ten + &Bigi::<N>::from(digit);
        }
        res
    }

    /// Converts the integer into a hex string.
    pub fn to_hex(&self) -> String {
        let mut hex = "0x".to_string();
        let mut is_started = false;

        for i in (0..N).rev() {
            if is_started || self.digits[i] > 0 {
                if is_started {
                    hex += &format!("{:016X}", self.digits[i]);
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

    /// Converts hex string into an integer.
    pub fn from_hex(hex: &str) -> Bigi<N> {
        let hex_without_pref = hex.trim_start_matches("0x");
        let mut res = Bigi::<N>::from(0);

        let length = hex_without_pref.chars().count();

        for i in 0..N {
            if 16 * i >= length {
                break;
            }

            let start_idx = if length >= 16 * (i + 1) {
                length - 16 * (i + 1)
            } else { 0 };
            let end_idx = length - 16 * i;
            let hex_sliced = &hex_without_pref[start_idx..end_idx];
            res.digits[i] = u64::from_str_radix(hex_sliced, 16).unwrap();
        }

        res
    }

    /// Converts the integer into a vector of bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.digits.iter()
            .map(|digit| digit.to_le_bytes())
            .collect::<Vec<[u8; 8]>>().concat()
    }

    /// Converts bytes into an integer.
    pub fn from_bytes(bytes: &[u8]) -> Bigi<N> {
        let mut res = Bigi::<N>::from(0);

        for i in 0..N {
            let mut buffer: [u8; 8] = [0; 8];
            buffer[..].clone_from_slice(&bytes[i << 3..(i + 1) << 3]);
            res.digits[i] = u64::from_le_bytes(buffer);
        }

        res
    }
}


#[cfg(test)]
mod tests {
    use crate::bigi;
    use super::*;
    use test::Bencher;

    #[test]
    fn test_to_decimal() {
        assert_eq!(bigi![8; 0].to_decimal(), "0");
        assert_eq!(bigi![8; 28].to_decimal(), "28");
        assert_eq!(bigi![8; 17963675599646983440, 11847261249634922397,
                            2906472191358324295, 535436869065898768,
                            17947275024882114903, 7073572901032037107,
                            15544459943282863109, 17620433762786716670
                        ].to_decimal(),
            "12807213597679137463932514806347052655957637939275876135854488682117351437458783714615759068249821520207988536511439106887893347291351711661378175857820944"
        );
    }

    #[test]
    fn test_from_decimal() {
        assert_eq!(Bigi::<8>::from_decimal("0"), bigi![8; 0]);
        assert_eq!(Bigi::<8>::from_decimal("28"), bigi![8; 28]);
        assert_eq!(
            Bigi::<8>::from_decimal(
                "12807213597679137463932514806347052655957637939275876135854488682117351437458783714615759068249821520207988536511439106887893347291351711661378175857820944"
            ), bigi![8; 17963675599646983440, 11847261249634922397,
                        2906472191358324295, 535436869065898768,
                        17947275024882114903, 7073572901032037107,
                        15544459943282863109, 17620433762786716670]
        );
    }

    #[test]
    fn test_to_hex() {
        assert_eq!(bigi![8; 0].to_hex(), "0x0");
        assert_eq!(bigi![8; 28].to_hex(), "0x1C");
        assert_eq!(bigi![8; 17963675599646983440, 11847261249634922397,
                            2906472191358324295, 535436869065898768,
                            17947275024882114903, 7073572901032037107,
                            15544459943282863109, 17620433762786716670
                        ].to_hex(),
            "0xF4885B267333ABFED7B903F5D8A9EC05622A61137AD80AF3F911878BE056A157076E40FEDC51FB102855DD0F1376CE47A469ED5649BD6F9DF94BCBC8414F8510"
        );
    }

    #[test]
    fn test_from_hex() {
        assert_eq!(Bigi::<8>::from_hex("0x0"), bigi![8; 0]);
        assert_eq!(Bigi::<8>::from_hex("0x1C"), bigi![8; 28]);
        assert_eq!(
            Bigi::<8>::from_hex(
                "0xF4885B267333ABFED7B903F5D8A9EC05622A61137AD80AF3F911878BE056A157076E40FEDC51FB102855DD0F1376CE47A469ED5649BD6F9DF94BCBC8414F8510"
            ), bigi![8; 17963675599646983440, 11847261249634922397,
                        2906472191358324295, 535436869065898768,
                        17947275024882114903, 7073572901032037107,
                        15544459943282863109, 17620433762786716670]
        );
    }

    #[test]
    fn test_to_bytes() {
        assert_eq!(
            bigi![8; 25].to_bytes(),
            vec![25, 0, 0, 0, 0, 0, 0, 0,
                  0, 0, 0, 0, 0, 0, 0, 0,
                  0, 0, 0, 0, 0, 0, 0, 0,
                  0, 0, 0, 0, 0, 0, 0, 0,
                  0, 0, 0, 0, 0, 0, 0, 0,
                  0, 0, 0, 0, 0, 0, 0, 0,
                  0, 0, 0, 0, 0, 0, 0, 0,
                  0, 0, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(
            bigi![8; 25, 11].to_bytes(),
            vec![25, 0, 0, 0, 0, 0, 0, 0,
                 11, 0, 0, 0, 0, 0, 0, 0,
                  0, 0, 0, 0, 0, 0, 0, 0,
                  0, 0, 0, 0, 0, 0, 0, 0,
                  0, 0, 0, 0, 0, 0, 0, 0,
                  0, 0, 0, 0, 0, 0, 0, 0,
                  0, 0, 0, 0, 0, 0, 0, 0,
                  0, 0, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(
            bigi![8; 1000, 11].to_bytes(),
            vec![232, 3, 0, 0, 0, 0, 0, 0,
                  11, 0, 0, 0, 0, 0, 0, 0,
                   0, 0, 0, 0, 0, 0, 0, 0,
                   0, 0, 0, 0, 0, 0, 0, 0,
                   0, 0, 0, 0, 0, 0, 0, 0,
                   0, 0, 0, 0, 0, 0, 0, 0,
                   0, 0, 0, 0, 0, 0, 0, 0,
                   0, 0, 0, 0, 0, 0, 0, 0]
        );
    }

    #[test]
    fn test_from_bytes() {
        assert_eq!(
            Bigi::<8>::from_bytes(&vec![25, 0, 0, 0, 0, 0, 0, 0,
                                         0, 0, 0, 0, 0, 0, 0, 0,
                                         0, 0, 0, 0, 0, 0, 0, 0,
                                         0, 0, 0, 0, 0, 0, 0, 0,
                                         0, 0, 0, 0, 0, 0, 0, 0,
                                         0, 0, 0, 0, 0, 0, 0, 0,
                                         0, 0, 0, 0, 0, 0, 0, 0,
                                         0, 0, 0, 0, 0, 0, 0, 0]),
            bigi![8; 25]
        );
        assert_eq!(
            Bigi::<8>::from_bytes(&vec![25, 0, 0, 0, 0, 0, 0, 0,
                                        11, 0, 0, 0, 0, 0, 0, 0,
                                         0, 0, 0, 0, 0, 0, 0, 0,
                                         0, 0, 0, 0, 0, 0, 0, 0,
                                         0, 0, 0, 0, 0, 0, 0, 0,
                                         0, 0, 0, 0, 0, 0, 0, 0,
                                         0, 0, 0, 0, 0, 0, 0, 0,
                                         0, 0, 0, 0, 0, 0, 0, 0]),
            bigi![8; 25, 11]
        );
        assert_eq!(
            Bigi::<8>::from_bytes(&vec![232, 3, 0, 0, 0, 0, 0, 0,
                                         11, 0, 0, 0, 0, 0, 0, 0,
                                          0, 0, 0, 0, 0, 0, 0, 0,
                                          0, 0, 0, 0, 0, 0, 0, 0,
                                          0, 0, 0, 0, 0, 0, 0, 0,
                                          0, 0, 0, 0, 0, 0, 0, 0,
                                          0, 0, 0, 0, 0, 0, 0, 0,
                                          0, 0, 0, 0, 0, 0, 0, 0]),
            bigi![8; 1000, 11]
        );
    }

    #[bench]
    fn bench_to_decimal_256(bencher: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let x = Bigi::<8>::gen_random(&mut rng, 256, false);
        bencher.iter(|| x.to_decimal());
    }

    #[bench]
    fn bench_from_decimal_256(bencher: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let x = Bigi::<8>::gen_random(&mut rng, 256, false);
        let y = x.to_decimal();
        bencher.iter(|| Bigi::<8>::from_decimal(&y));
    }

    #[bench]
    fn bench_to_hex_256(bencher: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let x = Bigi::<8>::gen_random(&mut rng, 256, false);
        bencher.iter(|| x.to_hex());
    }

    #[bench]
    fn bench_from_hex_256(bencher: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let x = Bigi::<8>::gen_random(&mut rng, 256, false);
        let y = x.to_hex();
        bencher.iter(|| Bigi::<8>::from_hex(&y));
    }

    #[bench]
    fn bench_to_bytes_256(bencher: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let x = Bigi::<8>::gen_random(&mut rng, 256, false);
        bencher.iter(|| x.to_bytes());
    }

    #[bench]
    fn bench_from_bytes_256(bencher: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let x = Bigi::<8>::gen_random(&mut rng, 256, false);
        let y = x.to_bytes();
        bencher.iter(|| Bigi::<8>::from_bytes(&y));
    }
}
