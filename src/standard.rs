use crate::{Bitwise, Evolution};

/// Standard evolution
pub struct Standard {}

impl Evolution for Standard {
    fn mutate(bstr: &mut dyn Bitwise, ndx: usize) {
        bstr.flip(ndx);
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
}
