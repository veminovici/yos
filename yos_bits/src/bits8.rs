use super::bit::Bit;
use super::{Bits, BitsConstructors, BitsRange};

/// Different internal constant values
mod constants {
    /// A series of u8 values, where the lower bits are set to 1
    pub const LOW_ONES: [u8; 9] = [
        0b00000000, 0b00000001, 0b00000011, 0b00000111, 0b00001111, 0b00011111, 0b00111111,
        0b01111111, 0b11111111,
    ];

    /// A series of u8 values where the higher bugs are set to 1
    pub const HIGH_ONES: [u8; 9] = [
        0b00000000, 0b10000000, 0b11000000, 0b11100000, 0b11110000, 0b11111000, 0b11111100,
        0b11111110, 0b11111111,
    ];
}

/// Different helper function which perform bitwise operations on u8 values.
mod helper {
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
pub struct Bits8(u8);

/// Implementation of formatting traits
pub mod formatting {
    use super::Bits8;
    use std::fmt::{Debug, Display};

    /// Implements Display trait for "{}" formatting
    impl Display for Bits8 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let s = format!("{:#010b}", self.0);
            write!(f, "Bits8:{:03}|{}|", self.0, s.strip_prefix("0b").unwrap())
        }
    }

    /// Implements Debug trait for "{:?}" formatting
    impl Debug for Bits8 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let s = format!("{:#010b}", self.0);
            write!(f, "Bits8:{:03}|{}|", self.0, s.strip_prefix("0b").unwrap())
        }
    }

    #[cfg(test)]
    mod ptests {
        use crate::bits8::*;
        use quickcheck_macros::quickcheck;

        #[quickcheck]
        fn prop_display(x: Bits8) -> bool {
            let s = format!("{}", x);
            s.len() == 19
        }

        #[quickcheck]
        fn prop_debug(x: Bits8) -> bool {
            let s = format!("{:?}", x);
            s.len() == 19
        }

        #[quickcheck]
        fn prop_display_debug(x: u8) -> bool {
            let x = Bits8::from(x);
            let d1 = format!("{}", x);
            let d2 = format!("{:?}", x);
            d1 == d2
        }
    }
}

/// Iterator and IntoIterator implementations
pub mod iterator {
    use super::*;

    /// Iterator over the bits stored into an Bits8 value.
    pub struct IterBits8 {
        /// The bit string
        bits: Bits8,

        /// The current index
        ndx: usize,
    }

    impl IterBits8 {
        /// Builds a new instance of the Bits8 iterator
        pub fn new(bits: Bits8) -> Self {
            Self { bits, ndx: 0 }
        }
    }

    /// Iterator trait implementation
    impl Iterator for IterBits8 {
        type Item = Bit;

        fn next(&mut self) -> Option<Self::Item> {
            if self.ndx == Bits8::len() {
                None
            } else {
                let v = self.bits.get(self.ndx);
                self.ndx += 1;
                Some(v)
            }
        }
    }

    /// IntoIterator trait implementation for the Bits8
    impl IntoIterator for Bits8 {
        type Item = Bit;
        type IntoIter = IterBits8;

        fn into_iter(self) -> Self::IntoIter {
            IterBits8::new(self)
        }
    }

    #[cfg(test)]
    mod utests {
        use super::IterBits8;
        use crate::bits8::*;
        use std::iter::FromIterator;

        #[test]
        fn test_iter() {
            let bits = Bits8::one();
            let mut iter = IterBits8::new(bits);

            assert_eq!(Some(Bit::One), iter.next());
            assert_eq!(Some(Bit::Zero), iter.next());
            assert_eq!(Some(Bit::Zero), iter.next());
            assert_eq!(Some(Bit::Zero), iter.next());
            assert_eq!(Some(Bit::Zero), iter.next());
            assert_eq!(Some(Bit::Zero), iter.next());
            assert_eq!(Some(Bit::Zero), iter.next());
            assert_eq!(Some(Bit::Zero), iter.next());
            assert_eq!(None, iter.next());
        }

