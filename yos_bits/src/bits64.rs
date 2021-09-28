use super::bit::Bit;
use super::Bitstring;

const LOW_ONES: [u64; 65] = [
    0b0000000000000000000000000000000000000000000000000000000000000000,
    0b0000000000000000000000000000000000000000000000000000000000000001,
    0b0000000000000000000000000000000000000000000000000000000000000011,
    0b0000000000000000000000000000000000000000000000000000000000000111,
    0b0000000000000000000000000000000000000000000000000000000000001111,
    0b0000000000000000000000000000000000000000000000000000000000011111,
    0b0000000000000000000000000000000000000000000000000000000000111111,
    0b0000000000000000000000000000000000000000000000000000000001111111,
    0b0000000000000000000000000000000000000000000000000000000011111111,
    0b0000000000000000000000000000000000000000000000000000000111111111,
    0b0000000000000000000000000000000000000000000000000000001111111111,
    0b0000000000000000000000000000000000000000000000000000011111111111,
    0b0000000000000000000000000000000000000000000000000000111111111111,
    0b0000000000000000000000000000000000000000000000000001111111111111,
    0b0000000000000000000000000000000000000000000000000011111111111111,
    0b0000000000000000000000000000000000000000000000000111111111111111,
    0b0000000000000000000000000000000000000000000000001111111111111111,
    0b0000000000000000000000000000000000000000000000011111111111111111,
    0b0000000000000000000000000000000000000000000000111111111111111111,
    0b0000000000000000000000000000000000000000000001111111111111111111,
    0b0000000000000000000000000000000000000000000011111111111111111111,
    0b0000000000000000000000000000000000000000000111111111111111111111,
    0b0000000000000000000000000000000000000000001111111111111111111111,
    0b0000000000000000000000000000000000000000011111111111111111111111,
    0b0000000000000000000000000000000000000000111111111111111111111111,
    0b0000000000000000000000000000000000000001111111111111111111111111,
    0b0000000000000000000000000000000000000011111111111111111111111111,
    0b0000000000000000000000000000000000000111111111111111111111111111,
    0b0000000000000000000000000000000000001111111111111111111111111111,
    0b0000000000000000000000000000000000011111111111111111111111111111,
    0b0000000000000000000000000000000000111111111111111111111111111111,
    0b0000000000000000000000000000000001111111111111111111111111111111,
    0b0000000000000000000000000000000011111111111111111111111111111111,
    0b0000000000000000000000000000000111111111111111111111111111111111,
    0b0000000000000000000000000000001111111111111111111111111111111111,
    0b0000000000000000000000000000011111111111111111111111111111111111,
    0b0000000000000000000000000000111111111111111111111111111111111111,
    0b0000000000000000000000000001111111111111111111111111111111111111,
    0b0000000000000000000000000011111111111111111111111111111111111111,
    0b0000000000000000000000000111111111111111111111111111111111111111,
    0b0000000000000000000000001111111111111111111111111111111111111111,
    0b0000000000000000000000011111111111111111111111111111111111111111,
    0b0000000000000000000000111111111111111111111111111111111111111111,
    0b0000000000000000000001111111111111111111111111111111111111111111,
    0b0000000000000000000011111111111111111111111111111111111111111111,
    0b0000000000000000000111111111111111111111111111111111111111111111,
    0b0000000000000000001111111111111111111111111111111111111111111111,
    0b0000000000000000011111111111111111111111111111111111111111111111,
    0b0000000000000000111111111111111111111111111111111111111111111111,
    0b0000000000000001111111111111111111111111111111111111111111111111,
    0b0000000000000011111111111111111111111111111111111111111111111111,
    0b0000000000000111111111111111111111111111111111111111111111111111,
    0b0000000000001111111111111111111111111111111111111111111111111111,
    0b0000000000011111111111111111111111111111111111111111111111111111,
    0b0000000000111111111111111111111111111111111111111111111111111111,
    0b0000000001111111111111111111111111111111111111111111111111111111,
    0b0000000011111111111111111111111111111111111111111111111111111111,
    0b0000000111111111111111111111111111111111111111111111111111111111,
    0b0000001111111111111111111111111111111111111111111111111111111111,
    0b0000011111111111111111111111111111111111111111111111111111111111,
    0b0000111111111111111111111111111111111111111111111111111111111111,
    0b0001111111111111111111111111111111111111111111111111111111111111,
    0b0011111111111111111111111111111111111111111111111111111111111111,
    0b0111111111111111111111111111111111111111111111111111111111111111,
    0b1111111111111111111111111111111111111111111111111111111111111111,
];

