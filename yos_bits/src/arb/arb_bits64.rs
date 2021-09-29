#[cfg(test)]
mod arbitrary_bits64 {
    use super::super::super::bits64::*;

    use quickcheck::{Arbitrary, Gen};

    impl Arbitrary for Bits64 {
        fn arbitrary(g: &mut Gen) -> Self {
            Bits64::from(u64::arbitrary(g))
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            let mut shrunk_vals = Vec::new();
            let u: u64 = (*self).into();
            if u > 0 {
                shrunk_vals.push(Bits64::from(u - 1));
            }
            Box::new(shrunk_vals.into_iter())
        }
    }

    #[cfg(test)]
    mod utests {
        use super::*;

        #[test]
        fn test_shrink() {
            let x = Bits64::from(2);
            let mut xs = x.shrink();
            assert_eq!((*xs).next().unwrap(), Bits64::from(1));
        }
    }
}