        #[test]
        fn test_into_iter() {
            let bits = Bits8::one();
            let mut iter = bits.into_iter();

            assert_eq!(Some(Bit::One), iter.next());
            assert_eq!(Some(Bit::Zero), iter.next());
            assert_eq!(Some(Bit::Zero), iter.next());
            assert_eq!(Some(Bit::Zero), iter.next());
            assert_eq!(Some(Bit::Zero), iter.next());
            assert_eq!(Some(Bit::Zero), iter.next());
            assert_eq!(Some(Bit::Zero), iter.next());
            assert_eq!(Some(Bit::Zero), iter.next());
            assert_eq!(None, iter.next());
        }

        #[test]
        fn test_enumerate() {
            let bits = Bits8::one();
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
            let bits = Bits8::one();
            let bits1 = Bits8::from_iter(bits.into_iter());
            assert_eq!(bits, bits1);
        }
    }
}

/// Implementations for From and FromIterator traits
pub mod conversions {
    use super::*;
    use std::iter::FromIterator;

    /// Converts from u8 to Bits8
    impl From<u8> for Bits8 {
        fn from(x: u8) -> Self {
            Bits8(x)
        }
    }

    /// Converts from Bits8 to u8
    impl From<Bits8> for u8 {
        fn from(x: Bits8) -> Self {
            x.0
        }
    }

    /// Converts from Bits8 to Vec<u8>
    impl From<Bits8> for Vec<u8> {
        fn from(x: Bits8) -> Self {
            vec![x.0]
        }
    }

    /// Converts from an iterator of Bits to Bits8
    impl FromIterator<Bit> for Bits8 {
        fn from_iter<T: IntoIterator<Item = Bit>>(iter: T) -> Self {
            let mut us: Vec<u8> = Vec::with_capacity(8);
            for b in iter {
                us.push(b.into())
            }

            Self::from_iter(us)
        }
    }

    /// Converts from an interator of u8 to Bits8
    impl FromIterator<u8> for Bits8 {
        fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
            let xs = iter.into_iter().take(8).collect::<Vec<u8>>();
            let mut u = 0u8;

            for (ndx, x) in xs.iter().enumerate() {
                if *x != 0 {
                    u += Bits8::pow2(ndx).0;
                }
            }

            Self::from(u)
        }
    }

    #[cfg(test)]
    mod ptests {
        use crate::bits8::*;
        use quickcheck_macros::quickcheck;
        use std::iter::FromIterator;

        #[quickcheck]
        fn prop_from_to_u8(x: u8) -> bool {
            let bstr = Bits8::from(x);
            let y: u8 = bstr.into();
            bstr.0 == x && x == y
        }

        #[quickcheck]
        fn prop_form_to_vecu8() -> bool {
            let xs = vec![0, 1, 0, 0, 0, 0, 0, 0];
            let bstr = Bits8::from_iter(xs);
            let y: Vec<u8> = bstr.into();
            bstr.0 == 2 && y.len() == 1 && 2 == y[0]
        }
    }
}

