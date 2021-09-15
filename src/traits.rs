/// The bitewise operations
pub trait Bitwise {
    /// Set the bit to a given index
    fn set(&mut self, ndx: usize);

    /// Reset the bit to a given index
    fn reset(&mut self, ndx: usize);

    /// Flip the value for a bit at a given index
    fn flip(&mut self, ndx: usize);

    /// Returns the value of a bit at a given index
    fn get(&self, ndx: usize) -> u8;

    /// Reset the low indexed bits
    fn reset_low(&mut self, n: usize);

    /// Reset the high indexed bits
    fn reset_high(&mut self, n: usize);

    /// Returns the list of u8 values.
    fn ueights(&self) -> Vec<u8>;

    /// Generate a bit string with the first-low len bits set to 1.
    fn low_mask(len: usize) -> Self
    where
        Self: Sized;

    /// Generate a bit string with the last-high len bits set to 1.
    fn high_mask(len: usize) -> Self
    where
        Self: Sized;

    /// Splits the bit string into a head and tail
    fn split(&self, cut: usize) -> (Self, Self)
    where
        Self: Sized;
}

/// The debug representation of a bitwise structure.
pub trait BitwiseDebug {
    /// Returns the debug representation
    fn debug(&self) -> String;
}

/// Defines the evolution functions.
pub trait Evolution {
    /// Mutates a bit in a bit string at the given index
    fn mutate(bstr: &mut dyn Bitwise, ndx: usize);
}
