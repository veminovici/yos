use super::to_bytes::*;
use std::fmt::{Debug, Display};

/// A difficult hash
pub struct DifficultHash {
    /// The size of the difficult bytes
    diff_len: usize,
    /// The size of the regular (remaining) bytes.
    regular_len: usize,
    /// The internal data structure
    data: Vec<u8>,
}

impl DifficultHash {
    /// Build a new instance
    pub fn new(data: Vec<u8>, diff_len: usize) -> DifficultHash {
        let regular_len = data.len() - diff_len;
        DifficultHash {
            diff_len,
            regular_len,
            data,
        }
    }

    /// Returns the dificult section as u128
    pub fn diff128(&self) -> u128 {
        difficulty_bytes(self.data.as_slice(), self.diff_len)
    }
}

impl Display for DifficultHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let xs = self.data.iter();
        write!(f, "{}", to_hex(xs))
    }
}

impl Debug for DifficultHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.diff_len == 0 || self.regular_len == 0 {
            let xs = self.data.iter();
            write!(f, "{}|{}", to_hex(xs), self.diff128())
        } else {
            let xs = self.data.iter().take(self.regular_len);
            let ys = self.data.iter().skip(self.regular_len);
            write!(f, "{}.{}|{}", to_hex(xs), to_hex(ys), self.diff128())
        }
    }
}