pub mod operators {
    use super::*;
    use std::ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl,
        ShlAssign, Shr, ShrAssign, Sub, SubAssign,
    };

    impl Add for Bits8 {
        type Output = Bits8;

        fn add(self, rhs: Self) -> Self::Output {
            Bits8(self.0 + rhs.0)
        }
    }

    impl AddAssign for Bits8 {
        fn add_assign(&mut self, rhs: Self) {
            self.0 += rhs.0;
        }
    }

    impl Sub for Bits8 {
        type Output = Bits8;

        fn sub(self, rhs: Self) -> Self::Output {
            Bits8(self.0 - rhs.0)
        }
    }

    impl SubAssign for Bits8 {
        fn sub_assign(&mut self, rhs: Self) {
            self.0 -= rhs.0;
        }
    }

    impl BitAnd for Bits8 {
        type Output = Bits8;

        fn bitand(self, rhs: Self) -> Self::Output {
            Bits8(self.0 & rhs.0)
        }
    }

    impl BitAndAssign for Bits8 {
        fn bitand_assign(&mut self, rhs: Self) {
            self.0 = self.0 & rhs.0;
        }
    }

    impl BitOr for Bits8 {
        type Output = Bits8;

        fn bitor(self, rhs: Self) -> Self::Output {
            Bits8(self.0 | rhs.0)
        }
    }

    impl BitOrAssign for Bits8 {
        fn bitor_assign(&mut self, rhs: Self) {
            self.0 = self.0 | rhs.0;
        }
    }

    impl BitXor for Bits8 {
        type Output = Bits8;

        fn bitxor(self, rhs: Self) -> Self::Output {
            Bits8(self.0 ^ rhs.0)
        }
    }

    impl BitXorAssign for Bits8 {
        fn bitxor_assign(&mut self, rhs: Self) {
            self.0 = self.0 ^ rhs.0;
        }
    }

    impl Not for Bits8 {
        type Output = Bits8;

        fn not(self) -> Self::Output {
            Bits8::from(!self.0)
        }
    }

    impl Shr<usize> for Bits8 {
        type Output = Bits8;

        fn shr(self, rhs: usize) -> Self::Output {
            Bits8(self.0 >> rhs)
        }
    }

    impl ShrAssign<usize> for Bits8 {
        fn shr_assign(&mut self, rhs: usize) {
            self.0 >>= rhs;
        }
    }

    impl Shl<usize> for Bits8 {
        type Output = Bits8;

        fn shl(self, rhs: usize) -> Self::Output {
            Bits8(self.0 << rhs)
        }
    }

    impl ShlAssign<usize> for Bits8 {
        fn shl_assign(&mut self, rhs: usize) {
            self.0 <<= rhs;
        }
    }

    #[cfg(test)]
    mod utests {
        use super::constants::*;
        use crate::bits8::*;

        #[test]
        fn test_add() {
            let x = Bits8::from(5);
            let y = x + Bits8::from(3);
            assert_eq!(y.0, 8);
        }

        #[test]
        fn test_add_and_assign() {
            let mut x = Bits8::from(5);
            x += Bits8::from(3);
            assert_eq!(x.0, 8);
        }

        #[test]
        fn test_sub() {
            let x = Bits8::from(5);
            let y = x - Bits8::from(3);
            assert_eq!(y.0, 2);
        }

        #[test]
        fn test_sub_and_assign() {
            let mut x = Bits8::from(5);
            x -= Bits8::from(3);
            assert_eq!(x.0, 2);
        }

        #[test]
        fn test_and() {
            let x = Bits8::from(5);
            let y = x & Bits8::from(3);
            assert_eq!(y.0, 1);
        }

        #[test]
        fn test_and_assign() {
            let mut x = Bits8::from(5);
            x &= Bits8::from(3);
            assert_eq!(x.0, 1);
        }

        #[test]
        fn test_or() {
            let x = Bits8::from(5);
            let y = x | Bits8::from(3);
            assert_eq!(y.0, 7);
        }

        #[test]
        fn test_or_assign() {
            let mut x = Bits8::from(5);
            x |= Bits8::from(3);
            assert_eq!(x.0, 7);
        }

        #[test]
        fn test_xor() {
            let x = Bits8::from(5);
            let y = x ^ Bits8::from(3);
            assert_eq!(y.0, 6);
        }

        #[test]
        fn test_xor_assign() {
            let mut x = Bits8::from(5);
            x ^= Bits8::from(3);
            assert_eq!(x.0, 6);
        }

        #[test]
        fn test_not_assign() {
            let x = Bits8::from(LOW_ONES[1]);
            let y = !x;
            assert_eq!(y.0, HIGH_ONES[7])
        }

        #[test]
        fn test_shr() {
            let x = Bits8::from(5);
            let y = x >> 1;
            assert_eq!(y.0, 2);
        }

        #[test]
        fn test_shr_assign() {
            let mut x = Bits8::from(5);
            x >>= 1;
            assert_eq!(x.0, 2);
        }

        #[test]
        fn test_shl() {
            let x = Bits8::from(3);
            let y = x << 1;
            assert_eq!(y.0, 6);
        }

        #[test]
        fn test_shl_assign() {
            let mut x = Bits8::from(3);
            x <<= 1;
            assert_eq!(x.0, 6);
        }
    }
}

