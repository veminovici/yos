//! A crate for bitstring operations
//!
//! The crate exposes u8 and u64 as bit strings.
#![deny(missing_docs)]
#![deny(unreachable_code)]

// Nested modules

mod bit;
mod bits64;
mod bits8;
mod traits;

// Exposed public traits

pub use crate::bit::Bit;
pub use crate::bits64::Bits64;
pub use crate::bits8::Bits8;
pub use crate::traits::{BitsConstructors, Bitstring};
