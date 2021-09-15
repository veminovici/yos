//! A crate for some stuff

#![crate_type = "lib"]
#![crate_name = "yos"]
#![deny(missing_docs)]
#![deny(unreachable_code)]

/// Bitwise functionality
mod traits;
pub use crate::traits::{Bitwise, BitwiseDebug};

/// Bitwise implementation for numberic values
mod primitive_u8;
pub use crate::primitive_u8::*;
