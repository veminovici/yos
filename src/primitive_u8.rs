use crate::Bitwise;

fn mask(ndx: usize) -> u8 {
    1 << ndx
}

/// Implementation of the bitwise trait for the u8.
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
}

#[cfg(test)]
mod utest {
    use super::*;

    #[test]
    fn set_bit() {
        let mut v = 0u8;
        v.set(0);
        assert_eq!(v, 1);
    }

    #[test]
    fn reset_bit() {
        let mut v = 1u8;
        v.reset(0);
        assert_eq!(v, 0);
    }

    #[test]
    fn flip_bit() {
        let mut v = 1u8;
        v.flip(0);
        assert_eq!(v, 0);
    }

    #[test]
    fn get_bit() {
        let v = 1u8;
        let b = v.get(0);
        assert_eq!(b, 1);
    }
}
