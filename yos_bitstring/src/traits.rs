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
