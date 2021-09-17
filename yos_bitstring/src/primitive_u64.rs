use super::bit::Bit;
use super::traits::{
    Bitstring, BitstringCombinators, BitstringConstructor, BitstringDebug, BitstringInto,
    BitstringOps, BitstringRange, BitstringShift,
};

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

/// Computes the bitwise representation of a power of 2
#[inline(always)]
fn pow2(p: usize) -> u64 {
    1 << p
}

/// Reset a given bit
fn rst_bit(x: u64, ndx: usize) -> u64 {
    x & !pow2(ndx)
}

/// Set to 1 a given bit
fn set_bit(x: u64, ndx: usize) -> u64 {
    rst_bit(x, ndx) | pow2(ndx)
}

/// Get the bit value
fn get_bit(x: u64, ndx: usize) -> u64 {
    x & pow2(ndx)
}

impl Bitstring for u64 {
    fn len() -> usize {
        64
    }

    fn get(&self, ndx: usize) -> Bit {
        if get_bit(*self, ndx) == 0 {
            Bit::Zero
        } else {
            Bit::One
        }
    }

    fn set(&mut self, ndx: usize) {
        *self = set_bit(*self, ndx);
    }

    fn rst(&mut self, ndx: usize) {
        *self = rst_bit(*self, ndx);
    }
}

impl BitstringConstructor<u64> for u64 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }

    fn pow2(power: usize) -> Self {
        pow2(power)
    }

    fn low_ones(len: usize) -> Self {
        LOW_ONES[len]
    }

    fn high_ones(len: usize) -> Self {
        HIGH_ONES[len]
    }
}

/// Prints in binary format an u8 value without the 0b prefix.
fn u8_debug(u: &u8) -> String {
    let str = format!("{:#010b}", u);
    str.strip_prefix("0b").unwrap().to_string()
}