pub mod constructors {
    use super::constants::*;
    use super::*;

    impl BitsConstructors for Bits8 {
        type Output = Self;

        fn zero() -> Self::Output {
            Bits8(0)
        }

        fn one() -> Self::Output {
            Bits8(1)
        }

        fn with_all_ones() -> Self::Output {
            Bits8::from(LOW_ONES[8])
        }

        fn pow2(power: usize) -> Self::Output {
            Bits8(helper::pow2(power))
        }

        fn with_low_ones(len: usize) -> Self::Output {
            Bits8(LOW_ONES[len])
        }

        fn with_high_ones(len: usize) -> Self::Output {
            Bits8(HIGH_ONES[len])
        }

        fn split_at(&self, pos: usize) -> (Self::Output, Self::Output) {
            let l = self.0 & LOW_ONES[pos];
            let h = self.0 & HIGH_ONES[8 - pos];
            (Bits8::from(l), Bits8::from(h))
        }
    }

    #[cfg(test)]
    mod utests {
        use super::constants::*;
        use crate::bits8::*;

        #[test]
        fn test_zero() {
            assert_eq!(Bits8::zero().0, 0);
        }

        #[test]
        fn test_one() {
            assert_eq!(Bits8::one().0, 1);
        }

        #[test]
        fn test_with_all_ones() {
            assert_eq!(Bits8::with_all_ones().0, LOW_ONES[8]);
        }

        #[test]
        fn test_pow2() {
            assert_eq!(Bits8::pow2(2).0, 4);
        }

        #[test]
        fn test_with_low_ones() {
            assert_eq!(Bits8::with_low_ones(2).0, 3);
        }

        #[test]
        fn test_with_high_ones() {
            assert_eq!(Bits8::with_high_ones(7).0, 254);
        }

        #[test]
        fn test_split_at() {
            let x = Bits8::from(56u8);

            for i in 0..9 {
                let (h, t) = x.split_at(i);
                assert_eq!(h.0 + t.0, x.0);
            }
        }
    }
}

pub mod bits {
    use super::*;

    impl Bits for Bits8 {
        fn len() -> usize {
            8
        }

        fn get(&self, ndx: usize) -> Bit {
            if helper::get(self.0, ndx) == 0 {
                Bit::Zero
            } else {
                Bit::One
            }
        }

        fn set(&mut self, ndx: usize) {
            self.0 = helper::set(self.0, ndx);
        }

        fn rst(&mut self, ndx: usize) {
            self.0 = helper::rst(self.0, ndx);
        }

        fn flip(&mut self, ndx: usize) {
            *self ^= Self::pow2(ndx)
        }
    }

    #[cfg(test)]
    mod utests {
        use crate::bits8::*;

        #[test]
        fn test_len() {
            assert_eq!(Bits8::len(), 8);
        }

        #[test]
        fn test_get() {
            let bstr = Bits8::from(5);
            assert_eq!(bstr.get(0), Bit::One);
            assert_eq!(bstr.get(1), Bit::Zero);
            assert_eq!(bstr.get(2), Bit::One);
        }

        #[test]
        fn test_set() {
            let mut bstr = Bits8::from(5);
            bstr.set(1);
            assert_eq!(bstr.0, 7);
        }

        #[test]
        fn test_rst() {
            let mut bstr = Bits8::from(5);
            bstr.rst(2);
            assert_eq!(bstr.0, 1);
        }

        #[test]
        fn test_flip() {
            let mut x = Bits8::from(6u8);
            x.flip(1);
            assert_eq!(x.0, 4);

            x.flip(1);
            assert_eq!(x.0, 6);
        }
    }
}

