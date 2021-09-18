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
pub struct Bu64(u64);

/// Implementation of fmt traits
pub mod formatting {
    use crate::bu64::*;
    use std::fmt::{Debug, Display};

    fn u8_debug(u: &u8) -> String {
        let str = format!("{:#010b}", u);
        str.strip_prefix("0b").unwrap().to_string()
    }

    impl Display for Bu64 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let xs: Vec<u8> = (*self).into();
            let us = xs.iter().map(u8_debug).collect::<Vec<String>>();
            write!(
                f,
                "Bu64:{:020}|{}|{}|{}|{}|{}|{}|{}|{}|",
                self.0, us[7], us[6], us[5], us[4], us[3], us[2], us[1], us[0]
            )
        }
    }

    impl Debug for Bu64 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let xs: Vec<u8> = (*self).into();
            let us = xs.iter().map(u8_debug).collect::<Vec<String>>();
            write!(
                f,
                "Bu64:{:020}|{}|{}|{}|{}|{}|{}|{}|{}|",
                self.0, us[7], us[6], us[5], us[4], us[3], us[2], us[1], us[0]
            )
        }
    }

    #[cfg(test)]
    mod ptests {
        use crate::bu64::*;
        use quickcheck_macros::quickcheck;

        #[quickcheck]
        fn prop_display(x: Bu64) -> bool {
            let s = format!("{}", x);
            !s.is_empty()
        }

        #[quickcheck]
        fn prop_debug(x: Bu64) -> bool {
            let s = format!("{:?}", x);
            !s.is_empty()
        }
    }
}

pub mod conversions {
    use super::*;

    impl From<u64> for Bu64 {
        fn from(x: u64) -> Self {
            Bu64(x)
        }
    }

    impl From<Bu64> for u64 {
        fn from(x: Bu64) -> Self {
            x.0
        }
    }