const HIGH_ONES: [u64; 65] = [
    0b0000000000000000000000000000000000000000000000000000000000000000,
    0b1000000000000000000000000000000000000000000000000000000000000000,
    0b1100000000000000000000000000000000000000000000000000000000000000,
    0b1110000000000000000000000000000000000000000000000000000000000000,
    0b1111000000000000000000000000000000000000000000000000000000000000,
    0b1111100000000000000000000000000000000000000000000000000000000000,
    0b1111110000000000000000000000000000000000000000000000000000000000,
    0b1111111000000000000000000000000000000000000000000000000000000000,
    0b1111111100000000000000000000000000000000000000000000000000000000,
    0b1111111110000000000000000000000000000000000000000000000000000000,
    0b1111111111000000000000000000000000000000000000000000000000000000,
    0b1111111111100000000000000000000000000000000000000000000000000000,
    0b1111111111110000000000000000000000000000000000000000000000000000,
    0b1111111111111000000000000000000000000000000000000000000000000000,
    0b1111111111111100000000000000000000000000000000000000000000000000,
    0b1111111111111110000000000000000000000000000000000000000000000000,
    0b1111111111111111000000000000000000000000000000000000000000000000,
    0b1111111111111111100000000000000000000000000000000000000000000000,
    0b1111111111111111110000000000000000000000000000000000000000000000,
    0b1111111111111111111000000000000000000000000000000000000000000000,
    0b1111111111111111111100000000000000000000000000000000000000000000,
    0b1111111111111111111110000000000000000000000000000000000000000000,
    0b1111111111111111111111000000000000000000000000000000000000000000,
    0b1111111111111111111111100000000000000000000000000000000000000000,
    0b1111111111111111111111110000000000000000000000000000000000000000,
    0b1111111111111111111111111000000000000000000000000000000000000000,
    0b1111111111111111111111111100000000000000000000000000000000000000,
    0b1111111111111111111111111110000000000000000000000000000000000000,
    0b1111111111111111111111111111000000000000000000000000000000000000,
    0b1111111111111111111111111111100000000000000000000000000000000000,
    0b1111111111111111111111111111110000000000000000000000000000000000,
    0b1111111111111111111111111111111000000000000000000000000000000000,
    0b1111111111111111111111111111111100000000000000000000000000000000,
    0b1111111111111111111111111111111110000000000000000000000000000000,
    0b1111111111111111111111111111111111000000000000000000000000000000,
    0b1111111111111111111111111111111111100000000000000000000000000000,
    0b1111111111111111111111111111111111110000000000000000000000000000,
    0b1111111111111111111111111111111111111000000000000000000000000000,
    0b1111111111111111111111111111111111111100000000000000000000000000,
    0b1111111111111111111111111111111111111110000000000000000000000000,
    0b1111111111111111111111111111111111111111000000000000000000000000,
    0b1111111111111111111111111111111111111111100000000000000000000000,
    0b1111111111111111111111111111111111111111110000000000000000000000,
    0b1111111111111111111111111111111111111111111000000000000000000000,
    0b1111111111111111111111111111111111111111111100000000000000000000,
    0b1111111111111111111111111111111111111111111110000000000000000000,
    0b1111111111111111111111111111111111111111111111000000000000000000,
    0b1111111111111111111111111111111111111111111111100000000000000000,
    0b1111111111111111111111111111111111111111111111110000000000000000,
    0b1111111111111111111111111111111111111111111111111000000000000000,
    0b1111111111111111111111111111111111111111111111111100000000000000,
    0b1111111111111111111111111111111111111111111111111110000000000000,
    0b1111111111111111111111111111111111111111111111111111000000000000,
    0b1111111111111111111111111111111111111111111111111111100000000000,
    0b1111111111111111111111111111111111111111111111111111110000000000,
    0b1111111111111111111111111111111111111111111111111111111000000000,
    0b1111111111111111111111111111111111111111111111111111111100000000,
    0b1111111111111111111111111111111111111111111111111111111110000000,
    0b1111111111111111111111111111111111111111111111111111111111000000,
    0b1111111111111111111111111111111111111111111111111111111111100000,
    0b1111111111111111111111111111111111111111111111111111111111110000,
    0b1111111111111111111111111111111111111111111111111111111111111000,
    0b1111111111111111111111111111111111111111111111111111111111111100,
    0b1111111111111111111111111111111111111111111111111111111111111110,
    0b1111111111111111111111111111111111111111111111111111111111111111,
];