impl BitstringDebug for u64 {
    fn bdebug(&self) -> String {
        let ueights = self.to_u8s().iter().map(u8_debug).collect::<Vec<String>>();
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

impl BitstringOps for u64 {
    fn and(&mut self, other: &Self) {
        *self &= *other;
    }

    fn or(&mut self, other: &Self) {
        *self |= *other;
    }

    fn xor(&mut self, other: &Self) {
        *self ^= *other;
    }

    fn neg(&mut self) {
        *self = !*self
    }
}

impl BitstringRange<u64> for u64 {
    fn rst_low(&mut self, len: usize) {
        *self &= HIGH_ONES[64 - len]
    }

    fn rst_high(&mut self, len: usize) {
        *self &= LOW_ONES[64 - len]
    }

    fn set_low(&mut self, len: usize) {
        *self |= LOW_ONES[len]
    }

    fn set_high(&mut self, len: usize) {
        *self |= HIGH_ONES[len]
    }
}

impl BitstringShift<u64> for u64 {
    fn shift_left(&mut self, with: usize) {
        *self <<= with;
    }

    fn shift_right(&mut self, with: usize) {
        *self >>= with;
    }
}

impl BitstringCombinators<u64> for u64 {
    fn split(&self, pos: usize) -> (u64, u64) {
        let l = self & LOW_ONES[pos];
        let h = self & HIGH_ONES[64 - pos];
        (l, h)
    }

    fn flip(&mut self, pos: usize) {
        *self ^= pow2(pos)
    }

    fn combine(&mut self, other: &u64) {
        self.or(other);
    }
}

impl BitstringInto<u64> for u64 {
    fn to_u8s(&self) -> Vec<u8> {
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

#[cfg(test)]
mod utests {
    use super::*;

    #[test]
    fn test_rst_bit() {
        let x = 3u64;
        let y = rst_bit(x, 0);
        assert_eq!(y, 2);
    }

    #[test]
    fn test_set_bit() {
        let x = 2u64;
        let y = set_bit(x, 0);
        assert_eq!(y, 3);
    }

    #[test]
    fn test_get_bit() {
        let x = 3u64;
        assert_eq!(get_bit(x, 0), 1);
        assert_eq!(get_bit(x, 1), 2);
        assert_eq!(get_bit(x, 2), 0);
    }

    #[test]
    fn test_bstr_len() {
        assert_eq!(u64::len(), 64);
    }

    #[test]
    fn test_bstr_rst() {
        let mut x = 3u64;
        x.rst(0);
        assert_eq!(x, 2);
    }

    #[test]
    fn test_bstr_set() {
        let mut x = 2u64;
        x.set(0);
        assert_eq!(x, 3);
    }

    #[test]
    fn test_bstr_get() {
        let x = 3u64;
        assert_eq!(x.get(0), Bit::One);
        assert_eq!(x.get(1), Bit::One);
        assert_eq!(x.get(2), Bit::Zero);
    }

    #[test]
    fn test_bstr_ops_and() {
        let mut x = 5u64;
        x.and(&3);
        assert_eq!(x, 1);
    }

    #[test]
    fn test_bstr_ops_or() {
        let mut x = 5u64;
        x.or(&3);
        assert_eq!(x, 7);
    }

    #[test]
    fn test_bstr_ops_xor() {
        let mut x = 5u64;
        x.xor(&3);
        assert_eq!(x, 6);
    }

    #[test]
    fn test_bstr_ops_neg() {
        let mut x = 1u64;
        x.neg();
        assert_eq!(x, HIGH_ONES[63]);
    }

    #[test]
    fn test_bstr_shift_left() {
        let mut x = 3u64;
        x.shift_left(1);
        assert_eq!(x, 6);
    }

    #[test]
    fn test_bstr_shift_right() {
        let mut x = 6u64;
        x.shift_right(1);
        assert_eq!(x, 3);
    }

    #[test]
    fn test_bstr_range_rst_low() {
        let mut x = 7u64;
        x.rst_low(2);
        assert_eq!(x, 4);
    }

    #[test]
    fn test_bstr_range_rst_high() {
        let mut x = u64::MAX;
        x.rst_high(63);
        assert_eq!(x, 1);
    }

    #[test]
    fn test_bstr_range_set_low() {
        let mut x = 4u64;
        x.set_low(2);
        assert_eq!(x, 7);
    }

    #[test]
    fn test_bstr_range_set_high() {
        let mut x = 1u64;
        x.set_high(63);
        assert_eq!(x, u64::MAX);
    }

    #[test]
    fn test_bstr_debug_bdebug() {
        let mut x = 5u64;
        x.neg();
        let s = x.bdebug();
        assert_eq!(s.len(), 97);
    }

    #[test]
    fn test_bstr_constructor_zero() {
        assert_eq!(u64::zero(), 0);
    }

    #[test]
    fn test_bstr_constructor_one() {
        assert_eq!(u64::one(), 1);
    }

    #[test]
    fn test_bstr_constructor_pow2() {
        assert_eq!(u64::pow2(3), 8);
    }

    #[test]
    fn test_bstr_constructor_low_ones() {
        let x = u64::low_ones(3);
        assert_eq!(x, 7);
    }

    #[test]
    fn test_bstr_constructor_high_ones() {
        let x = u64::high_ones(63);
        assert_eq!(x, u64::MAX - 1);
    }

    #[test]
    fn test_bstr_combinators_split() {
        let x = 56u64;

        for i in 0..65 {
            let (h, t) = x.split(i);
            assert_eq!(h + t, x);
        }
    }

    #[test]
    fn test_bstr_combinators_flip() {
        let mut x = 6u64;
        x.flip(1);
        assert_eq!(x, 4);

        x.flip(1);
        assert_eq!(x, 6);
    }

    #[test]
    fn test_bstr_combinators_combine() {
        let x = 56u64;

        for i in 0..65 {
            let (mut h, t) = x.split(i);
            assert_eq!(h + t, x);
            h.combine(&t);
            assert_eq!(h, x);
        }
    }
}

#[cfg(test)]
mod ptests {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn prop_bit_set_reset_set(x: u64) -> bool {
        let y = set_bit(x, 1);
        let z = rst_bit(y, 1);
        let w = set_bit(z, 1);
        y == w
    }

    #[quickcheck]
    fn prop_bstr_set_rst_set(x: u64) -> bool {
        let mut a = x;
        a.set(1);
        let y = a;
        a.rst(1);
        a.set(1);
        a == y
    }

    #[quickcheck]
    fn prop_into_u8s(val: u64) -> bool {
        let ueights = val.to_u8s();
        let u0 = (val & 255) as u8;
        (ueights.len() == 8) && (ueights[0] == u0)
    }
}
