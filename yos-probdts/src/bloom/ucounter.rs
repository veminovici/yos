use std::ops::AddAssign;
use yos_num::*;

/// Implements a positive counter
pub trait UCounter: AddAssign + One + PartialOrd + Zero {}

macro_rules! ucounter_impl {
    ($t:ty) => {
        impl UCounter for $t {}
    };
}

ucounter_impl!(usize);
ucounter_impl!(u8);
ucounter_impl!(u16);
ucounter_impl!(u32);
ucounter_impl!(u64);
ucounter_impl!(u128);