mod bits {
    /// Computes the bitwise representation of a power of 2
    #[inline(always)]
    pub(crate) fn pow2(p: usize) -> u64 {
        1 << p
    }

    /// Reset a given bit
    #[inline(always)]
    pub(crate) fn rst(x: u64, ndx: usize) -> u64 {
        x & !pow2(ndx)
    }

    /// Set to 1 a given bit
    #[inline(always)]
    pub(crate) fn set(x: u64, ndx: usize) -> u64 {
        rst(x, ndx) | pow2(ndx)
    }

    /// Get the bit value
    #[inline(always)]
    pub(crate) fn get(x: u64, ndx: usize) -> u64 {
        x & pow2(ndx)
    }

    /// The unit test for the bitwise operations
    #[cfg(test)]
    mod utests {
        use super::*;

        #[test]
        fn test_pow2() {
            assert_eq!(pow2(0), 1);
            assert_eq!(pow2(1), 2);
            assert_eq!(pow2(2), 4);
            assert_eq!(pow2(3), 8);
            assert_eq!(pow2(4), 16);
            assert_eq!(pow2(5), 32);
            assert_eq!(pow2(6), 64);
            assert_eq!(pow2(7), 128);
        }

        #[test]
        fn test_rst() {
            let y = rst(3, 0);
            assert_eq!(y, 2);
        }

        #[test]
        fn test_set() {
            let y = set(2, 0);
            assert_eq!(y, 3);
        }

        #[test]
        fn test_get() {
            let x = 3u64;
            assert_eq!(get(x, 0), 1);
            assert_eq!(get(x, 1), 2);
            assert_eq!(get(x, 2), 0);
        }
    }
}

/// A bit-string repsented on an u64 value.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Bits64(u64);

/// Implementation of fmt traits
pub mod formatting {
    use crate::bits64::*;
    use std::fmt::{Debug, Display};

    fn u8_debug(u: &u8) -> String {
        let str = format!("{:#010b}", u);
        str.strip_prefix("0b").unwrap().to_string()
    }

    impl Display for Bits64 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let xs: Vec<u8> = (*self).into();
            let us = xs.iter().map(u8_debug).collect::<Vec<String>>();
            write!(
                f,
                "Bits64:{:020}|{}|{}|{}|{}|{}|{}|{}|{}|",
                self.0, us[7], us[6], us[5], us[4], us[3], us[2], us[1], us[0]
            )
        }
    }

    impl Debug for Bits64 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let xs: Vec<u8> = (*self).into();
            let us = xs.iter().map(u8_debug).collect::<Vec<String>>();
            write!(
                f,
                "Bits64:{:020}|{}|{}|{}|{}|{}|{}|{}|{}|",
                self.0, us[7], us[6], us[5], us[4], us[3], us[2], us[1], us[0]
            )
        }
    }

    #[cfg(test)]
    mod ptests {
        use crate::bits64::*;
        use quickcheck_macros::quickcheck;

        #[quickcheck]
        fn prop_display(x: Bits64) -> bool {
            let s = format!("{}", x);
            !s.is_empty()
        }

        #[quickcheck]
        fn prop_debug(x: Bits64) -> bool {
            let s = format!("{:?}", x);
            !s.is_empty()
        }
    }
}

pub mod conversions {
    use super::*;

    impl From<u64> for Bits64 {
        fn from(x: u64) -> Self {
            Bits64(x)
        }
    }

    impl From<Bits64> for u64 {
        fn from(x: Bits64) -> Self {
            x.0
        }
    }

    impl From<Bits64> for Vec<u8> {
        fn from(x: Bits64) -> Self {
            let msk = 255u64;

            let u0 = (x.0 & msk) as u8;
            let u1 = ((x.0 & (msk << 8)) >> 8) as u8;
            let u2 = ((x.0 & (msk << 16)) >> 16) as u8;
            let u3 = ((x.0 & (msk << 24)) >> 24) as u8;
            let u4 = ((x.0 & (msk << 32)) >> 32) as u8;
            let u5 = ((x.0 & (msk << 40)) >> 40) as u8;
            let u6 = ((x.0 & (msk << 48)) >> 48) as u8;
            let u7 = ((x.0 & (msk << 56)) >> 56) as u8;

            vec![u0, u1, u2, u3, u4, u5, u6, u7]
        }
    }

