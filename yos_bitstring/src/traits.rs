use super::bit::Bit;

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

/// Constants for bitstring
pub trait BitstringConstructor<A: Bitstring> {
    /// Return a bistring representing 0
    fn zero() -> A;

    /// Return a bitstring representing 1
    fn one() -> A;

    /// Return the power of 2
    fn pow2(power: usize) -> A;

    /// Build a bistring with all low bits set to 1
    fn low_ones(len: usize) -> A;

    /// Build a bitstring with all high bits set to 1
    fn high_ones(len: usize) -> A;
}

/// The debug representation of a bitwise structure.
pub trait BitstringDebug {
    /// Returns the debug representation
    fn bdebug(&self) -> String;
}

/// Bitwise primitive operations
pub trait BitstringOps {
    /// Apply a bitwise AND between two bitstrings
    fn and(&mut self, other: &Self);

    /// Apply a bitwise OR between two bitstrings
    fn or(&mut self, other: &Self);

    /// Apply a bitwise XOR between two bitstrings
    fn xor(&mut self, other: &Self);

    /// Apply a bitwise NEG operation
    fn neg(&mut self);
}

/// Defines the shift operations
pub trait BitstringShift<A: Bitstring> {
    /// Apply a shift to the left
    fn shift_left(&mut self, with: usize);

    /// Apply a shift to the right
    fn shift_right(&mut self, with: usize);
}

/// Range operations
pub trait BitstringRange<A: Bitstring> {
    /// Reset all low bits
    fn rst_low(&mut self, len: usize);

    /// Reset all high bits
    fn rst_high(&mut self, len: usize);

    /// Set all the low bits
    fn set_low(&mut self, len: usize);

    /// Set all the high bits
    fn set_high(&mut self, len: usize);
}

/// Converts a bitstring into a vecotr of u8 values
pub trait BitstringInto<A: Bitstring> {
    /// List of u8 components,from low to high
    fn to_u8s(&self) -> Vec<u8>;

    /// List of bits from low to high
    fn to_bits(&self) -> Vec<Bit>;
}

/// Different combinators
pub trait BitstringCombinators<A: Bitstring> {
    /// Combine two bitstring
    fn combine(&mut self, other: &A);

    /// Flips a bit value at a given position
    fn flip(&mut self, pos: usize);

    /// Splt a bitstring in two at a given position
    fn split(&self, pos: usize) -> (A, A);
}
