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
}