    impl From<Vec<u8>> for Bits64 {
        fn from(xs: Vec<u8>) -> Self {
            let u0 = xs[0] as u64;
            let u1 = (xs[1] as u64) << 8;
            let u2 = (xs[2] as u64) << 16;
            let u3 = (xs[3] as u64) << 24;
            let u4 = (xs[4] as u64) << 32;
            let u5 = (xs[5] as u64) << 40;
            let u6 = (xs[6] as u64) << 48;
            let u7 = (xs[7] as u64) << 56;
            Bits64(u0 + u1 + u2 + u3 + u4 + u5 + u6 + u7)
        }
    }

    impl From<&[u8]> for Bits64 {
        fn from(xs: &[u8]) -> Self {
            let u0 = xs[0] as u64;
            let u1 = (xs[1] as u64) << 8;
            let u2 = (xs[2] as u64) << 16;
            let u3 = (xs[3] as u64) << 24;
            let u4 = (xs[4] as u64) << 32;
            let u5 = (xs[5] as u64) << 40;
            let u6 = (xs[6] as u64) << 48;
            let u7 = (xs[7] as u64) << 56;
            Bits64(u0 + u1 + u2 + u3 + u4 + u5 + u6 + u7)
        }
    }

    #[cfg(test)]
    mod ptests {
        use super::*;
        use quickcheck_macros::quickcheck;

        #[quickcheck]
        fn prop_from_to_u8(x: u64) -> bool {
            let bstr = Bits64::from(x);
            let y: u64 = bstr.into();
            bstr.0 == x && x == y
        }

        #[quickcheck]
        fn prop_form_to_vecu8(x: u8) -> bool {
            let bstr = Bits64::from(vec![x; 8]);
            let y: Vec<u8> = bstr.into();
            (bstr.0 as u8) == x && y.len() == 8 && x == y[0]
        }

        #[quickcheck]
        fn prop_from_slice_u8(x: u8) -> bool {
            let xs = &*(vec![x; 8]);
            let bstr = Bits64::from(xs);
            let y: Vec<u8> = bstr.into();
            (bstr.0 as u8) == x && y.len() == 8 && x == y[0]
        }
    }
}

pub mod bitwise {
    use super::*;
    use std::ops::{
        BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
        ShrAssign,
    };

    impl BitAnd for Bits64 {
        type Output = Bits64;

        fn bitand(self, rhs: Self) -> Self::Output {
            Bits64(self.0 & rhs.0)
        }
    }

    impl BitAndAssign for Bits64 {
        fn bitand_assign(&mut self, rhs: Self) {
            self.0 = self.0 & rhs.0;
        }
    }

    impl BitOr for Bits64 {
        type Output = Bits64;

        fn bitor(self, rhs: Self) -> Self::Output {
            Bits64(self.0 | rhs.0)
        }
    }

    impl BitOrAssign for Bits64 {
        fn bitor_assign(&mut self, rhs: Self) {
            self.0 = self.0 | rhs.0;
        }
    }

    impl BitXor for Bits64 {
        type Output = Bits64;

        fn bitxor(self, rhs: Self) -> Self::Output {
            Bits64(self.0 ^ rhs.0)
        }
    }

    impl BitXorAssign for Bits64 {
        fn bitxor_assign(&mut self, rhs: Self) {
            self.0 = self.0 ^ rhs.0;
        }
    }

    impl Not for Bits64 {
        type Output = Bits64;

        fn not(self) -> Self::Output {
            Bits64::from(!self.0)
        }
    }

    impl Shr<usize> for Bits64 {
        type Output = Bits64;

        fn shr(self, rhs: usize) -> Self::Output {
            Bits64(self.0 >> rhs)
        }
    }

    impl ShrAssign<usize> for Bits64 {
        fn shr_assign(&mut self, rhs: usize) {
            self.0 >>= rhs;
        }
    }

    impl Shl<usize> for Bits64 {
        type Output = Bits64;

        fn shl(self, rhs: usize) -> Self::Output {
            Bits64(self.0 << rhs)
        }
    }

