use crate::{Bit, Bitstring, BitstringDebug};

/// Implementation of the bitstring trait for the u8.
/// # Examples
///
/// ```
/// use yos::Bitstring;
///
/// let mut v = 5u8;
/// v.brst(0);
/// assert_eq!(4, v);
///
/// ```
impl Bitstring for u8 {
    fn band(&mut self, other: &Self) {
        *self &= *other
    }

    fn bget(&self, ndx: usize) -> Bit {
        let mut v = *self;
        v.band(&Self::bpow2(ndx));

        if v == 0 {
            Bit::Zero
        } else {
            Bit::One
        }
    }

    fn blen(&self) -> usize {
        8
    }

    fn bneg(&mut self) {
        *self = !*self
    }

    fn bone() -> Self {
        1
    }

    fn bone_low(len: usize) -> Self {
        if len == 0 {
            Self::bzero()
        } else {
            let mut v = 1u8;
            for _ in 0..len - 1 {
                v <<= 1;
                v += 1;
            }

            v
        }
    }

    fn bor(&mut self, other: &Self) {
        *self |= *other
    }

    fn bpow2(p: usize) -> Self {
        1 << p
    }

    fn blshift(&mut self, len: usize) {
        *self <<= len;
    }

    fn bsplit(&self, cut: usize) -> (Self, Self)
    where
        Self: Sized,
    {
        let len = self.blen();
        let l = self & Self::bone_low(cut);
        let h = self & Self::bone_high(len - cut);
        (l, h)
    }

    fn bueights(&self) -> Vec<u8> {
        vec![*self]
    }

    fn bxor(&mut self, other: &Self) {
        *self ^= *other
    }

    fn bzero() -> Self {
        0
    }
}

/// # Examples
///
/// ```
/// use yos::{Bitstring, BitstringDebug};
///
/// let mut v = 5u8;
/// v.brst(0);
/// assert_eq!(4, v);
///
/// println!("v={}", v.bdebug());
/// ```
impl BitstringDebug for u8 {
    fn bdebug(&self) -> String {
        let str = format!("{:#010b}", *self);
        format!("u8:{:3}|{}|", self, str.strip_prefix("0b").unwrap())
    }
}

#[cfg(test)]
mod utest {
    use super::*;

    use quickcheck_macros::quickcheck;

    #[test]
    fn test_blen() {
        let a = 5u8;
        assert_eq!(a.blen(), 8);
    }

    #[test]
    fn test_bor() {
        let mut a = 5u8;
        let b = 2u8;
        a.bor(&b);
        assert_eq!(a, 7);
    }

    #[test]
    fn test_band() {
        let mut a = 5u8;
        let b = 3u8;
        a.band(&b);
        assert_eq!(a, 1);
    }

    #[test]
    fn test_bneg() {
        let mut a = 5u8;
        a.bneg();
        a.bneg();
        assert_eq!(a, 5);
    }

    #[test]
    fn test_bxor() {
        let mut a = 5u8;
        let b = 3u8;
        a.bxor(&b);
        assert_eq!(a, 6);
    }

    #[test]
    fn test_blshift() {
        let mut x = 1u8;
        x.blshift(2);
        assert_eq!(x, 4);
    }

    #[test]
    fn test_bpow2() {
        assert_eq!(u8::bpow2(0), 1);
        assert_eq!(u8::bpow2(1), 2);
        assert_eq!(u8::bpow2(2), 4);
        assert_eq!(u8::bpow2(3), 8);
    }

    #[test]
    fn test_brst() {
        let mut v = 7u8;
        v.brst(1);
        assert_eq!(v, 5);
    }

    #[test]
    fn test_bset() {
        let mut v = 5u8;
        v.bset(1);
        assert_eq!(v, 7);
    }

    #[test]
    fn test_bflip() {
        let mut v = 7u8;
        v.bflip(1);
        assert_eq!(v, 5);

        v.bflip(1);
        assert_eq!(v, 7);
    }

    #[test]
    fn test_bget() {
        let v = 5u8;
        let b = v.bget(1);
        assert_eq!(b, Bit::Zero);

        let b = v.bget(2);
        assert_eq!(b, Bit::One);

        assert_eq!(v, 5);
    }

    #[test]
    fn test_brst_low() {
        let mut v = 5u8;
        v.brst_low(2);
        assert_eq!(v, 4);
    }

    #[test]
    fn test_brst_high() {
        let mut v = 5u8;
        v.brst_high(6);
        assert_eq!(v, 1);
    }

    #[quickcheck]
    fn prop_bueights(val: u8) -> bool {
        let ueights = val.bueights();
        !ueights.is_empty() && (ueights[0] == val)
    }

    #[test]
    fn test_bzero() {
        assert_eq!(u8::bzero(), 0);
    }

    #[test]
    fn test_bone() {
        assert_eq!(u8::bone(), 1);
    }

    #[test]
    fn test_bone_low() {
        assert_eq!(u8::bone_low(0), 0);
        assert_eq!(u8::bone_low(1), 1);
        assert_eq!(u8::bone_low(2), 3);
        assert_eq!(u8::bone_low(3), 7);
        assert_eq!(u8::bone_low(4), 15);
        assert_eq!(u8::bone_low(5), 31);
        assert_eq!(u8::bone_low(6), 63);
        assert_eq!(u8::bone_low(7), 127);
        assert_eq!(u8::bone_low(8), 255);
    }

    #[test]
    fn test_bone_high() {
        assert_eq!(u8::bone_high(0), 0);
        assert_eq!(u8::bone_high(8), 255);
    }

    #[test]
    fn test_bsplit() {
        let x = 56u8;

        for i in 0..9 {
            let (h, t) = x.bsplit(i);
            assert_eq!(h + t, x);
        }
    }

    #[test]
    fn test_bcombine() {
        let x = 56u8;

        for i in 0..9 {
            let (mut h, t) = x.bsplit(i);
            assert_eq!(h + t, x);
            h.bcombine(&t);
            assert_eq!(h, x);
        }
    }
}