    impl From<Bu64> for Vec<u8> {
        fn from(x: Bu64) -> Self {
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

    impl From<Vec<u8>> for Bu64 {
        fn from(xs: Vec<u8>) -> Self {
            let u0 = xs[0] as u64;
            let u1 = (xs[1] as u64) << 8;
            let u2 = (xs[2] as u64) << 16;
            let u3 = (xs[3] as u64) << 24;
            let u4 = (xs[4] as u64) << 32;
            let u5 = (xs[5] as u64) << 40;
            let u6 = (xs[6] as u64) << 48;
            let u7 = (xs[7] as u64) << 56;
            Bu64(u0 + u1 + u2 + u3 + u4 + u5 + u6 + u7)
        }
    }

    impl From<&[u8]> for Bu64 {
        fn from(xs: &[u8]) -> Self {
            let u0 = xs[0] as u64;
            let u1 = (xs[1] as u64) << 8;
            let u2 = (xs[2] as u64) << 16;
            let u3 = (xs[3] as u64) << 24;
            let u4 = (xs[4] as u64) << 32;
            let u5 = (xs[5] as u64) << 40;
            let u6 = (xs[6] as u64) << 48;
            let u7 = (xs[7] as u64) << 56;
            Bu64(u0 + u1 + u2 + u3 + u4 + u5 + u6 + u7)
        }
    }

    #[cfg(test)]
    mod ptests {
        use super::*;
        use quickcheck_macros::quickcheck;

        #[quickcheck]
        fn prop_from_to_u8(x: u64) -> bool {
            let bstr = Bu64::from(x);
            let y: u64 = bstr.into();
            bstr.0 == x && x == y
        }

        #[quickcheck]
        fn prop_form_to_vecu8(x: u8) -> bool {
            let bstr = Bu64::from(vec![x; 8]);
            let y: Vec<u8> = bstr.into();
            (bstr.0 as u8) == x && y.len() == 8 && x == y[0]
        }

        #[quickcheck]
        fn prop_from_slice_u8(x: u8) -> bool {
            let xs = &*(vec![x; 8]);
            let bstr = Bu64::from(xs);
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

    impl BitAnd for Bu64 {
        type Output = Bu64;

        fn bitand(self, rhs: Self) -> Self::Output {
            Bu64(self.0 & rhs.0)
        }
    }

    impl BitAndAssign for Bu64 {
        fn bitand_assign(&mut self, rhs: Self) {
            self.0 = self.0 & rhs.0;
        }
    }

    impl BitOr for Bu64 {
        type Output = Bu64;

        fn bitor(self, rhs: Self) -> Self::Output {
            Bu64(self.0 | rhs.0)
        }
    }

    impl BitOrAssign for Bu64 {
        fn bitor_assign(&mut self, rhs: Self) {
            self.0 = self.0 | rhs.0;
        }
    }

    impl BitXor for Bu64 {
        type Output = Bu64;

        fn bitxor(self, rhs: Self) -> Self::Output {
            Bu64(self.0 ^ rhs.0)
        }
    }

    impl BitXorAssign for Bu64 {
        fn bitxor_assign(&mut self, rhs: Self) {
            self.0 = self.0 ^ rhs.0;
        }
    }

    impl Not for Bu64 {
        type Output = Bu64;

        fn not(self) -> Self::Output {
            Bu64::from(!self.0)
        }
    }

    impl Shr<usize> for Bu64 {
        type Output = Bu64;

        fn shr(self, rhs: usize) -> Self::Output {
            Bu64(self.0 >> rhs)
        }
    }

    impl ShrAssign<usize> for Bu64 {
        fn shr_assign(&mut self, rhs: usize) {
            self.0 >>= rhs;
        }
    }

    impl Shl<usize> for Bu64 {
        type Output = Bu64;

        fn shl(self, rhs: usize) -> Self::Output {
            Bu64(self.0 << rhs)
        }
    }

    impl ShlAssign<usize> for Bu64 {
        fn shl_assign(&mut self, rhs: usize) {
            self.0 <<= rhs;
        }
    }

    #[cfg(test)]
    mod utests {
        use crate::bu64::*;

        #[test]
        fn prop_and() {
            let x = Bu64::from(5);
            let y = x & Bu64::from(3);
            assert_eq!(y.0, 1);
        }

        #[test]
        fn prop_and_assign() {
            let mut x = Bu64::from(5);
            x &= Bu64::from(3);
            assert_eq!(x.0, 1);
        }

        #[test]
        fn prop_or() {
            let x = Bu64::from(5);
            let y = x | Bu64::from(3);
            assert_eq!(y.0, 7);
        }

        #[test]
        fn prop_or_assign() {
            let mut x = Bu64::from(5);
            x |= Bu64::from(3);
            assert_eq!(x.0, 7);
        }

        #[test]
        fn prop_xor() {
            let x = Bu64::from(5);
            let y = x ^ Bu64::from(3);
            assert_eq!(y.0, 6);
        }

        #[test]
        fn prop_xor_assign() {
            let mut x = Bu64::from(5);
            x ^= Bu64::from(3);
            assert_eq!(x.0, 6);
        }

        #[test]
        fn prop_not_assign() {
            let x = Bu64::from(LOW_ONES[1]);
            let y = !x;
            assert_eq!(y.0, HIGH_ONES[63])
        }

        #[test]
        fn prop_shr() {
            let x = Bu64::from(5);
            let y = x >> 1;
            assert_eq!(y.0, 2);
        }

        #[test]
        fn prop_shr_assign() {
            let mut x = Bu64::from(5);
            x >>= 1;
            assert_eq!(x.0, 2);
        }

        #[test]
        fn prop_shl() {
            let x = Bu64::from(3);
            let y = x << 1;
            assert_eq!(y.0, 6);
        }

        #[test]
        fn prop_shl_assign() {
            let x = Bu64::from(3);
            let y = x << 1;
            assert_eq!(y.0, 6);
        }
    }
}

pub mod constructors {
    use super::*;

    impl Bu64 {
        /// Build a zero value
        pub fn zero() -> Bu64 {
            Bu64(0)
        }

        /// Builds a one value
        pub fn one() -> Bu64 {
            Bu64(1)
        }

        /// Build a power of 2 value
        pub fn pow2(power: usize) -> Bu64 {
            Bu64(bits::pow2(power))
        }

        /// Build a vlaue with all lower bits set to 1
        pub fn low_ones(len: usize) -> Bu64 {
            Bu64(LOW_ONES[len])
        }

        /// Build a value with all higher bits set to 1
        pub fn high_ones(len: usize) -> Bu64 {
            Bu64(HIGH_ONES[len])
        }
    }

    #[cfg(test)]
    mod utests {
        use crate::bu64::*;

        #[test]
        fn test_zero() {
            assert_eq!(Bu64::from(0).0, 0);
        }

        #[test]
        fn test_one() {
            assert_eq!(Bu64::from(1).0, 1);
        }

        #[test]
        fn test_pow2() {
            assert_eq!(Bu64::pow2(2).0, 4);
        }

        #[test]
        fn test_low_ones() {
            assert_eq!(Bu64::low_ones(2).0, 3);
        }

        #[test]
        fn test_high_ones() {
            assert_eq!(Bu64::high_ones(63).0, HIGH_ONES[63]);
        }
    }
}

pub mod bitstring {
    use super::*;

    impl Bitstring for Bu64 {
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
        use crate::bu64::*;

        #[test]
        fn test_len() {
            assert_eq!(Bu64::len(), 64);
        }

        #[test]
        fn test_get() {
            let bstr = Bu64::from(5);
            assert_eq!(bstr.get(0), Bit::One);
            assert_eq!(bstr.get(1), Bit::Zero);
            assert_eq!(bstr.get(2), Bit::One);
        }

        #[test]
        fn test_set() {
            let mut bstr = Bu64::from(5);
            bstr.set(1);
            assert_eq!(bstr.0, 7);
        }

        #[test]
        fn test_rst() {
            let mut bstr = Bu64::from(5);
            bstr.rst(2);
            assert_eq!(bstr.0, 1);
        }
    }
}

pub mod range {
    use super::*;

    impl Bu64 {
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
        use crate::bu64::*;

        #[test]
        fn test_rst_low() {
            let mut x = Bu64::from(5);
            x.rst_low(1);
            assert_eq!(x.0, 4);
        }

        #[test]
        fn test_rst_high() {
            let mut x = Bu64::from(5);
            x.rst_high(62);
            assert_eq!(x.0, 1);
        }

        #[test]
        fn test_set_low() {
            let mut x = Bu64::from(5);
            x.set_low(2);
            assert_eq!(x.0, 7);
        }

        #[test]
        fn test_set_high() {
            let mut x = Bu64::from(6);
            x.set_high(63);
            assert_eq!(x.0, HIGH_ONES[63]);
        }
    }
}

pub mod combinators {
    use super::*;

    impl Bu64 {
        /// Splits a bitstring
        pub fn split(&self, pos: usize) -> (Bu64, Bu64) {
            let l = self.0 & LOW_ONES[pos];
            let h = self.0 & HIGH_ONES[64 - pos];
            (Bu64::from(l), Bu64::from(h))
        }

        /// Flips a bit value
        pub fn flip(&mut self, pos: usize) {
            *self ^= Bu64::pow2(pos)
        }

        /// Combines two bit strings.
        pub fn combine(&mut self, other: &Bu64) {
            *self |= *other;
        }
    }

    #[cfg(test)]
    mod utests {
        use crate::bu64::*;

        #[test]
        fn test_bstr_combinators_split() {
            let x = Bu64::from(56u64);

            for i in 0..65 {
                let (h, t) = x.split(i);
                assert_eq!(h.0 + t.0, x.0);
            }
        }

        #[test]
        fn test_bstr_combinators_flip() {
            let mut x = Bu64::from(6u64);
            x.flip(1);
            assert_eq!(x.0, 4);

            x.flip(1);
            assert_eq!(x.0, 6);
        }

        #[test]
        fn test_bstr_combinators_combine() {
            let x = Bu64::from(56u64);

            for i in 0..65 {
                let (mut h, t) = x.split(i);
                assert_eq!(h.0 + t.0, x.0);
                h.combine(&t);
                assert_eq!(h, x);
            }
        }
    }
}

#[cfg(test)]
mod arbitrary {
    use super::*;

    use quickcheck::{Arbitrary, Gen};

    impl Arbitrary for Bu64 {
        fn arbitrary(g: &mut Gen) -> Self {
            Bu64::from(u64::arbitrary(g))
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            let mut shrunk_vals = Vec::new();
            if self.0 > 0 {
                shrunk_vals.push(Bu64::from(self.0 - 1));
            }
            Box::new(shrunk_vals.into_iter())
        }
    }

    #[cfg(test)]
    mod utests {
        use super::*;

        #[test]
        fn test_shrink() {
            let x = Bu64::from(2);
            let mut xs = x.shrink();
            assert_eq!((*xs).next().unwrap(), Bu64::from(1));
        }
    }
}