pub mod range {
    use super::constants::*;
    use super::*;

    impl BitsRange for Bits8 {
        type Output = Self;

        fn rst_low_range(&mut self, len: usize) {
            self.0 &= HIGH_ONES[8 - len]
        }

        fn rst_high_range(&mut self, len: usize) {
            self.0 &= LOW_ONES[8 - len]
        }

        fn set_low_range(&mut self, len: usize) {
            self.0 |= LOW_ONES[len]
        }

        fn set_high_range(&mut self, len: usize) {
            self.0 |= HIGH_ONES[len]
        }

        fn with_range_ones(pos: usize, len: usize) -> Self::Output {
            let mut mask = Bits8::with_low_ones(len);
            mask <<= pos;
            mask
        }

        fn with_range_zeros(pos: usize, len: usize) -> Self::Output {
            let mut mask = Bits8::zero();
            if pos > 0 {
                mask = Bits8::with_low_ones(pos);
            }

            mask |= Bits8::with_high_ones(8 - pos - len);
            mask
        }
    }

    #[cfg(test)]
    mod utests {
        use crate::bits8::*;

        #[test]
        fn test_rst_low_range() {
            let mut x = Bits8::from(5);
            x.rst_low_range(1);
            assert_eq!(x.0, 4);
        }

        #[test]
        fn test_rst_high_range() {
            let mut x = Bits8::from(5);
            x.rst_high_range(6);
            assert_eq!(x.0, 1);
        }

        #[test]
        fn test_set_low_range() {
            let mut x = Bits8::from(5);
            x.set_low_range(2);
            assert_eq!(x.0, 7);
        }

        #[test]
        fn test_set_high_range() {
            let mut x = Bits8::from(6);
            x.set_high_range(7);
            assert_eq!(x.0, 254);
        }

        #[test]
        fn test_with_range_ones() {
            assert_eq!(Bits8::with_range_ones(0, 0).0, 0);
            assert_eq!(Bits8::with_range_ones(2, 1).0, 4);
            assert_eq!(Bits8::with_range_ones(2, 2).0, 12);
            assert_eq!(Bits8::with_range_ones(0, 8).0, 255);
        }

        #[test]
        fn test_with_range_zeros() {
            assert_eq!(Bits8::with_range_zeros(0, 8).0, 0);
            assert_eq!(Bits8::with_range_zeros(0, 0).0, 255);
            assert_eq!(Bits8::with_range_zeros(1, 7).0, 1);
            assert_eq!(Bits8::with_range_zeros(1, 6).0, 129);
        }
    }
}

pub mod combinators {
    use super::*;

    impl Bits8 {
        /// Combines the bitstring with a second one.
        pub fn combine(&mut self, other: &Self) {
            *self |= *other;
        }
    }

    #[cfg(test)]
    mod utests {
        use crate::bits8::*;

        #[test]
        fn test_combinators_combine() {
            let x = Bits8::from(56u8);
            for i in 0..9 {
                let (mut h, t) = x.split_at(i);
                assert_eq!(h.0 + t.0, x.0);
                h.combine(&t);
                assert_eq!(h, x);
            }
        }
    }
}

#[cfg(test)]
mod arbitrary_bits8 {
    use super::*;

    use quickcheck::{Arbitrary, Gen};

    impl Arbitrary for Bits8 {
        fn arbitrary(g: &mut Gen) -> Self {
            Bits8::from(u8::arbitrary(g))
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            let mut shrunk_vals = Vec::new();
            let u: u8 = (*self).into();
            if u > 0 {
                shrunk_vals.push(Bits8::from(u - 1));
            }
            Box::new(shrunk_vals.into_iter())
        }
    }

    #[cfg(test)]
    mod utests {
        use super::*;

        #[test]
        fn test_shrink() {
            let x = Bits8::from(2);
            let mut xs = x.shrink();
            assert_eq!((*xs).next().unwrap(), Bits8::from(1));
        }
    }
}
