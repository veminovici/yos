//! A crate for some stuff

#![crate_type = "lib"]
#![crate_name = "yos"]
#![deny(missing_docs)]
#![deny(unreachable_code)]

/// Public modules
pub mod evolution;

/// Bitwise and evolutionary functionality
mod traits;
pub use crate::traits::{Bit, Bitstring, BitstringDebug};

/// Bitwise implementation for u8
mod primitive_u8;
pub use crate::primitive_u8::*;

/// Bitwise implemetation for u64
mod primitive_u64;
pub use crate::primitive_u64::*;