    impl ShlAssign<usize> for Bits64 {
        fn shl_assign(&mut self, rhs: usize) {
            self.0 <<= rhs;
        }
    }

    #[cfg(test)]
    mod utests {
        use crate::bits64::*;

        #[test]
        fn test_and() {
            let x = Bits64::from(5);
            let y = x & Bits64::from(3);
            assert_eq!(y.0, 1);
        }

        #[test]
        fn test_and_assign() {
            let mut x = Bits64::from(5);
            x &= Bits64::from(3);
            assert_eq!(x.0, 1);
        }

        #[test]
        fn test_or() {
            let x = Bits64::from(5);
            let y = x | Bits64::from(3);
            assert_eq!(y.0, 7);
        }

        #[test]
        fn test_or_assign() {
            let mut x = Bits64::from(5);
            x |= Bits64::from(3);
            assert_eq!(x.0, 7);
        }

        #[test]
        fn test_xor() {
            let x = Bits64::from(5);
            let y = x ^ Bits64::from(3);
            assert_eq!(y.0, 6);
        }

        #[test]
        fn test_xor_assign() {
            let mut x = Bits64::from(5);
            x ^= Bits64::from(3);
            assert_eq!(x.0, 6);
        }

        #[test]
        fn test_not_assign() {
            let x = Bits64::from(LOW_ONES[1]);
            let y = !x;
            assert_eq!(y.0, HIGH_ONES[63])
        }

        #[test]
        fn test_shr() {
            let x = Bits64::from(5);
            let y = x >> 1;
            assert_eq!(y.0, 2);
        }

        #[test]
        fn test_shr_assign() {
            let mut x = Bits64::from(5);
            x >>= 1;
            assert_eq!(x.0, 2);
        }

        #[test]
        fn test_shl() {
            let x = Bits64::from(3);
            let y = x << 1;
            assert_eq!(y.0, 6);
        }

        #[test]
        fn test_shl_assign() {
            let mut x = Bits64::from(3);
            x <<= 1;
            assert_eq!(x.0, 6);
        }
    }
}

pub mod constructors {
    use super::*;

    impl Bits64 {
        /// Build a zero value
        pub fn zero() -> Bits64 {
            Bits64(0)
        }

        /// Builds a one value
        pub fn one() -> Bits64 {
            Bits64(1)
        }

        /// Builds a bitstring where all bits are set.
        pub fn ones() -> Bits64 {
            Bits64::from(LOW_ONES[64])
        }

        /// Build a power of 2 value
        pub fn pow2(power: usize) -> Bits64 {
            Bits64(bits::pow2(power))
        }

        /// Build a vlaue with all lower bits set to 1
        pub fn low_ones(len: usize) -> Bits64 {
            Bits64(LOW_ONES[len])
        }

        /// Build a value with all higher bits set to 1
        pub fn high_ones(len: usize) -> Bits64 {
            Bits64(HIGH_ONES[len])
        }

        /// Builds a bit-string which has set on 1 the bits
        /// within a range starting at a given position with
        /// a given length.
        pub fn range_ones(pos: usize, len: usize) -> Bits64 {
            let mut mask = Bits64::low_ones(len);
            mask <<= pos;
            mask
        }

        /// Builds a bit-string which has set on 0 the bts
        /// within a range starting at a given position with
        /// a given length.
        pub fn range_zeros(pos: usize, len: usize) -> Bits64 {
            let mut mask = Bits64::zero();
            if pos > 0 {
                mask = Bits64::low_ones(pos);
            }

            mask |= Bits64::high_ones(64 - pos - len);
            mask
        }
    }

    #[cfg(test)]
    mod utests {
        use crate::bits64::*;

        #[test]
        fn test_zero() {
            assert_eq!(Bits64::zero().0, 0);
        }

        #[test]
        fn test_one() {
            assert_eq!(Bits64::one().0, 1);
        }

        #[test]
        fn test_ones() {
            assert_eq!(Bits64::ones().0, LOW_ONES[64]);
        }

        #[test]
        fn test_pow2() {
            assert_eq!(Bits64::pow2(2).0, 4);
        }

        #[test]
        fn test_low_ones() {
            assert_eq!(Bits64::low_ones(2).0, 3);
        }

        #[test]
        fn test_high_ones() {
            assert_eq!(Bits64::high_ones(63).0, HIGH_ONES[63]);
        }

