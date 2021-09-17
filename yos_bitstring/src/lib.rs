//! A crate for bitstring operations
#![deny(missing_docs)]
#![deny(unreachable_code)]

// Nested modules

mod primitive_u64;
mod primitive_u8;
mod traits;

// Exposed public traits

pub use crate::primitive_u64::*;
pub use crate::primitive_u8::*;
pub use crate::traits::{
    Bit, Bitstring, BitstringCombinators, BitstringConstructor, BitstringDebug, BitstringInto,
    BitstringOps, BitstringRange, BitstringShift,
};
