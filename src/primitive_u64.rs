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
///
/// let mut v = 5u64;
/// v.reset(0);
/// assert_eq!(4, v);
///
/// println!("v={}", v.debug());
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

    fn split(&self, cut: usize) -> (Self, Self)
    where
        Self: Sized,
    {
        let l = self & u64::low_mask(cut);
        let h = self & u64::high_mask(64 - cut);
        (l, h)
    }

    fn low_mask(len: usize) -> Self
    where
        Self: Sized,
    {
        if len == 0 {
            0
        } else {
            let mut v = 1u64;
            for _ in 0..len - 1 {
                v <<= 1;
                v += 1;
            }

            v
        }
    }

    fn high_mask(len: usize) -> Self
    where
        Self: Sized,
    {
        if len == 0 {
            0
        } else {
            let mut v = Self::low_mask(len);
            v <<= 64 - len;
            v
        }
    }
}

/// Prints in binary format an u8 value
/// without the 0b prefix.
fn u8_debug(u: &u8) -> String {
    let str = format!("{:#010b}", u);
    str.strip_prefix("0b").unwrap().to_string()
}

/// # Examples
///
/// ```
/// use yos::{Bitwise, BitwiseDebug};
///
/// let mut v = 5u64;
/// v.reset(0);
/// assert_eq!(4, v);
///
/// println!("v={}", v.debug());
/// ```
impl BitwiseDebug for u64 {
    fn debug(&self) -> String {
        let ueights = self.ueights().iter().map(u8_debug).collect::<Vec<String>>();
        format!(
            "u64:{:20}|{}|{}|{}|{}|{}|{}|{}|{}|",
            self,
            ueights[7],
            ueights[6],
            ueights[5],
            ueights[4],
            ueights[3],
            ueights[2],
            ueights[1],
            ueights[0]
        )
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
        assert_eq!(str.len(), 97);
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

    #[test]
    fn test_low_mask() {
        assert_eq!(u64::low_mask(0), 0);
        assert_eq!(u64::low_mask(1), 1);
        assert_eq!(u64::low_mask(2), 3);
        assert_eq!(u64::low_mask(3), 7);
        assert_eq!(u64::low_mask(4), 15);
        assert_eq!(u64::low_mask(5), 31);
        assert_eq!(u64::low_mask(6), 63);
        assert_eq!(u64::low_mask(7), 127);
        assert_eq!(u64::low_mask(8), 255);
    }

    #[test]
    fn test_high_mask() {
        assert_eq!(u64::high_mask(0), 0);
        assert_eq!(u64::high_mask(63), 18446744073709551614);
        assert_eq!(u64::high_mask(64), 18446744073709551615);
    }

    #[test]
    fn test_split() {
        let x = 56u64;

        for i in 0..65 {
            let (h, t) = x.split(i);
            assert_eq!(h + t, x);
        }
    }

}
