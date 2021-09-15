use crate::{Bitwise, BitwiseDebug};

#[inline(always)]
fn mask(ndx: usize) -> u64 {
    1 << ndx
}

/// Implementation of the bitwise trait for the u8.
/// # Examples
///
/// ```
/// use yos::{Bitwise, BitwiseDebug};
/// let mut v = 5u64;
/// v.reset(0);
/// ```
impl Bitwise for u64 {
    fn set(&mut self, ndx: usize) {
        self.reset(ndx);
        *self |= mask(ndx);
    }

    fn reset(&mut self, ndx: usize) {
        *self &= !mask(ndx);
    }

    fn flip(&mut self, ndx: usize) {
        *self ^= mask(ndx);
    }

    fn get(&self, ndx: usize) -> u8 {
        let v = *self & mask(ndx);
        if v == 0 {
            0
        } else {
            1
        }
    }

    fn reset_low(&mut self, n: usize) {
        for ndx in 0..n {
            self.reset(ndx);
        }
    }

    fn reset_high(&mut self, n: usize) {
        for ndx in (63 - n)..63 {
            self.reset(ndx);
        }
    }

    fn ueights(&self) -> Vec<u8> {
        let msk = 255u64;

        let u0 = (*self & msk) as u8;
        let u1 = ((*self & (msk << 8)) >> 8) as u8;
        let u2 = ((*self & (msk << 16)) >> 16) as u8;
        let u3 = ((*self & (msk << 24)) >> 24) as u8;
        let u4 = ((*self & (msk << 32)) >> 32) as u8;
        let u5 = ((*self & (msk << 40)) >> 40) as u8;
        let u6 = ((*self & (msk << 48)) >> 48) as u8;
        let u7 = ((*self & (msk << 56)) >> 56) as u8;

        vec![u0, u1, u2, u3, u4, u5, u6, u7]
    }
}

/// # Examples
///
/// ```
/// use yos::{Bitwise, BitwiseDebug};
/// let mut v = 5u64;
/// v.reset(0);
/// println!("v={}", v.debug());
/// ```
impl BitwiseDebug for u64 {
    fn debug(&self) -> String {
        let str = format!("{:#066b}", *self);
        format!("u64:{}", str.strip_prefix("0b").unwrap())
    }
}

#[cfg(test)]
mod utest {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[test]
    fn set_bit() {
        let mut v = 5u64;
        v.set(1);
        assert_eq!(v, 7);
    }

    #[test]
    fn reset_bit() {
        let mut v = 7u64;
        v.reset(1);
        assert_eq!(v, 5);
    }

    #[test]
    fn flip_bit() {
        let mut v = 7u64;
        v.flip(1);
        assert_eq!(v, 5);

        v.flip(1);
        assert_eq!(v, 7);
    }

    #[test]
    fn get_bit() {
        let v = 5u64;
        let b = v.get(1);
        assert_eq!(b, 0);

        let b = v.get(2);
        assert_eq!(b, 1);
    }

    #[test]
    fn debug() {
        let v = 5u64;
        let str = v.debug();
        assert_eq!(str.len(), 68);
    }

    #[test]
    fn reset_low() {
        let mut v = 5u64;
        v.reset_low(2);
        assert_eq!(v, 4);
    }

    #[test]
    fn reset_high() {
        let mut v = 5u64;
        v.reset_high(62);
        assert_eq!(v, 1);
    }

    #[quickcheck]
    fn prop_ueights(val: u64) -> bool {
        let ueights = val.ueights();
        let u0 = (val & 255) as u8;
        (ueights.len() == 8) && (ueights[0] == u0)
    }
}
