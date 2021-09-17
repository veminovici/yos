use std::fmt::Display;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

/// The values allowed into a bit
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bit {
    /// Zero
    Zero,
    /// One
    One,
}

impl Default for Bit {
    fn default() -> Self {
        Bit::Zero
    }
}

impl BitOr for Bit {
    type Output = Bit;

    fn bitor(self, rhs: Self) -> Self::Output {
        let a: bool = self.into();
        let b: bool = rhs.into();
        Bit::from(a || b)
    }
}

impl BitOrAssign for Bit {
    fn bitor_assign(&mut self, rhs: Self) {
        let a: bool = (*self).into();
        let b: bool = rhs.into();
        *self = Bit::from(a || b);
    }
}

impl BitAnd for Bit {
    type Output = Bit;

    fn bitand(self, rhs: Self) -> Self::Output {
        let a: bool = self.into();
        let b: bool = rhs.into();
        Bit::from(a && b)
    }
}

impl BitAndAssign for Bit {
    fn bitand_assign(&mut self, rhs: Self) {
        let a: bool = (*self).into();
        let b: bool = rhs.into();
        *self = Bit::from(a && b);
    }
}

impl BitXor for Bit {
    type Output = Bit;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let a: u8 = self.into();
        let b: u8 = rhs.into();
        Bit::from(a ^ b)
    }
}

impl BitXorAssign for Bit {
    fn bitxor_assign(&mut self, rhs: Self) {
        let a: u8 = (*self).into();
        let b: u8 = rhs.into();
        *self = Bit::from(a ^ b);
    }
}

impl Not for Bit {
    type Output = Bit;

    fn not(self) -> Self::Output {
        let b: bool = self.into();
        Bit::from(!b)
    }
}

impl Display for Bit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if *self == Bit::Zero {
            write!(f, "B0")
        } else {
            write!(f, "B1")
        }
    }
}

impl From<bool> for Bit {
    fn from(b: bool) -> Self {
        if b {
            Bit::One
        } else {
            Bit::Zero
        }
    }
}

impl From<&bool> for Bit {
    fn from(b: &bool) -> Self {
        if *b {
            Bit::One
        } else {
            Bit::Zero
        }
    }
}

impl From<u8> for Bit {
    fn from(u: u8) -> Self {
        if u % 2 == 0 {
            Bit::Zero
        } else {
            Bit::One
        }
    }
}

impl From<&u8> for Bit {
    fn from(u: &u8) -> Self {
        Bit::from(*u)
    }
}

impl From<Bit> for bool {
    fn from(b: Bit) -> Self {
        match b {
            Bit::One => true,
            Bit::Zero => false,
        }
    }
}

impl From<&Bit> for bool {
    fn from(b: &Bit) -> Self {
        (*b).into()
    }
}

impl From<Bit> for u8 {
    fn from(b: Bit) -> Self {
        match b {
            Bit::Zero => 0,
            Bit::One => 1,
        }
    }
}

impl From<&Bit> for u8 {
    fn from(b: &Bit) -> Self {
        (*b).into()
    }
}

#[cfg(test)]
mod ptests {
    use super::*;

    use quickcheck::{Arbitrary, Gen};
    use quickcheck_macros::quickcheck;

    impl Arbitrary for Bit {
        fn arbitrary(g: &mut Gen) -> Self {
            let b = bool::arbitrary(g);
            Bit::from(b)
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            let mut shrunk_dots = Vec::new();
            if *self == Bit::One {
                shrunk_dots.push(Bit::Zero);
            }
            Box::new(shrunk_dots.into_iter())
        }
    }

    #[quickcheck]
    fn prop_display(bit: Bit) -> bool {
        let s = format!("{}", bit);
        !s.is_empty()
    }

    #[quickcheck]
    fn prop_debug(bit: Bit) -> bool {
        let s = format!("{:?}", bit);
        !s.is_empty()
    }

    #[quickcheck]
    fn prop_from_to_bool(b: bool) -> bool {
        let bit = Bit::from(b);
        let a = bit.into();
        b == a
    }

    #[quickcheck]
    fn prop_to_from_bool(bit: Bit) -> bool {
        let b: bool = bit.into();
        let a = Bit::from(b);
        bit == a
    }

    #[quickcheck]
    fn prop_from_to_ref_bool(b: bool) -> bool {
        let bit = Bit::from(&b);
        let a = (&bit).into();
        b == a
    }

    #[quickcheck]
    fn prop_from_to_u8(u: u8) -> bool {
        let bit = Bit::from(u);
        let a: u8 = bit.into();
        a == (u % 2)
    }

    #[quickcheck]
    fn prop_to_from_u8(bit: Bit) -> bool {
        let b: u8 = bit.into();
        let a = Bit::from(b);
        bit == a
    }

    #[quickcheck]
    fn prop_from_to_ref_u8(u: u8) -> bool {
        let bit = Bit::from(&u);
        let a: u8 = (&bit).into();
        a == (u % 2)
    }

    #[quickcheck]
    fn prop_or(a: Bit, b: Bit) -> bool {
        let c = a | b;
        if a == Bit::One || b == Bit::One {
            c == Bit::One
        } else {
            c == Bit::Zero
        }
    }

    #[quickcheck]
    fn prop_or_assign(a: Bit, b: Bit) -> bool {
        let mut z = a;
        z |= b;
        if a == Bit::One || b == Bit::One {
            z == Bit::One
        } else {
            z == Bit::Zero
        }
    }

    #[quickcheck]
    fn prop_and(a: Bit, b: Bit) -> bool {
        let c = a & b;
        if a == Bit::Zero || b == Bit::Zero {
            c == Bit::Zero
        } else {
            c == Bit::One
        }
    }

    #[quickcheck]
    fn prop_and_assign(a: Bit, b: Bit) -> bool {
        let mut c = a;
        c &= b;
        if a == Bit::Zero || b == Bit::Zero {
            c == Bit::Zero
        } else {
            c == Bit::One
        }
    }

    #[quickcheck]
    fn prop_xor(a: Bit, b: Bit) -> bool {
        let c = a ^ b;
        match (a, b) {
            (Bit::Zero, Bit::Zero) => Bit::Zero == c,
            (Bit::Zero, Bit::One) => Bit::One == c,
            (Bit::One, Bit::Zero) => Bit::One == c,
            (Bit::One, Bit::One) => Bit::Zero == c,
        }
    }

    #[quickcheck]
    fn prop_xor_assign(a: Bit, b: Bit) -> bool {
        let mut c = a;
        c ^= b;
        match (a, b) {
            (Bit::Zero, Bit::Zero) => Bit::Zero == c,
            (Bit::Zero, Bit::One) => Bit::One == c,
            (Bit::One, Bit::Zero) => Bit::One == c,
            (Bit::One, Bit::One) => Bit::Zero == c,
        }
    }

    #[quickcheck]
    fn prop_not(a: Bit) -> bool {
        let b = !a;
        a & b == Bit::Zero
    }
}
