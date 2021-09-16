use crate::Bitstring;

/// Defines the evolution functions.
pub trait Evolution<A: Bitstring> {
    /// Mutates a bit in a bit string at the given index
    fn mutate(bstr: &mut A, ndx: usize);

    /// Crossovers two bit strings at a given cutting point
    fn crossover(a: &mut A, b: &mut A, cut: usize);
}
