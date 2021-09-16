# ![rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white) Simplee...Yos... 

Just another rust crate, this one implements some bit-strings.

[![ci pipeline](https://github.com/veminovici/yos/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/veminovici/yos/actions/workflows/ci.yml)
[![Coverage Status](https://coveralls.io/repos/github/veminovici/yos/badge.svg)](https://coveralls.io/github/veminovici/yos)
[![Last commit](https://img.shields.io/github/last-commit/veminovici/yos)](https://github.com/veminovici/yos)
[![Repo size](https://img.shields.io/github/repo-size/veminovici/yos)](https://github.com/veminovici/yos)

[![Github Actions](https://buildstats.info/github/chart/veminovici/yos)](https://github.com/veminovici/yos)

</br>

### Bitstring Trait
The **Bitstring** trait defines the behavior that control a sequence of bits.

```rust
pub trait Bitstring {
    /// Build a bitstring that represents zero
    fn bzero() -> Self;

    /// Build a bitstring that represents one
    fn bone() -> Self;

    /// Build a bistring with all low bits set to one
    fn bone_low(len: usize) -> Self;

    /// Build a bitstring with all high bits set to one
    fn bone_high(len: usize) -> Self;

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
    fn brst(&mut self, ndx: usize);

    /// Sets the bit to a given index
    fn bset(&mut self, ndx: usize);

    /// Flips the value of the bit at a given index
    fn bflip(&mut self, ndx: usize);

    /// Returns the value of a bit at a given index
    fn bget(&self, ndx: usize) -> Bit;

    /// Reset the lowest n bits
    fn brst_low(&mut self, len: usize);

    /// Reset the highest n bits
    fn brst_high(&mut self, len: usize);

    /// Split a bitstring into string at a cutting point
    fn bsplit(&self, cut: usize) -> (Self, Self);

    /// The list of u8 components from low to high
    fn bueights(&self) -> Vec<u8>;

    /// Combine two bitstring (by applying an OR operation)
    fn bcombine(&mut self, other: &Self);
}
```

</br>

### Extending u8, u64
The crate extends the functionality of **u8**, and **u64** primitive data types by implementing the **Bitstring** trait on those data types.

```rust
use yos::*;

let mut v = 5u8;
v.brst(1);
assert_eq!(v, 4);
println!("v={}", v.debug());
```

</br>

### Thank you!!!

> You can contact me at veminovici@hotmail.com. Code designed and written in Päädu, on the beautiful island of [**Saaremaa**](https://goo.gl/maps/DmB9ewY2R3sPGFnTA), Estonia.
