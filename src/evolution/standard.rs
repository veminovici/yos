//! A crate for some stuff

use super::Evolution;
use crate::Bitstring;

/// Standard evolution
pub struct Standard {}

impl<A: Bitstring + Sized> Evolution<A> for Standard {
    fn mutate(bstr: &mut A, ndx: usize) {
        bstr.bflip(ndx)
    }

    fn crossover(a: &mut A, b: &mut A, cut: usize) {
        let (mut al, ah) = a.bsplit(cut);
        let (mut bl, bh) = b.bsplit(cut);
        al.bcombine(&bh);
        bl.bcombine(&ah);

        *a = al;
        *b = bl;
    }
}

#[cfg(test)]
mod utest {
    use super::*;

    #[test]
    fn test_mutate() {
        let mut v = 5u8;
        Standard::mutate(&mut v, 0);
        assert_eq!(v, 4);
    }

    #[test]
    fn test_crossover() {
        let x = 5u8;
        let (mut xl, mut xh) = x.bsplit(4);
        Standard::crossover(&mut xl, &mut xh, 4);
        assert_eq!(xl + xh, x);
    }
}
