use super::bit::Bit;
use super::Bitstring;

const LOW_ONES: [u8; 9] = [
    0b00000000, 0b00000001, 0b00000011, 0b00000111, 0b00001111, 0b00011111, 0b00111111, 0b01111111,
    0b11111111,
];

const HIGH_ONES: [u8; 9] = [
    0b00000000, 0b10000000, 0b11000000, 0b11100000, 0b11110000, 0b11111000, 0b11111100, 0b11111110,
    0b11111111,
];

/// Different helper function which perform bitwise operations on u8 values.
mod bits {
    /// Computes the bitwise representation of a power of 2
    #[inline(always)]
    pub(crate) fn pow2(p: usize) -> u8 {
        1 << p
    }

    /// Reset a given bit
    #[inline(always)]
    pub(crate) fn rst(x: u8, ndx: usize) -> u8 {
        x & !pow2(ndx)
    }

    /// Set to 1 a given bit
    #[inline(always)]
    pub(crate) fn set(x: u8, ndx: usize) -> u8 {
        rst(x, ndx) | pow2(ndx)
    }

    /// Get the bit value
    #[inline(always)]
    pub(crate) fn get(x: u8, ndx: usize) -> u8 {
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
            let x = 3u8;
            assert_eq!(get(x, 0), 1);
            assert_eq!(get(x, 1), 2);
            assert_eq!(get(x, 2), 0);
        }
    }

    #[cfg(test)]
    mod ptests {
        use super::*;
        use quickcheck_macros::quickcheck;

        #[quickcheck]
        fn prop_pow2(x: u8) -> bool {
            let p = x % 8;
            assert_eq!(pow2(p as usize), 2u8.pow(p as u32) as u8);
            pow2(p as usize) == (2u8.pow(p as u32) as u8)
        }

        #[quickcheck]
        fn prop_rst(x: u8, ndx: usize) -> bool {
            let ndx = ndx % 8;

            let y = rst(x, ndx);
            let z = rst(y, ndx);

            y == z
        }

        #[quickcheck]
        fn prop_set(x: u8, ndx: usize) -> bool {
            let ndx = ndx % 8;

            let y = set(x, ndx);
            let z = set(y, ndx);

            y == z
        }

        #[quickcheck]
        fn prop_get(x: u8, ndx: usize) -> bool {
            let ndx = ndx % 8;

            let y = get(x, ndx);
            let z = get(y, ndx);

            y == z
        }
    }
}

/// A bit-string repsented on an u8 value.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Bu8(u8);

/// Implementation of fmt traits
pub mod formatting {
    use super::Bu8;
    use std::fmt::{Debug, Display};

    impl Display for Bu8 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let s = format!("{:#010b}", self.0);
            write!(f, "Bu8:{:03}|{}|", self.0, s.strip_prefix("0b").unwrap())
        }
    }

    impl Debug for Bu8 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let s = format!("{:#010b}", self.0);
            write!(f, "Bu8:{:03}|{}|", self.0, s.strip_prefix("0b").unwrap())
        }
    }

    #[cfg(test)]
    mod ptests {
        use crate::bu8::*;
        use quickcheck_macros::quickcheck;

        #[quickcheck]
        fn prop_display(x: Bu8) -> bool {
            let s = format!("{}", x);
            s.len() == 17
        }

        #[quickcheck]
        fn prop_debug(x: Bu8) -> bool {
            let s = format!("{:?}", x);
            s.len() == 17
        }

        #[quickcheck]
        fn prop_display_debug(x: u8) -> bool {
            let x = Bu8::from(x);
            let d1 = format!("{}", x);
            let d2 = format!("{:?}", x);
            d1 == d2
        }
    }
}

pub mod conversions {
    use super::*;

    impl From<u8> for Bu8 {
        fn from(x: u8) -> Self {
            Bu8(x)
        }
    }

    impl From<Bu8> for u8 {
        fn from(x: Bu8) -> Self {
            x.0
        }
    }

