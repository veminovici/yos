use crate::{Bit, Bitstring, BitstringDebug};

/// Implementation of the bitstring trait for the u8.
/// # Examples
///
/// ```
/// use yos::{Bitstring, BitstringDebug};
///
/// let mut v = 5u64;
/// v.brst(0);
/// assert_eq!(4, v);
///
/// println!("v={}", v.bdebug());
/// ```
impl Bitstring for u64 {
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
        64
    }

    fn bneg(&mut self) {
        *self = !*self
    }

    fn bone() -> Self {
        1
    }

    fn bone_low(len: usize) -> Self {
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

    fn bxor(&mut self, other: &Self) {
        *self ^= *other
    }

    fn bzero() -> Self {
        0
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
/// use yos::{Bitstring, BitstringDebug};
///
/// let mut v = 5u64;
/// v.brst(0);
/// assert_eq!(4, v);
///
/// println!("v={}", v.bdebug());
/// ```
impl BitstringDebug for u64 {
    fn bdebug(&self) -> String {
        let ueights = self
            .bueights()
            .iter()
            .map(u8_debug)
            .collect::<Vec<String>>();
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
    fn test_blen() {
        let a = 5u64;
        assert_eq!(a.blen(), 64);
    }

    #[test]
    fn test_bor() {
        let mut a = 5u64;
        let b = 2u64;
        a.bor(&b);
        assert_eq!(a, 7);
    }

    #[test]
    fn test_band() {
        let mut a = 5u64;
        let b = 3u64;
        a.band(&b);
        assert_eq!(a, 1);
    }

    #[test]
    fn test_bneg() {
        let mut a = 5u64;
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
        let mut x = 1u64;
        x.blshift(2);
        assert_eq!(x, 4);
    }

    #[test]
    fn test_bpow2() {
        assert_eq!(u64::bpow2(0), 1);
        assert_eq!(u64::bpow2(1), 2);
        assert_eq!(u64::bpow2(2), 4);
        assert_eq!(u64::bpow2(3), 8);
    }

    #[test]
    fn test_brst() {
        let mut v = 7u64;
        v.brst(1);
        assert_eq!(v, 5);
    }

    #[test]
    fn test_bset() {
        let mut v = 5u64;
        v.bset(1);
        assert_eq!(v, 7);
    }

    #[test]
    fn test_bflip() {
        let mut v = 7u64;
        v.bflip(1);
        assert_eq!(v, 5);

        v.bflip(1);
        assert_eq!(v, 7);
    }

    #[test]
    fn test_bget() {
        let v = 5u64;
        let b = v.bget(1);
        assert_eq!(b, Bit::Zero);

        let b = v.bget(2);
        assert_eq!(b, Bit::One);

        assert_eq!(v, 5);
    }

    #[test]
    fn test_brst_low() {
        let mut v = 5u64;
        v.brst_low(2);
        assert_eq!(v, 4);
    }

    #[test]
    fn test_brst_high() {
        let mut v = 5u64;
        v.brst_high(62);
        assert_eq!(v, 1);
    }

    #[quickcheck]
    fn prop_bueights(val: u64) -> bool {
        let ueights = val.bueights();
        let u0 = (val & 255) as u8;
        (ueights.len() == 8) && (ueights[0] == u0)
    }

    #[test]
    fn test_bzero() {
        assert_eq!(u64::bzero(), 0);
    }

    #[test]
    fn test_bone() {
        assert_eq!(u64::bone(), 1);
    }

    #[test]
    fn test_bone_low() {
        assert_eq!(u64::bone_low(0), 0);
        assert_eq!(u64::bone_low(1), 1);
        assert_eq!(u64::bone_low(2), 3);
        assert_eq!(u64::bone_low(3), 7);
        assert_eq!(u64::bone_low(4), 15);
        assert_eq!(u64::bone_low(5), 31);
        assert_eq!(u64::bone_low(6), 63);
        assert_eq!(u64::bone_low(7), 127);
        assert_eq!(u64::bone_low(8), 255);
    }

    #[test]
    fn test_bone_high() {
        assert_eq!(u64::bone_high(0), 0);
        assert_eq!(u64::bone_high(63), 18446744073709551614);
        assert_eq!(u64::bone_high(64), 18446744073709551615);
    }

    #[test]
    fn test_bsplit() {
        let x = 56u64;

        for i in 0..65 {
            let (h, t) = x.bsplit(i);
            assert_eq!(h + t, x);
        }
    }

    #[test]
    fn test_bcombine() {
        let x = 56u64;

        for i in 0..65 {
            let (mut h, t) = x.bsplit(i);
            assert_eq!(h + t, x);
            h.bcombine(&t);
            assert_eq!(h, x);
        }
    }
}
