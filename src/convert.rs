use std::convert;
use crate::{bigi, base::{Bigi, BigiType, Bigi2Type, BIGI_TYPE_BITS, BIGI_MAX_DIGITS}};


impl convert::From<BigiType> for Bigi {
    fn from(z: BigiType) -> Self {
        bigi![z]
    }
}


impl convert::From<Bigi> for BigiType {
    fn from(a: Bigi) -> BigiType {
        a.digits[0]
    }
}


impl convert::From<Bigi2Type> for Bigi {
    fn from(z: Bigi2Type) -> Self {
        bigi![z as BigiType, (z >> BIGI_TYPE_BITS) as BigiType]
    }
}


impl convert::From<Bigi> for Bigi2Type {
    fn from(a: Bigi) -> Bigi2Type {
        ((a.digits[1] as Bigi2Type) << BIGI_TYPE_BITS) + (a.digits[0] as Bigi2Type)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_bigitype() {
        assert_eq!(Bigi::from(28 as u32), bigi![28]);
    }

    #[test]
    fn test_from_bigitype2() {
        assert_eq!(Bigi::from(1000000000000 as u64), bigi![3567587328, 232]);
    }

    #[test]
    fn test_to_bigitype() {
        assert_eq!(u32::from(bigi![28]), 28);
        assert_eq!(u32::from(bigi![28, 11, 64]), 28);
    }

    #[test]
    fn test_to_bigitype2() {
        assert_eq!(u64::from(bigi![3567587328, 232]), 1000000000000);
        assert_eq!(u64::from(bigi![3567587328, 232, 0, 29]), 1000000000000);
    }
}
