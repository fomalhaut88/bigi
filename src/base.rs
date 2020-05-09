use std::mem;

include!("constants.rs");


pub type BigiType = u32;
pub type Bigi2Type = u64;

pub const BIGI_BYTES: usize = BIGI_BITS / 8;
pub const BIGI_TYPE_BYTES: usize = mem::size_of::<BigiType>();
pub const BIGI_TYPE_BITS: usize = 8 * BIGI_TYPE_BYTES;
pub const BIGI_MAX_DIGITS: usize = BIGI_BYTES / BIGI_TYPE_BYTES;


#[derive(Clone, Copy)]
pub struct Bigi {
    pub digits: [BigiType; BIGI_MAX_DIGITS],
    pub order: usize
}


impl Bigi {
    pub fn update_order(&mut self) {
        for i in (0..BIGI_MAX_DIGITS).rev() {
            if self.digits[i] != 0 {
                self.order = i + 1;
                return;
            }
        }
        self.order = 0;
    }
}


#[macro_export]
macro_rules! bigi {
    ($($x:expr),*) => [{
        let mut digits = [0; BIGI_MAX_DIGITS];
        let mut idx: usize = 0;
        $(
            #[allow(unused_assignments)]
            {
                digits[idx] = $x;
                idx += 1;
            }
        )*
        let mut res = Bigi { digits: digits, order: 0 };
        res.update_order();
        res
    }]
}


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_bigi_macro() {
        let x = bigi![2, 5, 6, 90];
        assert_eq!(x.digits[0], 2);
        assert_eq!(x.digits[1], 5);
        assert_eq!(x.digits[2], 6);
        assert_eq!(x.digits[3], 90);
        assert_eq!(x.digits[4], 0);
    }

    #[bench]
    fn bench_update_order_256(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let mut x = Bigi::gen_random(&mut rng, 256, false);
        b.iter(|| x.update_order());
    }

    #[bench]
    fn bench_bigi1_256(b: &mut Bencher) {
        b.iter(|| bigi![1]);
    }

    #[bench]
    fn bench_clone_256(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let x = Bigi::gen_random(&mut rng, 256, false);
        b.iter(|| x.clone());
    }
}
