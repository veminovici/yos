/// The values allowed into a bit
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bit {
    /// Zero
    Zero,
    /// One
    One,
}

/// The bitewise operations
pub trait Bitstring {
    /// Build a bitstring that represents zero
    fn bzero() -> Self;

    /// Build a bitstring that represents one
    fn bone() -> Self;

    /// Build a bistring with all low bits set to one
    fn bone_low(len: usize) -> Self;

    /// Build a bitstring with all high bits set to one
    fn bone_high(len: usize) -> Self
    where
        Self: Sized,
    {
        if len == 0 {
            Self::bzero()
        } else {
            let mut v = Self::bone_low(len);
            v.blshift(v.blen() - len);
            v
        }
    }

    /// The length of the bit string
    fn blen(&self) -> usize;

    /// AND bitwise operation between two bitstrings
    fn band(&mut self, other: &Self);

    /// OR bitwise operation between two bitstrings
    fn bor(&mut self, other: &Self);

    /// Neg bitwise operation on a bitstring
    fn bneg(&mut self);

    /// XOR bitwise operation between two bitstring
    fn bxor(&mut self, other: &Self);

    /// Shift the bit string to the left
    fn blshift(&mut self, len: usize);

    /// Builds a bitstring representing a power of 2
    fn bpow2(p: usize) -> Self;

    /// Reset the bit to a given index
    fn brst(&mut self, ndx: usize)
    where
        Self: Sized,
    {
        let mut m = Self::bpow2(ndx);
        m.bneg();
        self.band(&m);
    }

    /// Sets the bit to a given index
    fn bset(&mut self, ndx: usize)
    where
        Self: Sized,
    {
        self.brst(ndx);
        self.bor(&Self::bpow2(ndx));
    }

    /// Flips the value of the bit at a given index
    fn bflip(&mut self, ndx: usize)
    where
        Self: Sized,
    {
        self.bxor(&Self::bpow2(ndx));
    }

    /// Returns the value of a bit at a given index
    fn bget(&self, ndx: usize) -> Bit;

    /// Reset the lowest n bits
    fn brst_low(&mut self, len: usize)
    where
        Self: Sized,
    {
        for i in 0..len {
            self.brst(i);
        }
    }

    /// Reset the highest n bits
    fn brst_high(&mut self, len: usize)
    where
        Self: Sized,
    {
        let blen = self.blen();
        for i in (blen - len)..blen {
            self.brst(i);
        }
    }

    /// Split a bitstring into string at a cutting point
    fn bsplit(&self, cut: usize) -> (Self, Self)
    where
        Self: Sized;

    /// The list of u8 components from low to high
    fn bueights(&self) -> Vec<u8>;

    /// Combine two bitstring (by applying an OR operation)
    fn bcombine(&mut self, other: &Self) {
        self.bor(other);
    }
}

/// The debug representation of a bitwise structure.
pub trait BitstringDebug {
    /// Returns the debug representation
    fn bdebug(&self) -> String;
}

/// Defines the evolution functions.
pub trait Evolution<A: Bitstring + Sized> {
    /// Mutates a bit in a bit string at the given index
    fn mutate(bstr: &mut A, ndx: usize);

    /// Crossovers two bit strings at a given cutting point
    fn crossover(a: &mut A, b: &mut A, cut: usize);
}
