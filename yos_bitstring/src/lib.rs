//! A crate for bitstring operations
#![deny(missing_docs)]
#![deny(unreachable_code)]

// Nested modules

mod primitive_u8;
mod traits;

// Exposed public traits

pub use crate::traits::{Bit, Bitstring, BitstringConstructor, BitstringDebug, BitstringOps};