        #[test]
        fn test_range_ones() {
            assert_eq!(Bits64::range_ones(0, 0).0, 0);
            assert_eq!(Bits64::range_ones(2, 1).0, 4);
            assert_eq!(Bits64::range_ones(2, 2).0, 12);
            assert_eq!(Bits64::range_ones(0, 8).0, 255);
        }

        #[test]
        fn test_range_zeros() {
            assert_eq!(Bits64::range_zeros(0, 64).0, 0);
            assert_eq!(Bits64::range_zeros(0, 0).0, LOW_ONES[64]);
            assert_eq!(Bits64::range_zeros(1, 62).0, 2u64.pow(63) + 1);
        }
    }
}

pub mod bitstring {
    use super::*;

    impl Bitstring for Bits64 {
        fn len() -> usize {
            64
        }

        fn get(&self, ndx: usize) -> Bit {
            if bits::get(self.0, ndx) == 0 {
                Bit::Zero
            } else {
                Bit::One
            }
        }

        /// Set the bit value at a given index
        fn set(&mut self, ndx: usize) {
            self.0 = bits::set(self.0, ndx);
        }

        /// Reset the bit value at a given index
        fn rst(&mut self, ndx: usize) {
            self.0 = bits::rst(self.0, ndx);
        }
    }

    #[cfg(test)]
    mod utests {
        use crate::bits64::*;

        #[test]
        fn test_len() {
            assert_eq!(Bits64::len(), 64);
        }

        #[test]
        fn test_get() {
            let bstr = Bits64::from(5);
            assert_eq!(bstr.get(0), Bit::One);
            assert_eq!(bstr.get(1), Bit::Zero);
            assert_eq!(bstr.get(2), Bit::One);
        }

        #[test]
        fn test_set() {
            let mut bstr = Bits64::from(5);
            bstr.set(1);
            assert_eq!(bstr.0, 7);
        }

        #[test]
        fn test_rst() {
            let mut bstr = Bits64::from(5);
            bstr.rst(2);
            assert_eq!(bstr.0, 1);
        }
    }
}

pub mod range {
    use super::*;

    impl Bits64 {
        /// Reset the low bits
        pub fn rst_low(&mut self, len: usize) {
            self.0 &= HIGH_ONES[64 - len]
        }

        /// Reset the high bits
        pub fn rst_high(&mut self, len: usize) {
            self.0 &= LOW_ONES[64 - len]
        }

        /// Set the low bits
        pub fn set_low(&mut self, len: usize) {
            self.0 |= LOW_ONES[len]
        }

        /// Set tehe high bits
        pub fn set_high(&mut self, len: usize) {
            self.0 |= HIGH_ONES[len]
        }
    }

    #[cfg(test)]
    mod utests {
        use crate::bits64::*;

        #[test]
        fn test_rst_low() {
            let mut x = Bits64::from(5);
            x.rst_low(1);
            assert_eq!(x.0, 4);
        }

        #[test]
        fn test_rst_high() {
            let mut x = Bits64::from(5);
            x.rst_high(62);
            assert_eq!(x.0, 1);
        }

        #[test]
        fn test_set_low() {
            let mut x = Bits64::from(5);
            x.set_low(2);
            assert_eq!(x.0, 7);
        }

        #[test]
        fn test_set_high() {
            let mut x = Bits64::from(6);
            x.set_high(63);
            assert_eq!(x.0, HIGH_ONES[63]);
        }
    }
}

pub mod combinators {
    use super::*;

    impl Bits64 {
        /// Splits a bitstring
        pub fn split(&self, pos: usize) -> (Bits64, Bits64) {
            let l = self.0 & LOW_ONES[pos];
            let h = self.0 & HIGH_ONES[64 - pos];
            (Bits64::from(l), Bits64::from(h))
        }

        /// Flips a bit value
        pub fn flip(&mut self, pos: usize) {
            *self ^= Bits64::pow2(pos)
        }

        /// Combines two bit strings.
        pub fn combine(&mut self, other: &Bits64) {
            *self |= *other;
        }
    }

    #[cfg(test)]
    mod utests {
        use crate::bits64::*;

        #[test]
        fn test_combinators_split() {
            let x = Bits64::from(56u64);

            for i in 0..65 {
                let (h, t) = x.split(i);
                assert_eq!(h.0 + t.0, x.0);
            }
        }

