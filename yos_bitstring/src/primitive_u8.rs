use super::traits::{Bit, Bitstring, BitstringConstructor, BitstringDebug, BitstringOps};

const LOW_ONES: [u8; 9] = [
    0b00000000, 0b00000001, 0b00000011, 0b00000111, 0b00001111, 0b00011111, 0b00111111, 0b01111111,
    0b11111111,
];

const HIGH_ONES: [u8; 9] = [
    0b00000000, 0b10000000, 0b11000000, 0b11100000, 0b11110000, 0b11111000, 0b11111100, 0b11111110,
    0b11111111,
];

/// Computes the bitwise representation of a power of 2
#[inline(always)]
fn pow2(p: usize) -> u8 {
    1 << p
}

/// Reset a given bit
fn rst_bit(x: u8, ndx: usize) -> u8 {
    x & !pow2(ndx)
}

/// Set to 1 a given bit
fn set_bit(x: u8, ndx: usize) -> u8 {
    rst_bit(x, ndx) | pow2(ndx)
}

/// Get the bit value
fn get_bit(x: u8, ndx: usize) -> u8 {
    x & pow2(ndx)
}

impl Bitstring for u8 {
    fn len() -> usize {
        8
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

impl BitstringConstructor<u8> for u8 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }

    fn pow2(power: usize) -> Self {
        pow2(power)
    }

    fn low_ones(len: usize) -> u8 {
        LOW_ONES[len]
    }

    fn high_ones(len: usize) -> u8 {
        HIGH_ONES[len]
    }
}

impl BitstringDebug for u8 {
    fn bdebug(&self) -> String {
        let s = format!("{:#010b}", *self);
        format!("u8:{:3}|{}|", self, s.strip_prefix("0b").unwrap())
    }
}

impl BitstringOps for u8 {
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

    fn shift_left(&mut self, with: usize) {
        *self <<= with;
    }

    fn shift_right(&mut self, with: usize) {
        *self >>= with;
    }

    fn flip(&mut self, pos: usize) {
        *self ^= pow2(pos)
    }

    fn rst_low(&mut self, len: usize) {
        *self &= HIGH_ONES[8 - len]
    }

    fn rst_high(&mut self, len: usize) {
        *self &= LOW_ONES[8 - len]
    }

    fn set_low(&mut self, len: usize) {
        *self |= LOW_ONES[len]
    }

    fn set_high(&mut self, len: usize) {
        *self |= HIGH_ONES[len]
    }
}

#[cfg(test)]
mod utests {
    use super::*;

    #[test]
    fn test_rst_bit() {
        let x = 3u8;
        let y = rst_bit(x, 0);
        assert_eq!(y, 2);
    }

    #[test]
    fn test_set_bit() {
        let x = 2u8;
        let y = set_bit(x, 0);
        assert_eq!(y, 3);
    }

    #[test]
    fn test_get_bit() {
        let x = 3u8;
        assert_eq!(get_bit(x, 0), 1);
        assert_eq!(get_bit(x, 1), 2);
        assert_eq!(get_bit(x, 2), 0);
    }

    #[test]
    fn test_bstr_len() {
        assert_eq!(u8::len(), 8);
    }

    #[test]
    fn test_bstr_rst() {
        let mut x = 3u8;
        x.rst(0);
        assert_eq!(x, 2);
    }

    #[test]
    fn test_bstr_set() {
        let mut x = 2u8;
        x.set(0);
        assert_eq!(x, 3);
    }

    #[test]
    fn test_bstr_get() {
        let x = 3u8;
        assert_eq!(x.get(0), Bit::One);
        assert_eq!(x.get(1), Bit::One);
        assert_eq!(x.get(2), Bit::Zero);
    }

    #[test]
    fn test_bstr_ops_and() {
        let mut x = 5u8;
        x.and(&3);
        assert_eq!(x, 1);
    }

    #[test]
    fn test_bstr_ops_or() {
        let mut x = 5u8;
        x.or(&3);
        assert_eq!(x, 7);
    }

    #[test]
    fn test_bstr_ops_xor() {
        let mut x = 5u8;
        x.xor(&3);
        assert_eq!(x, 6);
    }

    #[test]
    fn test_bstr_ops_neg() {
        let mut x = 1u8;
        x.neg();
        assert_eq!(x, 254);
    }

    #[test]
    fn test_bstr_ops_shift_left() {
        let mut x = 3u8;
        x.shift_left(1);
        assert_eq!(x, 6);
    }

    #[test]
    fn test_bstr_ops_shift_right() {
        let mut x = 6u8;
        x.shift_right(1);
        assert_eq!(x, 3);
    }

    #[test]
    fn test_bstr_ops_flip() {
        let mut x = 6u8;
        x.flip(1);
        assert_eq!(x, 4);

        //x.flip(1);
        //assert_eq!(x, 6);
    }

    #[test]
    fn test_bstr_ops_rst_low() {
        let mut x = 7u8;
        x.rst_low(2);
        assert_eq!(x, 4);
    }

    #[test]
    fn test_bstr_ops_rst_high() {
        let mut x = 255u8;
        x.rst_high(7);
        assert_eq!(x, 1);
    }

    #[test]
    fn test_bstr_ops_set_low() {
        let mut x = 4u8;
        x.set_low(2);
        assert_eq!(x, 7);
    }

    #[test]
    fn test_bstr_ops_set_high() {
        let mut x = 1u8;
        x.set_high(7);
        assert_eq!(x, 255);
    }

    #[test]
    fn test_bstr_debug_bdebug() {
        let mut x = 5u8;
        x.neg();
        let s = x.bdebug();
        assert_eq!(s.len(), 16);
    }

    #[test]
    fn test_bstr_constructor_zero() {
        assert_eq!(u8::zero(), 0);
    }

    #[test]
    fn test_bstr_constructor_one() {
        assert_eq!(u8::one(), 1);
    }

    #[test]
    fn test_bstr_constructor_pow2() {
        assert_eq!(u8::pow2(3), 8);
    }

    #[test]
    fn test_bstr_constructor_low_ones() {
        let x = u8::low_ones(3);
        assert_eq!(x, 7);
    }

    #[test]
    fn test_bstr_constructor_high_ones() {
        let x = u8::high_ones(7);
        assert_eq!(x, 254);
    }
}

#[cfg(test)]
mod ptests {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn prop_bit_set_reset_set(x: u8) -> bool {
        let y = set_bit(x, 1);
        let z = rst_bit(y, 1);
        let w = set_bit(z, 1);
        y == w
    }

    #[quickcheck]
    fn prop_bstr_set_rst_set(x: u8) -> bool {
        let mut a = x;
        a.set(1);
        let y = a;
        a.rst(1);
        a.set(1);
        a == y
    }
}
