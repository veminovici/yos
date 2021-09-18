//! A crate for bitstring operations
#![deny(missing_docs)]
#![deny(unreachable_code)]

// Nested modules

mod bit;
mod bu8;
mod primitive_u64;
mod primitive_u8;
mod traits;

// Exposed public traits

pub use crate::bit::Bit;
pub use crate::bu8::Bu8;
pub use crate::primitive_u64::*;
pub use crate::primitive_u8::*;
pub use crate::traits::{
    Bitstring, BitstringCombinators, BitstringConstructor, BitstringDebug, BitstringInto,
    BitstringOps, BitstringRange, BitstringShift,
};
