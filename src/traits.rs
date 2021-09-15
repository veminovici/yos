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
}