        #[test]
        fn test_combinators_flip() {
            let mut x = Bits64::from(6u64);
            x.flip(1);
            assert_eq!(x.0, 4);

            x.flip(1);
            assert_eq!(x.0, 6);
        }

        #[test]
        fn test_combinators_combine() {
            let x = Bits64::from(56u64);

            for i in 0..65 {
                let (mut h, t) = x.split(i);
                assert_eq!(h.0 + t.0, x.0);
                h.combine(&t);
                assert_eq!(h, x);
            }
        }
    }
}

pub mod iterator {
    use super::*;
    use std::iter::FromIterator;

    pub struct IterBits64 {
        bits: Bits64,
        ndx: usize,
    }

    impl IterBits64 {
        /// Builds a new instance of the Bits8 iterator
        pub fn new(bits: Bits64) -> Self {
            Self { bits, ndx: 0 }
        }
    }

    impl Iterator for IterBits64 {
        type Item = Bit;

        fn next(&mut self) -> Option<Self::Item> {
            if self.ndx == Bits64::len() {
                None
            } else {
                let v = self.bits.get(self.ndx);
                self.ndx += 1;
                Some(v)
            }
        }
    }

    impl IntoIterator for Bits64 {
        type Item = Bit;
        type IntoIter = IterBits64;

        fn into_iter(self) -> Self::IntoIter {
            IterBits64::new(self)
        }
    }

    impl FromIterator<Bit> for Bits64 {
        fn from_iter<T: IntoIterator<Item = Bit>>(iter: T) -> Self {
            let mut us: Vec<u8> = Vec::with_capacity(64);
            for b in iter {
                us.push(b.into())
            }

            Self::from(us)
        }
    }

    #[cfg(test)]
    mod utests {
        use super::IterBits64;
        use crate::bits64::*;
        use std::iter::FromIterator;

        #[test]
        fn test_iter() {
            let bits = Bits64::one();
            let mut iter = IterBits64::new(bits);

            assert_eq!(Some(Bit::One), iter.next());
            assert_eq!(Some(Bit::Zero), iter.next());
            assert_eq!(Some(Bit::Zero), iter.next());
            assert_eq!(Some(Bit::Zero), iter.next());
            assert_eq!(Some(Bit::Zero), iter.next());
            assert_eq!(Some(Bit::Zero), iter.next());
            assert_eq!(Some(Bit::Zero), iter.next());
            assert_eq!(Some(Bit::Zero), iter.next());
        }

        #[test]
        fn test_into_iter() {
            let bits = Bits64::one();
            let mut iter = bits.into_iter();

            assert_eq!(Some(Bit::One), iter.next());
            assert_eq!(Some(Bit::Zero), iter.next());
            assert_eq!(Some(Bit::Zero), iter.next());
            assert_eq!(Some(Bit::Zero), iter.next());
            assert_eq!(Some(Bit::Zero), iter.next());
            assert_eq!(Some(Bit::Zero), iter.next());
            assert_eq!(Some(Bit::Zero), iter.next());
            assert_eq!(Some(Bit::Zero), iter.next());
        }

        #[test]
        fn test_enumerate() {
            let bits = Bits64::one();
            for (ndx, b) in bits.into_iter().enumerate() {
                if ndx == 0 {
                    assert_eq!(Bit::One, b);
                } else {
                    assert_eq!(Bit::Zero, b);
                }
            }
        }

        #[test]
        fn test_from_iter() {
            let bits = Bits64::one();
            let bits1 = Bits64::from_iter(bits.into_iter());
            assert_eq!(bits, bits1);
        }
    }
}

#[cfg(test)]
mod arbitrary {
    use super::*;

    use quickcheck::{Arbitrary, Gen};

    impl Arbitrary for Bits64 {
        fn arbitrary(g: &mut Gen) -> Self {
            Bits64::from(u64::arbitrary(g))
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            let mut shrunk_vals = Vec::new();
            if self.0 > 0 {
                shrunk_vals.push(Bits64::from(self.0 - 1));
            }
            Box::new(shrunk_vals.into_iter())
        }
    }

    #[cfg(test)]
    mod utests {
        use super::*;

        #[test]
        fn test_shrink() {
            let x = Bits64::from(2);
            let mut xs = x.shrink();
            assert_eq!((*xs).next().unwrap(), Bits64::from(1));
        }
    }
}
