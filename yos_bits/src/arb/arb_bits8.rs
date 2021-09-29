#[cfg(test)]
mod arbitrary_bits8 {
    use super::super::super::bits8::*;

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