    impl From<Bu8> for Vec<u8> {
        fn from(x: Bu8) -> Self {
            vec![x.0]
        }
    }

    impl From<Vec<u8>> for Bu8 {
        fn from(xs: Vec<u8>) -> Self {
            Bu8(xs[0])
        }
    }

    impl From<&[u8]> for Bu8 {
        fn from(xs: &[u8]) -> Self {
            Bu8(xs[0])
        }
    }

    #[cfg(test)]
    mod ptests {
        use crate::bu8::*;
        use quickcheck_macros::quickcheck;

        #[quickcheck]
        fn prop_from_to_u8(x: u8) -> bool {
            let bstr = Bu8::from(x);
            let y: u8 = bstr.into();
            bstr.0 == x && x == y
        }

        #[quickcheck]
        fn prop_form_to_vecu8(x: u8) -> bool {
            let bstr = Bu8::from(vec![x]);
            let y: Vec<u8> = bstr.into();
            bstr.0 == x && y.len() == 1 && x == y[0]
        }

        #[quickcheck]
        fn prop_from_slice_u8(x: u8) -> bool {
            let xs = &*vec![x];
            let bstr = Bu8::from(xs);
            let y: Vec<u8> = bstr.into();
            bstr.0 == x && y.len() == 1 && x == y[0]
        }
    }
}

pub mod bitwise {
    use super::*;
    use std::ops::{
        BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
        ShrAssign,
    };

    impl BitAnd for Bu8 {
        type Output = Bu8;

        fn bitand(self, rhs: Self) -> Self::Output {
            Bu8(self.0 & rhs.0)
        }
    }

    impl BitAndAssign for Bu8 {
        fn bitand_assign(&mut self, rhs: Self) {
            self.0 = self.0 & rhs.0;
        }
    }

    impl BitOr for Bu8 {
        type Output = Bu8;

        fn bitor(self, rhs: Self) -> Self::Output {
            Bu8(self.0 | rhs.0)
        }
    }

    impl BitOrAssign for Bu8 {
        fn bitor_assign(&mut self, rhs: Self) {
            self.0 = self.0 | rhs.0;
        }
    }

    impl BitXor for Bu8 {
        type Output = Bu8;

        fn bitxor(self, rhs: Self) -> Self::Output {
            Bu8(self.0 ^ rhs.0)
        }
    }

    impl BitXorAssign for Bu8 {
        fn bitxor_assign(&mut self, rhs: Self) {
            self.0 = self.0 ^ rhs.0;
        }
    }

    impl Not for Bu8 {
        type Output = Bu8;

        fn not(self) -> Self::Output {
            Bu8::from(!self.0)
        }
    }

    impl Shr<usize> for Bu8 {
        type Output = Bu8;

        fn shr(self, rhs: usize) -> Self::Output {
            Bu8(self.0 >> rhs)
        }
    }

    impl ShrAssign<usize> for Bu8 {
        fn shr_assign(&mut self, rhs: usize) {
            self.0 >>= rhs;
        }
    }

    impl Shl<usize> for Bu8 {
        type Output = Bu8;

        fn shl(self, rhs: usize) -> Self::Output {
            Bu8(self.0 << rhs)
        }
    }

    impl ShlAssign<usize> for Bu8 {
        fn shl_assign(&mut self, rhs: usize) {
            self.0 <<= rhs;
        }
    }

    #[cfg(test)]
    mod utests {
        use crate::bu8::*;

        #[test]
        fn test_and() {
            let x = Bu8::from(5);
            let y = x & Bu8::from(3);
            assert_eq!(y.0, 1);
        }

        #[test]
        fn test_and_assign() {
            let mut x = Bu8::from(5);
            x &= Bu8::from(3);
            assert_eq!(x.0, 1);
        }

        #[test]
        fn test_or() {
            let x = Bu8::from(5);
            let y = x | Bu8::from(3);
            assert_eq!(y.0, 7);
        }

