//! A crate for bitstring operations
#![deny(missing_docs)]
#![deny(unreachable_code)]

// Nested modules

mod bit;
mod bu64;
mod bu8;
mod traits;

// Exposed public traits

pub use crate::bit::Bit;
pub use crate::bu64::Bu64;
pub use crate::bu8::Bu8;
pub use crate::traits::Bitstring;
