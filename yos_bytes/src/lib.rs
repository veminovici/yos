//! A crate for bitstring operations
//!
//! The crate exposes u8 and u64 as bit strings.
#![deny(missing_docs)]
#![deny(unreachable_code)]

// Nested modules

mod difficult_hash;
mod to_bytes;

// Public elements

pub use crate::difficult_hash::*;
pub use crate::to_bytes::*;