        #[test]
        fn test_or_assign() {
            let mut x = Bu8::from(5);
            x |= Bu8::from(3);
            assert_eq!(x.0, 7);
        }

        #[test]
        fn test_xor() {
            let x = Bu8::from(5);
            let y = x ^ Bu8::from(3);
            assert_eq!(y.0, 6);
        }

        #[test]
        fn test_xor_assign() {
            let mut x = Bu8::from(5);
            x ^= Bu8::from(3);
            assert_eq!(x.0, 6);
        }

        #[test]
        fn test_not_assign() {
            let x = Bu8::from(LOW_ONES[1]);
            let y = !x;
            assert_eq!(y.0, HIGH_ONES[7])
        }

        #[test]
        fn test_shr() {
            let x = Bu8::from(5);
            let y = x >> 1;
            assert_eq!(y.0, 2);
        }

        #[test]
        fn test_shr_assign() {
            let mut x = Bu8::from(5);
            x >>= 1;
            assert_eq!(x.0, 2);
        }

        #[test]
        fn test_shl() {
            let x = Bu8::from(3);
            let y = x << 1;
            assert_eq!(y.0, 6);
        }

        #[test]
        fn test_shl_assign() {
            let mut x = Bu8::from(3);
            x <<= 1;
            assert_eq!(x.0, 6);
        }
    }
}

pub mod constructors {
    use super::*;

    impl Bu8 {
        /// Build a zero value
        pub fn zero() -> Bu8 {
            Bu8(0)
        }

        /// Builds a one value
        pub fn one() -> Bu8 {
            Bu8(1)
        }

        /// Build a power of 2 value
        pub fn pow2(power: usize) -> Bu8 {
            Bu8(bits::pow2(power))
        }

        /// Build a vlaue with all lower bits set to 1
        pub fn low_ones(len: usize) -> Bu8 {
            Bu8(LOW_ONES[len])
        }

        /// Build a value with all higher bits set to 1
        pub fn high_ones(len: usize) -> Bu8 {
            Bu8(HIGH_ONES[len])
        }

        /// Builds a bit-string which has set on 1 the bits
        /// within a range starting at a given position with
        /// a given length.
        pub fn range_ones(pos: usize, len: usize) -> Bu8 {
            let mut mask = Bu8::low_ones(len);
            mask <<= pos;
            mask
        }

        /// Builds a bit-string which has set on 0 the bts
        /// within a range starting at a given position with
        /// a given length.
        pub fn range_zeros(pos: usize, len: usize) -> Bu8 {
            let mut mask = Bu8::zero();
            if pos > 0 {
                mask = Bu8::low_ones(pos);
            }

            mask |= Bu8::high_ones(8 - pos - len);
            mask
        }
    }

    #[cfg(test)]
    mod utests {
        use crate::bu8::*;

        #[test]
        fn test_zero() {
            assert_eq!(Bu8::zero().0, 0);
        }

        #[test]
        fn test_one() {
            assert_eq!(Bu8::one().0, 1);
        }

        #[test]
        fn test_pow2() {
            assert_eq!(Bu8::pow2(2).0, 4);
        }

        #[test]
        fn test_low_ones() {
            assert_eq!(Bu8::low_ones(2).0, 3);
        }

        #[test]
        fn test_high_ones() {
            assert_eq!(Bu8::high_ones(7).0, 254);
        }

        #[test]
        fn test_range_ones() {
            assert_eq!(Bu8::range_ones(0, 0).0, 0);
            assert_eq!(Bu8::range_ones(2, 1).0, 4);
            assert_eq!(Bu8::range_ones(2, 2).0, 12);
            assert_eq!(Bu8::range_ones(0, 8).0, 255);
        }

        #[test]
        fn test_range_zeros() {
            assert_eq!(Bu8::range_zeros(0, 8).0, 0);
            assert_eq!(Bu8::range_zeros(0, 0).0, 255);
            assert_eq!(Bu8::range_zeros(1, 7).0, 1);
            assert_eq!(Bu8::range_zeros(1, 6).0, 129);
        }
    }
}

