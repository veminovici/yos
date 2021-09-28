use super::bit::Bit;

/// Defines the constructor functions
pub trait BitsConstructors {
    /// The output type of the constructor functions
    type Output;

    /// Build the 0 value for the bit-string.
    fn zero() -> Self::Output;

    /// Build the 1 value for the bit-string.
    fn one() -> Self::Output;

    /// Build a bit-string value which has all bits set to 1
    fn with_all_ones() -> Self::Output;

    /// Build a bit-string value which is a power of 2
    fn pow2(power: usize) -> Self::Output;

    /// Build a bit-string value with all low bits set to 1
    fn with_low_ones(len: usize) -> Self::Output;

    /// Build a bit-string value with all high bits set to 1
    fn with_high_ones(len: usize) -> Self::Output;
}

/// Defines operations on ranges
pub trait BitsRange {
    /// The output type of the constructor functions
    type Output;

    /// Reset the low range of bits
    fn rst_low_range(&mut self, len: usize);

    /// Reset the high range of bits
    fn rst_high_range(&mut self, len: usize);

    /// Set the low range of bits
    fn set_low_range(&mut self, len: usize);

    /// Set the high range of bits
    fn set_high_range(&mut self, len: usize);

    /// Build a bit-string value with the bits within a range set to 1
    fn with_range_ones(pos: usize, len: usize) -> Self::Output;

    /// Build a bit-string value with the bits within a range set to 0
    fn with_range_zeros(pos: usize, len: usize) -> Self::Output;
}


/// A bit-string is a collection of bits
pub trait Bitstring {
    /// Return the total number of bits
    fn len() -> usize;

    /// Return the value of a bit at a given index
    fn get(&self, ndx: usize) -> Bit;

    /// Set the value of a bit at a given index
    fn set(&mut self, ndx: usize);

    /// Reset the value of a bit at a given index
    fn rst(&mut self, ndx: usize);
}
