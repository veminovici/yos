use crate::{Bitwise, BitwiseDebug};

#[inline(always)]
fn mask(ndx: usize) -> u8 {
    1 << ndx
}

/// Implementation of the bitwise trait for the u8.
/// # Examples
///
/// ```
/// use yos::{Bitwise, BitwiseDebug};
///
/// let mut v = 5u8;
/// v.reset(0);
/// assert_eq!(4, v);
///
/// ```
impl Bitwise for u8 {
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
        for ndx in (7 - n)..7 {
            self.reset(ndx);
        }
    }

    fn ueights(&self) -> Vec<u8> {
        vec![*self]
    }

    fn split(&self, _cut: usize) -> (Self, Self)
    where
        Self: Sized,
    {
        todo!("Not implemented yet!")
    }

    fn low_mask(len: usize) -> Self
    where
        Self: Sized,
    {
        if len == 0 {
            0
        } else {
            let mut v = 1u8;
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
            v <<= 8 - len;
            v
        }
    }
}

/// # Examples
///
/// ```
/// use yos::{Bitwise, BitwiseDebug};
///
/// let mut v = 5u8;
/// v.reset(0);
/// assert_eq!(4, v);
///
/// println!("v={}", v.debug());
/// ```
impl BitwiseDebug for u8 {
    fn debug(&self) -> String {
        let str = format!("{:#010b}", *self);
        format!("u8:{:3}|{}|", self, str.strip_prefix("0b").unwrap())
    }
}

#[cfg(test)]
mod utest {
    use super::*;

    use quickcheck_macros::quickcheck;

    #[test]
    fn set_bit() {
        let mut v = 5u8;
        v.set(1);
        assert_eq!(v, 7);
    }

    #[test]
    fn reset_bit() {
        let mut v = 7u8;
        v.reset(1);
        assert_eq!(v, 5);
    }

    #[test]
    fn flip_bit() {
        let mut v = 7u8;
        v.flip(1);
        assert_eq!(v, 5);

        v.flip(1);
        assert_eq!(v, 7);
    }

    #[test]
    fn get_bit() {
        let v = 5u8;
        let b = v.get(1);
        assert_eq!(b, 0);

        let b = v.get(2);
        assert_eq!(b, 1);
    }

    #[test]
    fn debug() {
        let v = 5u8;
        let str = v.debug();
        assert_eq!(str.len(), 16);
    }

    #[test]
    fn reset_low() {
        let mut v = 5u8;
        v.reset_low(2);
        assert_eq!(v, 4);
    }

    #[test]
    fn reset_high() {
        let mut v = 5u8;
        v.reset_high(6);
        assert_eq!(v, 1);
    }

    #[quickcheck]
    fn prop_ueights(val: u8) -> bool {
        let ueights = val.ueights();
        !ueights.is_empty() && (ueights[0] == val)
    }

    #[test]
    fn test_low_mask() {
        assert_eq!(u8::low_mask(0), 0);
        assert_eq!(u8::low_mask(1), 1);
        assert_eq!(u8::low_mask(2), 3);
        assert_eq!(u8::low_mask(3), 7);
        assert_eq!(u8::low_mask(4), 15);
        assert_eq!(u8::low_mask(5), 31);
        assert_eq!(u8::low_mask(6), 63);
        assert_eq!(u8::low_mask(7), 127);
        assert_eq!(u8::low_mask(8), 255);
    }

    #[test]
    fn test_high_mask() {
        assert_eq!(u8::high_mask(0), 0);
        assert_eq!(u8::high_mask(8), 255);
    }
}