pub mod bitstring {
    use super::*;

    impl Bitstring for Bu8 {
        fn len() -> usize {
            8
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
        use crate::bu8::*;

        #[test]
        fn test_len() {
            assert_eq!(Bu8::len(), 8);
        }

        #[test]
        fn test_get() {
            let bstr = Bu8::from(5);
            assert_eq!(bstr.get(0), Bit::One);
            assert_eq!(bstr.get(1), Bit::Zero);
            assert_eq!(bstr.get(2), Bit::One);
        }

        #[test]
        fn test_set() {
            let mut bstr = Bu8::from(5);
            bstr.set(1);
            assert_eq!(bstr.0, 7);
        }

        #[test]
        fn test_rst() {
            let mut bstr = Bu8::from(5);
            bstr.rst(2);
            assert_eq!(bstr.0, 1);
        }
    }
}

pub mod range {
    use super::*;

    impl Bu8 {
        /// Reset the low bits
        pub fn rst_low(&mut self, len: usize) {
            self.0 &= HIGH_ONES[8 - len]
        }

        /// Reset the high bits
        pub fn rst_high(&mut self, len: usize) {
            self.0 &= LOW_ONES[8 - len]
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
        use crate::bu8::*;

        #[test]
        fn test_rst_low() {
            let mut x = Bu8::from(5);
            x.rst_low(1);
            assert_eq!(x.0, 4);
        }

        #[test]
        fn test_rst_high() {
            let mut x = Bu8::from(5);
            x.rst_high(6);
            assert_eq!(x.0, 1);
        }

        #[test]
        fn test_set_low() {
            let mut x = Bu8::from(5);
            x.set_low(2);
            assert_eq!(x.0, 7);
        }

        #[test]
        fn test_set_high() {
            let mut x = Bu8::from(6);
            x.set_high(7);
            assert_eq!(x.0, 254);
        }
    }
}

pub mod combinators {
    use super::*;

    impl Bu8 {
        /// Combines the bitstring with a second one.
        pub fn combine(&mut self, other: &Self) {
            *self |= *other;
        }

        /// Flips the bit at a given position
        pub fn flip(&mut self, pos: usize) {
            *self ^= Self::pow2(pos)
        }

        /// Splits the bistring in two bitstrings
        pub fn split(&self, pos: usize) -> (Self, Self) {
            let l = self.0 & LOW_ONES[pos];
            let h = self.0 & HIGH_ONES[8 - pos];
            (Bu8::from(l), Bu8::from(h))
        }
    }

    #[cfg(test)]
    mod utests {
        use crate::bu8::*;

        #[test]
        fn test_combinators_split() {
            let x = Bu8::from(56u8);

            for i in 0..9 {
                let (h, t) = x.split(i);
                assert_eq!(h.0 + t.0, x.0);
            }
        }

        #[test]
        fn test_combinators_flip() {
            let mut x = Bu8::from(6u8);
            x.flip(1);
            assert_eq!(x.0, 4);

            x.flip(1);
            assert_eq!(x.0, 6);
        }

        #[test]
        fn test_combinators_combine() {
            let x = Bu8::from(56u8);
            for i in 0..9 {
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

    impl Arbitrary for Bu8 {
        fn arbitrary(g: &mut Gen) -> Self {
            Bu8::from(u8::arbitrary(g))
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            let mut shrunk_vals = Vec::new();
            if self.0 > 0 {
                shrunk_vals.push(Bu8::from(self.0 - 1));
            }
            Box::new(shrunk_vals.into_iter())
        }
    }

    #[cfg(test)]
    mod utests {
        use super::*;

        #[test]
        fn test_shrink() {
            let x = Bu8::from(2);
            let mut xs = x.shrink();
            assert_eq!((*xs).next().unwrap(), Bu8::from(1));
        }
    }
}
