//! A rust library for binary heap implementation.

#![deny(missing_docs)]
#![deny(unreachable_pub)]
#![doc = include_str!("../README.md")]

mod binheap;
mod hole;
mod iter;

pub use crate::binheap::*;
pub use crate::hole::*;
pub use crate::iter::*;
