//! A crate for some stuff
use crate::Bitstring;

/// Crossovers two bitstring.
pub fn crossover<B: Bitstring>(a: &mut B, b: &mut B, cut: usize) {
    let (mut al, ah) = a.bsplit(cut);
    let (mut bl, bh) = b.bsplit(cut);
    al.bcombine(&bh);
    bl.bcombine(&ah);

    *a = al;
    *b = bl;
}

/// Mutate a bit at a given position
pub fn mutate<B: Bitstring>(bstr: &mut B, ndx: usize) {
    bstr.bflip(ndx)
}

#[cfg(test)]
mod utest {
    use super::*;

    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn prop_mutate(x: u8) -> bool {
        let mut y = x;

        mutate(&mut y, 2);
        mutate(&mut y, 2);

        y == x
    }
}
