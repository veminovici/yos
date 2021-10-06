//! A rust library for a cursor that travels over
//! a slice of memory and is able to move the values
//! from one position to another using diret memory access.

#![deny(missing_docs)]
#![deny(unreachable_pub)]

mod memcursor;

pub use crate::memcursor::*;
